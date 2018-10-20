#!/bin/bash
set -eux

cargo run -p lalrpop -- --force --no-whitespace --out-dir lalrpop/src/parser lalrpop/src/parser/lrgrammar.lalrpop
