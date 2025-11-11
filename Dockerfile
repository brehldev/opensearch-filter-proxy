FROM lukemathwalker/cargo-chef:latest-rust-1@sha256:a01e128c11b3dbc1b896d2c0b85b463de2d0be43c01e26efe76c7ceec4312211 AS chef
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
FROM debian:bookworm-slim@sha256:936abff852736f951dab72d91a1b6337cf04217b2a77a5eaadc7c0f2f1ec1758 AS runtime

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
