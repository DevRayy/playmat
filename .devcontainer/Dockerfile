FROM rust:1.78.0-buster

RUN apt-get update
RUN apt-get install -y protobuf-compiler

RUN rustup component add rustfmt
RUN rustup component add clippy

RUN cargo install just