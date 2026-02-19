set shell := ["bash", "-euc"]

fmt:
    cargo fmt --all

check:
    cargo fmt --check --all
    cargo clippy --bins --all-features -- -Dwarnings

check-strict:
    cargo clippy --all-targets --all-features -- -D clippy::pedantic -D clippy::nursery

test: unit-test integration-test

unit-test:
    cargo test --locked --workspace --exclude import-http-tests

integration-test: build
    cargo test --locked --lib integration --workspace --exclude common --exclude import-http-common -- --test-threads 1

build:
    cargo build --release --workspace --exclude common --exclude import-http-common --exclude import-http-tests