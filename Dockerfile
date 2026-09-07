FROM rust:1-bookworm AS builder

LABEL maintainer="Federal Office of Information Technology, Systems and Telecommunication <jeap@bit.admin.ch>"

RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    # protobuf-compiler alone only provides the protoc binary. lance-encoding's build.rs imports
    # the well-known types (google/protobuf/empty.proto etc.), which ship in libprotobuf-dev, not
    # protobuf-compiler - without it protoc fails with "File not found" on that import.
    libprotobuf-dev \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    tar \
    gzip \
    curl && \
    rm -rf /var/lib/apt/lists/*

# fastembed/ort is built with the "ort-load-dynamic" feature (see Cargo.toml) instead of the
# default "download-binaries": that default statically links a prebuilt ONNX Runtime binary built
# with a newer libstdc++ than any base image here provides (GCC 11+'s std::string::_M_replace_cold)
# - a link-time ABI mismatch no base-image swap or static-libstdc++ flag actually fixes, since the
# binary itself is fixed. load-dynamic instead loads libonnxruntime.so at runtime via dlopen
# (ORT_DYLIB_PATH, set on the runtime stage below), so nothing links against it at build time at
# all. Fetched here from Microsoft's own release (not pyke/ort's CDN repackaging, which uses a
# custom, non-standard archive format) - same 1.28.0 version ort/fastembed's "api-24" expects.
RUN curl -fsSL -o /tmp/onnxruntime.tgz \
      https://github.com/microsoft/onnxruntime/releases/download/v1.28.0/onnxruntime-linux-x64-1.28.0.tgz && \
    tar -xzf /tmp/onnxruntime.tgz -C /tmp && \
    cp -P /tmp/onnxruntime-linux-x64-1.28.0/lib/libonnxruntime.so* /usr/local/lib/ && \
    rm -rf /tmp/onnxruntime.tgz /tmp/onnxruntime-linux-x64-1.28.0

WORKDIR /projectrag
COPY . .

RUN cargo build --release

FROM amazoncorretto:25-al2023 AS runtime

RUN dnf install -y shadow-utils openssl-libs ca-certificates && \
    groupadd -r raguser && \
    useradd -r -g raguser -m -d /home/raguser raguser && \
    mkdir -p /projectrag && chown -R raguser:raguser /projectrag && \
    dnf clean all

COPY --from=builder /projectrag/target/release/project-rag /usr/local/bin/project-rag
COPY --from=builder /usr/local/lib/libonnxruntime.so* /usr/local/lib/
ENV ORT_DYLIB_PATH=/usr/local/lib/libonnxruntime.so

USER raguser

# Pre-download embedding model to ensure it's available at runtime without requiring network access.
ENV PROJECT_RAG_MODEL_PATH=/home/raguser/models/all-MiniLM-L6-v2
RUN mkdir -p "$PROJECT_RAG_MODEL_PATH" \
     && cd "$PROJECT_RAG_MODEL_PATH" \
     && BASE=https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main \
     && for f in model.onnx tokenizer.json config.json special_tokens_map.json tokenizer_config.json; do \
            curl -fL -o "$f" "$BASE/$f"; \
        done

ENTRYPOINT ["/usr/local/bin/project-rag"]
