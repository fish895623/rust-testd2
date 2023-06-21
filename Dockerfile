FROM rust:1-slim
WORKDIR /workspace
COPY . .
RUN cargo install --path .
CMD ["rust-testd"]
