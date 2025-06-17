# syntax=docker/dockerfile:1.4

FROM rust:1.85 AS builder

WORKDIR /app
COPY . .

RUN apt-get update && \
    apt-get install -y openssl libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

RUN cargo build --release

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl3 openssl && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/

CMD ["backend"]
