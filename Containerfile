FROM rust:1.94 as builder
WORKDIR /build
COPY . .
RUN cargo build --release


FROM debian:trixie-slim as carmen-extractor
WORKDIR /app
COPY --from=builder /build/target/release/carmen-extractor .
ENTRYPOINT ["/app/carmen-extractor"]


FROM debian:trixie-slim as carmen-indexer
WORKDIR /app
COPY --from=builder /build/target/release/carmen-indexer .
ENTRYPOINT ["/app/carmen-indexer"]


FROM debian:trixie-slim as carmen-migrations
WORKDIR /app
COPY --from=builder /build/target/release/carmen-migrate .
ENTRYPOINT ["/app/carmen-migrate"]


FROM debian:trixie-slim as carmen-search
WORKDIR /app
COPY --from=builder /build/target/release/carmen-search .
ENTRYPOINT ["/app/carmen-search"]
