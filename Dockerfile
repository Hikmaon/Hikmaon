FROM rust:1.79 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/hikmalayer /usr/local/bin/hikmalayer
EXPOSE 3000
ENV RUST_LOG=info
CMD ["hikmalayer"]
