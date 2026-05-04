FROM rust:1.95-trixie AS builder

LABEL maintainer="Federal Office of Information Technology, Systems and Telecommunication <jeap@bit.admin.ch>"

################################### Certs ###################################

COPY --from=bit-base-images-docker-hosted.nexus.bit.admin.ch/bit/ca-bundle:latest /certs/ /usr/local/share/ca-certificates/
RUN update-ca-certificates --verbose

################################### End Certs ###################################

WORKDIR /app

# Build dependencies required by this project.
RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    libprotobuf-dev \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

FROM debian:trixie AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

################################### Certs ###################################

COPY --from=builder /usr/local/share/ca-certificates/ /usr/local/share/ca-certificates/
RUN update-ca-certificates --verbose

################################### End Certs ###################################

WORKDIR /app

COPY --from=builder /app/target/release/project-rag /usr/local/bin/project-rag

ENTRYPOINT ["/usr/local/bin/project-rag"]
