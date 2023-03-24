FROM rust:1.67-bullseye AS chef
WORKDIR /chef
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
WORKDIR /build
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin

FROM debian:11.6-slim as prod
WORKDIR /prod
COPY --from=builder /build/target/release/yak-api /prod
EXPOSE 8080
CMD ["/prod/yak-api"]