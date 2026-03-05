# OpenFang Agent OS — Railway Deployment
# Standard openfang binary with DeepSeek R1 as primary brain

FROM rust:1.83-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates/ crates/
COPY xtask/ xtask/
COPY agents/ agents/
COPY packages/ packages/
COPY prompts/ prompts/

RUN cargo build --release -p openfang-cli 2>&1 | tail -20

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/openfang /app/openfang
COPY agents/ /app/agents/
COPY prompts/ /app/prompts/
COPY config.toml /app/config.toml

RUN mkdir -p /app/data

ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

CMD ["/app/openfang", "--config", "/app/config.toml", "start"]
