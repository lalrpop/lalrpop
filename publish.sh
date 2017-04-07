#!/bin/sh
#
# A script to bump the version number on all Cargo.toml files etc in
# an atomic fashion.

VERSION=$(
    ls lalrpop*/Cargo.toml | \
        xargs grep "# LALRPOP$" | \
        perl -p -e 's/.*version = "([0-9.]+)" # LALRPOP$/$1/' |
        sort |
        uniq)

if [ $(echo $VERSION | wc -w) != 1 ]; then
    echo "Error: inconsistent versions detected across Cargo.toml files!"
    echo "$VERSION"
    exit 1
fi

echo "Found consistent version $VERSION"

BASE_DIR="$PWD"

function publish_fail {
    printf "ERROR\n"
    cat $TMPDIR/publish-log
    exit 1
}

function publish {
    printf "Publishing %s..." "$1"
    cd $1
    cargo publish >& $TMPDIR/publish-log || publish_fail $1
    cd $BASE_DIR
    printf "OK\n"
}

publish lalrpop-intern
publish lalrpop-util
publish lalrpop-snap
publish lalrpop

printf "Updated version in README and tutorial..."
perl -p -i -e 's/^version = "[0-9.]+"$/version = "'$VERSION'"/' \
     README.md doc/tutorial.md doc/*/Cargo.toml >& $TMPDIR/publish-log || publish_fail
perl -p -i -e 's/^lalrpop = "[0-9.]+"$/lalrpop = "'$VERSION'"/' \
     doc/tutorial.md doc/*/Cargo.toml >& $TMPDIR/publish-log || publish_fail
perl -p -i -e 's/^lalrpop-util = "[0-9.]+"$/lalrpop-util = "'$VERSION'"/' \
     doc/tutorial.md doc/*/Cargo.toml >& $TMPDIR/publish-log || publish_fail
git add -f README.md doc/tutorial.md doc/*/Cargo.toml >& $TMPDIR/publish-log || publish_fail
printf "OK\n"

printf "\nAll set. Do not forget to commit new README.md et al.\n"
