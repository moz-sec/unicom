FROM rust:1.78.0-bullseye AS builder
ARG TARGETPLATFORM
WORKDIR /work/unicom
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
ARG VERSION=0.1.0
LABEL org.opencontainers.image.source=https://github.com/moz-sec/unicom \
        org.opencontainers.image.version=${VERSION} \
        org.opencontainers.image.title=unicom \
        org.opencontainers.image.description="unicom is Universal Compressor."
RUN adduser --disabled-password --disabled-login --home /workdir nonroot \
    && mkdir -p /workdir
COPY --from=builder /work/unicom/target/release/unicom /opt/unicom/unicom

WORKDIR /workdir
USER nonroot

ENTRYPOINT [ "/opt/unicom/unicom" ]
