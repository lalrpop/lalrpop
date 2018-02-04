#!/bin/bash
#
# This script runs on travis to prep the CI as needed.

if command -v mdbook >/dev/null 2>&1; then
    echo "mdbook already installed at $(command -v mdbook)"
else
    echo "installing mdbook"
    cargo install mdbook --vers "0.0.28"
fi

