#!/bin/bash
#
# This script runs on travis to prep the CI as needed.

if command -v mdbook >/dev/null 2>&1; then
    echo "mdbook already installed at $(command -v mdbook)"
else
    echo "installing mdbook"
    cargo install mdbook --vers "0.4.7"
fi

if command -v mdbook-linkcheck >/dev/null 2>&1; then
    echo "mdbook-linkcheck already installed at $(command -v mdbook-linkcheck)"
else
    echo "installing mdbook-linkcheck"
    cargo install mdbook-linkcheck --vers "0.7.6"
fi
