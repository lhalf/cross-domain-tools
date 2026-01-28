set shell := ["bash", "-euc"]

check:
    cargo fmt --check --all
    cargo clippy --bins --all-features -- -Dwarnings