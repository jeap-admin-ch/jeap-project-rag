FROM repo.bit.admin.ch:8444/amazonlinux:2023 AS builder

LABEL maintainer="Federal Office of Information Technology, Systems and Telecommunication <jeap@bit.admin.ch>"

################################### Certs ###################################

COPY --from=bit-base-images-docker-hosted.nexus.bit.admin.ch/bit/ca-bundle:latest /certs/ /etc/pki/ca-trust/source/anchors/
RUN update-ca-trust

################################### End Certs ###################################

WORKDIR /app

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
    curl \
    tar \
    gzip \
    && dnf clean all

# Install Rust via rustup  \
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --default-toolchain 1.95.0 --profile minimal --no-modify-path

COPY . .

RUN cargo build --release

FROM repo.bit.admin.ch:8444/amazoncorretto:25-al2023 AS runtime

RUN dnf install -y --setopt=install_weak_deps=False \
    openssl-libs \
    ca-certificates \
    && dnf clean all

################################### Certs ###################################

COPY --from=builder /etc/pki/ca-trust/source/anchors/ /etc/pki/ca-trust/source/anchors/
RUN update-ca-trust

################################### End Certs ###################################

WORKDIR /app

COPY --from=builder /app/target/release/project-rag /usr/local/bin/project-rag

ENTRYPOINT ["/usr/local/bin/project-rag"]
