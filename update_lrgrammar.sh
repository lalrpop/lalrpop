#!/bin/sh

cp -f lalrpop-internals/src/parser/lrgrammar.rs lalrpop-internals/src/parser/lrgrammar.rs.bak
cargo run -p lalrpop -- --force --no-whitespace --out-dir . lalrpop-internals/src/parser/lrgrammar.lalrpop || exit 1
