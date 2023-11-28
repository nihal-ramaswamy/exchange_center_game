FROM rust:slim as builder
RUN USER=root

WORKDIR .
ADD . ./
RUN cargo build --release
EXPOSE 8000

CMD ["./target/release/exchange_center_server"]
