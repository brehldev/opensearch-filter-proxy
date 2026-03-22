FROM lukemathwalker/cargo-chef:latest-rust-1@sha256:d6148f9afb1bb54d5371fa0cecd08d01393475e606b2407c8679f906189a49c8 AS chef
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
FROM debian:bookworm-slim@sha256:f06537653ac770703bc45b4b113475bd402f451e85223f0f2837acbf89ab020a AS runtime

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
