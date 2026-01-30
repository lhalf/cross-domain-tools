set shell := ["bash", "-euc"]

check:
    cargo fmt --check --all
    cargo clippy --bins --all-features -- -Dwarnings

test: unit-test integration-test

unit-test:
    cargo test --locked --workspace --exclude tests

integration-test: build
    cargo test --locked --lib integration --workspace --exclude common -- --test-threads 1

build:
    cargo build --release --workspace --exclude common --exclude tests