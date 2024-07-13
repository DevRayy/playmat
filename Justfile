lint:
    cargo fmt --check
    cargo clippy -- -D warnings

tidy:
    cargo fmt
    cargo clippy --allow-dirty --fix -- -D warnings

run-desktop:
    cargo run

build-desktop:
    cargo build

run-web: build-web
    python3 -m http.server 8080

build-web:
    wasm-pack build --target web

test:
    cargo test