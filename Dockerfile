FROM rust:1.62 as builder

RUN USER=root cargo new --bin rust-luau-server
WORKDIR /rust-luau-server

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/rust_luau_server*
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /rust-luau-server/target/release/rust-luau-server .

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

CMD ["./rust-luau-server"]
