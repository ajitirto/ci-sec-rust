# Stage 1: Build
FROM rust:1.75-slim-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
# Install library SSL yang dibutuhkan Axum/Serde jika ada request HTTPS nanti
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/rust-json-api . 

# Security: Jalankan sebagai user biasa, bukan root
RUN useradd -m appuser
USER appuser

# Axum default port
EXPOSE 3000

CMD ["./rust-json-api"]
