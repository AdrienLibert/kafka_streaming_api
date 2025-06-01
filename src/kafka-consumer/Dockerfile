FROM rust:1.87-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    build-essential \
    cmake \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . /app

RUN cargo build --release

FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/kafka-consumer /usr/local/bin/kafka-consumer

CMD ["kafka-consumer"]