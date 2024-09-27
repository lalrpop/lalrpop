#!/usr/bin/env bash
#
# A script to bump the version number on all Cargo.toml files etc in
# an atomic fashion.

set -e -o pipefail

if [ "$1" == "" ]; then
    echo "Usage: version.sh <new-version-number>"
    exit 1
fi

VERSION=$(
    ls Cargo.toml lalrpop*/Cargo.toml | \
        xargs grep "# LALRPOP" | \
        perl -p -e 's/.*version = "([0-9.]+)" # LALRPOP/$1/' |
        sort |
        uniq)

if [ $(echo $VERSION | wc -w) != 1 ]; then
    echo "Error: inconsistent versions detected across Cargo.toml files!"
    echo "$VERSION"
    exit 1
fi

echo "Found consistent version $VERSION"

clog --setversion $1

perl -p -i -e 's/lalrpop(.*)= *"'$VERSION'"/lalrpop\1= "'$1'"/' \
     $(ls Cargo.toml lalrpop*/Cargo.toml) \
     doc/src/quick_start_guide.md doc/src/tutorial/001_adding_lalrpop.md

perl -p -i -e 's/version *= *"'$VERSION'" *# LALRPOP/version = "'$1'" # LALRPOP"/' \
     $(ls Cargo.toml lalrpop*/Cargo.toml) \
     doc/src/quick_start_guide.md doc/src/tutorial/001_adding_lalrpop.md

# Leave in special handling for the docs, because this case is a little weird
# in that we're finding everything with the previous version number and
# updating it, which may in theory include things that aren't lalrpop.  We
# don't really want to use the "# LALRPOP" taggging solution used elsewhere
# because this is examples for people to read and copy, so we just accept that
# bug here, but don't spread it to the rest of the code base
perl -p -i -e 's/version *= *"'$VERSION'"/version = "'$1'"/' \
     $(find doc -name Cargo.toml)

./update_lrgrammar.sh
