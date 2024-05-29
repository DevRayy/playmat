lint:
    cargo fmt --check
    cargo clippy -- -D warnings

tidy:
    cargo fmt
    cargo clippy --allow-dirty --fix -- -D warnings

run app:
    cargo run -p {{app}}

deps:
    cargo fetch

build app:
    cargo build -p {{app}}

test app:
    cargo test -p {{app}}

start container:
    docker compose up -d {{container}}