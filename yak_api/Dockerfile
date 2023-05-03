# Build stage
FROM rust:1.67-bullseye AS builder
WORKDIR /build
COPY . .
RUN cargo install cargo-chef
RUN cargo chef prepare --recipe-path recipe.json
RUN --mount=type=cache,target=/build/target cargo chef cook --release --recipe-path recipe.json
RUN --mount=type=cache,target=/build/target cargo build --release

# Final stage
FROM debian:buster-slim AS prod
WORKDIR /app
COPY --from=builder /build/target/release/yak_api .
EXPOSE 8080
CMD ["./yak_api"]
