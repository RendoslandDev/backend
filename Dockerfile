FROM rust:1.83.0-slim as builder

WORKDIR /usr/src/backend

COPY Cargo.toml Cargo.lock ./

COPY . .
RUN cargo build --release

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/backend

EXPOSE 8080
CMD ["backend"]
