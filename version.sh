#!/bin/sh
#
# A script to bump the version number on all Cargo.toml files etc in
# an atomic fashion.

if [ "$1" == "" ]; then
    echo "Usage: version.sh <new-version-number>"
    exit 1
fi

perl -p -i -e 's/version *= *"[0-9.]+" # LALRPOP$/version = "'$1'" # LALRPOP/' \
     $(ls lalrpop*/Cargo.toml)

perl -p -i -e 's/^version = "[0-9.]+"$/version = "'$1'"/' \
     README.md
