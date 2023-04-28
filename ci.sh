# Exis on error
set -ex

export RUST_BACKTRACE=1
export CARGO_INCREMENTAL=0
cargo build --bin lalrpop --features pico-args
cargo hack test --workspace --each-feature
# Check the documentation examples separately so that the `lexer` feature specified in tests do not
# leak into them
cargo check -p calculator
cargo check -p pascal
cargo check -p whitespace
