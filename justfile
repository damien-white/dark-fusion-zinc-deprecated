set dotenv-load := false

# Check to verify that the project compiles
@compile:
    cargo fmt --all -- --check
    cargo test

# Run all common CI tasks locally
@check-all:
    cargo fmt --all -- --check && \
    cargo clippy -- --D warnings && \
    cargo test && \
    cargo audit
