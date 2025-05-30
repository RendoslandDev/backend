FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .


# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/my-backend /usr/local/bin/backend

ENV PORT=8080
EXPOSE $PORT

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release
COPY . .


CMD["backend"]


