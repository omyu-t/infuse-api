FROM rust:1.82-slim-bookworm AS builder
WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# 必要なパッケージをインストール
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN adduser default && chown -R default /app

USER default
COPY --from=builder ./app/target/release/app ./target/release/app

ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT ["./target/release/app"]