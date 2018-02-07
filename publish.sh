#!/bin/bash
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
TMPDIR=${TMPDIR:-"/tmp"}

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

git tag $VERSION
git push origin tag $VERSION

printf "Updated version in docs etc..."
perl -p -i -e 's/^version = "[0-9.]+"$/version = "'$VERSION'"/' \
     doc/*/Cargo.toml \
     >& $TMPDIR/publish-log || publish_fail
perl -p -i -e 's/^lalrpop = "[0-9.]+"$/lalrpop = "'$VERSION'"/' \
     doc/src/*.md \
     doc/src/tutorial/*.md \
     doc/*/Cargo.toml \
     >& $TMPDIR/publish-log || publish_fail
perl -p -i -e 's/^lalrpop-util = "[0-9.]+"$/lalrpop-util = "'$VERSION'"/' \
     doc/src/*.md \
     doc/src/tutorial/*.md \
     doc/*/Cargo.toml \
     >& $TMPDIR/publish-log || publish_fail
git add -f \
    doc/src/*.md \
    doc/src/tutorial/*.md \
    doc/*/Cargo.toml \
    >& $TMPDIR/publish-log || publish_fail
printf "OK\n"

printf "\nAll set. **Do not forget to commit new changes.**\n"
