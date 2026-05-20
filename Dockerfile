FROM repo.bit.admin.ch:8444/amazonlinux:2023 AS builder

LABEL maintainer="Federal Office of Information Technology, Systems and Telecommunication <jeap@bit.admin.ch>"

################################### Certs ###################################

COPY --from=bit-base-images-docker-hosted.nexus.bit.admin.ch/bit/ca-bundle:latest /certs/ /etc/pki/ca-trust/source/anchors/
RUN update-ca-trust

################################### End Certs ###################################

# Build dependencies required by this project.
RUN dnf install -y --setopt=install_weak_deps=False \
    gcc \
    gcc-c++ \
    make \
    protobuf-compiler \
    protobuf-devel \
    pkgconf-pkg-config \
    openssl-devel \
    ca-certificates \
    git \
    tar \
    gzip \
    shadow-utils && \
    groupadd -r raguser && \
    useradd -r -g raguser -m -d /home/raguser raguser && \
    mkdir -p /projectrag && chown -R raguser:raguser /projectrag && \
    dnf clean all

WORKDIR /projectrag

USER raguser

# Install Rust via rustup  \
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.95.0 --profile minimal
ENV PATH=/home/raguser/.cargo/bin:$PATH

COPY . .

RUN source /home/raguser/.cargo/env && \
    cargo build --release

FROM repo.bit.admin.ch:8444/amazoncorretto:25-al2023 AS runtime

COPY --from=bit-base-images-docker-hosted.nexus.bit.admin.ch/bit/ca-bundle:latest /certs/ /etc/pki/ca-trust/source/anchors/
RUN update-ca-trust && \
    dnf install -y shadow-utils openssl-libs ca-certificates && \
    groupadd -r raguser && \
    useradd -r -g raguser -m -d /home/raguser raguser && \
    mkdir -p /projectrag && chown -R raguser:raguser /projectrag && \
    dnf clean all

COPY --from=builder /projectrag/target/release/project-rag /usr/local/bin/project-rag

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
