lint:
    cargo fmt --check
    cargo clippy -- -D warnings

tidy:
    cargo fmt
    cargo clippy --allow-dirty --fix -- -D warnings

run app:
    cargo run -p {{app}} --release