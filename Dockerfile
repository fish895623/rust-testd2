FROM rust
WORKDIR /workspace
COPY . .
RUN cargo install --path .
CMD ["rust-testd"]
