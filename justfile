default:
    cargo

run level="info" +ARGS:
    RUST_LOG={{level}} cargo run -- --config-path ./test-config/ {{ARGS}}

test level="info":
    RUST_LOG={{level}} cargo test -- --nocapture

test-watch:
    cargo-watch -x test
