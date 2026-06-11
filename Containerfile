FROM rust:1.92 as builder
WORKDIR /build
COPY . .
RUN cargo build --release


FROM debian:trixie-slim as carmen-indexer
WORKDIR /app
COPY --from=builder /build/target/release/carmen-indexer .
ENTRYPOINT ["/app/carmen-indexer"]
