export RUST_BACKTRACE=1
export CARGO_INCREMENTAL=0

cargo build -p lalrpop
cargo test --all --all-features
# Check the documentation examples separately so that the `lexer` feature specified in tests do not
# leak into them
cargo check -p calculator
cargo check -p pascal
cargo check -p whitespace
