FROM rust:1.79.0-alpine

RUN apk update
RUN apk add --no-cache curl
RUN apk add --no-cache musl-dev
RUN apk add --no-cache just
RUN apk add --no-cache tar #to make actions/cache work

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN wasm-pack --version

RUN rustup component add rustfmt
RUN rustup component add clippy