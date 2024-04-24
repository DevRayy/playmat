lint:
    cargo fmt --check
    cargo clippy -- -D warnings

tidy:
    cargo fmt
    cargo clippy --fix -- -D warnings

run app:
    cargo run -p {{app}}