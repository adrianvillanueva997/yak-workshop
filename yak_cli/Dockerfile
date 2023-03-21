FROM rust:1.67-bullseye as builder
WORKDIR /build
RUN cargo build --release

FROM debian:11.6-slim as prod
WORKDIR /prod
COPY --from=builder /build/target/release/yak-cli /prod
EXPOSE 8080
CMD ["/prod/yak-cli"]