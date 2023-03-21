FROM rust:1.67-bullseye as builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:11.6-slim as prod
WORKDIR /prod
COPY --from=builder /build/target/release/yak-api /prod
EXPOSE 8080
CMD ["/prod/yak-api"]