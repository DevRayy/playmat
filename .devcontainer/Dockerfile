FROM rust:1.78.0-alpine

RUN apk update
RUN apk add --no-cache musl-dev
RUN apk add --no-cache make protobuf-dev
RUN apk add --no-cache just
RUN apk add --no-cache tar #to make actions/cache work

RUN rustup component add rustfmt
RUN rustup component add clippy