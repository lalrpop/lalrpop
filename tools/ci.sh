#!/bin/bash
set -e -u -o pipefail

export RUST_BACKTRACE=1
export CARGO_INCREMENTAL=0
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

cargo build --bin lalrpop --features pico-args
# build with minimal amount of features to make sure dependencies can build too.
cargo hack build --workspace --feature-powerset --exclude-features pico-args --optional-deps
# test with minimal amount of features plus a few extra on regex/regex-syntax
cargo hack test --workspace --feature-powerset --exclude-features pico-args --optional-deps
# Check the documentation examples separately so that the `lexer` feature specified in tests do not
# leak into them
cargo check -p calculator
cargo check -p pascal
cargo check -p whitespace
