default:
    cargo

run +ARGS:
    RUST_LOG=info cargo run -- --config-path ./test-config/ {{ARGS}}

test:
    cargo test -- --nocapture

test-watch:
    cargo-watch -x test
