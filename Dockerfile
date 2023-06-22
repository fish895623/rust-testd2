FROM rust:1-alpine AS builder
WORKDIR /workspace
COPY . .
RUN apk add zstd-libs build-base
RUN cargo install --path . \
    && rm -rf /workspace

CMD ["rust-testd"]
