#!/bin/bash

cargo run -p lalrpop -- --force --no-whitespace --out-dir . lalrpop/src/parser/lrgrammar.lalrpop
