FROM rust:1.94 as builder
WORKDIR /build
COPY . .
RUN cargo build --release


FROM debian:trixie-slim as carmen-api
WORKDIR /app
COPY --from=builder /build/target/release/carmen-api .
ENTRYPOINT ["/app/carmen-api"]


FROM debian:trixie-slim as carmen-extractor
WORKDIR /app

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates pandoc \
    && rm -rf /var/cache/* /var/lib/apt/lists/* /tmp/*

COPY --from=builder /build/target/release/carmen-extractor .

ENTRYPOINT ["/app/carmen-extractor"]


FROM debian:trixie-slim as carmen-indexer
WORKDIR /app

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/cache/* /var/lib/apt/lists/* /tmp/*

COPY --from=builder /build/target/release/carmen-indexer .

ENTRYPOINT ["/app/carmen-indexer"]


FROM debian:trixie-slim as carmen-migrations
WORKDIR /app
COPY --from=builder /build/target/release/carmen-migrate .
ENTRYPOINT ["/app/carmen-migrate"]
