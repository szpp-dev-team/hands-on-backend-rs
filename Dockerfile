FROM rust:1.67-slim-buster AS builder

WORKDIR /work
RUN cargo new --bin hands-on-backend-rs

WORKDIR /work/hands-on-backend-rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --debug \
    && rm src/*.rs \
    && rm -f target/release/deps/hands-on-backend-rs*

COPY src ./src
RUN cargo build --debug

FROM ubuntu:22.04

WORKDIR /work

RUN apt-get update && apt-get install --no-install-recommends -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /work/hands-on-backend-rs/target/release/hands-on-backend-rs .
ENTRYPOINT [ "/work/hands-on-backend-rs" ]
