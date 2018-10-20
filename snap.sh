#!/bin/bash
set -eux

cargo run -p lalrpop -- --force --out-dir lalrpop/src/parser lalrpop/src/parser/lrgrammar.lalrpop
