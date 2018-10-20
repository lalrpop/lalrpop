#!/bin/bash

cargo build -p lalrpop --bins
cargo build -p lalrpop --bins --feature snapshot
