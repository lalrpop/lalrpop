#!/bin/sh

cp -f lalrpop/src/parser/lrgrammar.rs lalrpop/src/parser/lrgrammar.rs.bak
cargo run -p lalrpop -- --force --no-whitespace --out-dir lalrpop/src/parser lalrpop/src/parser/lrgrammar.lalrpop || exit 1
