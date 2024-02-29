FROM rust:1.71 AS builder

WORKDIR /rust/src/app
COPY . .
RUN cargo generate-lockfile
RUN cargo build --release

FROM debian:bullseye

COPY --from=builder /rust/src/app/target/release/tiny /

CMD ["./tiny"]

EXPOSE 80
