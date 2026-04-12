FROM lukemathwalker/cargo-chef:latest-rust-1@sha256:d5a1cca12f21de999e5b221b5c6ff5635080f4b8d5e58053241ff169e0fc60f6 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin opensearch-filter-proxy

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim@sha256:4724b8cc51e33e398f0e2e15e18d5ec2851ff0c2280647e1310bc1642182655d AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/opensearch-filter-proxy /usr/local/bin

ARG USER_ID=1001
ARG GROUP_ID=1001
RUN groupadd -g ${GROUP_ID} nonrootgroup && useradd -r -u ${USER_ID} -g nonrootgroup nonrootuser

USER nonrootuser

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/opensearch-filter-proxy"]
