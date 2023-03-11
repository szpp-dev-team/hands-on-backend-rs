FROM rust:1.67-slim-buster AS builder

RUN apt-get update && apt-get install --no-install-recommends -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /work
RUN cargo new --bin hands-on-backend-rs

WORKDIR /work/hands-on-backend-rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build \
    && rm src/*.rs \
    && rm -f target/debug/deps/hands_on_backend_rs*

COPY src ./src
RUN cargo build

FROM ubuntu:22.04

RUN apt-get update && apt-get install --no-install-recommends -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /work
COPY --from=builder /work/hands-on-backend-rs/target/debug/hands-on-backend-rs .

ENTRYPOINT [ "/work/hands-on-backend-rs" ]
