FROM rust:1.87-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    build-essential \
    cmake \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY *.rs /app/src
COPY Cargo.toml /app/Cargo.toml

RUN cargo build --release

FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/kafka-producer-api /usr/local/bin/kafka-producer-api

CMD ["kafka-producer-api"]