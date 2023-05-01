# Build stage
FROM rust:1.67-bullseye AS builder
WORKDIR /app
COPY . .
RUN cargo install cargo-chef
RUN cargo chef prepare --recipe-path recipe.json
RUN --mount=type=cache,target=/app/target cargo chef cook --release --recipe-path recipe.json
RUN --mount=type=cache,target=/app/target cargo build --release

# Final stage
FROM debian:buster-slim AS prod
WORKDIR /app
COPY --from=builder /app/target/release/yak_api .
EXPOSE 8080
CMD ["./yak_api"]
