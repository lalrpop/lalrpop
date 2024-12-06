### TLDR;

Before doing anything else, run `cargo build -p lalrpop`!

When making changes the alter the generated code, use `sh update_lrgrammar.sh`
to pass the `verify_lalrpop_generates_itself` test.


### Contributor's "How to"

Just as any language that is written in itself, LALRPOP has to solve chicken-and-egg problem.
To compile LALRPOP we need a working version of LALRPOP. To have a working version we need
to compile it first.

The `lalrpop` crate already includes an auto-generated parser
in `lalrpop/src/parser/lrgrammar.rs` that end-users use.
Small (meh), (relatively) fast to compile, easy to use (really, for end-users).

That said, if you changes don't affect LALRPOP's own grammar
(`lalrpop/src/parser/lrgrammar.lalrpop`) your workflow is simple

```sh
# building
$ cargo build # -p lalrpop --release

# testing
$ cargo test # -p lalrpop --release
```

But if your changes *do* affect the grammar - well, that's where all the fun of
bootstrapping compilers comes! You're going to have to get a working `lalrpop` binary to
generate your own `lrgrammar.rs` parser. That's how you do it:

```sh
$ cargo build -p lalrpop # --release
```

Once you're done with your work, make sure that you are running against an
updated version of the grammar.

```sh
$ sh update_lrgrammar.sh
```

This script will replace `lalrpop/src/parser/lrgrammar.rs` with the newly generated one, renaming
the old one to `lalrpop/src/parser/lrgrammar.rs.bak` just in case.

Now feel free to `git commit` and `git push` your changes.

#### History

In the past, LALRPOP used snapshots of its older self to compile itself (you might notice
[lalrpop-snap](https://crates.io/crates/lalrpop-snap) on crates.io). Unfortunately, this meant end-users
had to build **both** `lalrpop` and `lalrpop-snap` crates and nobody liked this double-building.
So this approach has been abandoned a while ago.

### Releasing LALRPOP

#### Prerequisites
1. `version.sh` makes use of clog-cli.  Run `cargo install clog-cli` to get it

#### Without Cargo-Release

1. Run `./version.sh <NEW VERSION>`.
2. Commit the changes
3. Run `cargo publish` for lalrpop and lalrpop-util
4. Tag new release
5. Push new tag to repository

#### Cargo-Release

If using [cargo-release](https://github.com/crate-ci/cargo-release):

1. Run `./version.sh <NEW VERSION>`.
2. Commit the changes
3. `cargo release --execute --package lalrpop-util --tag-name "{{version}}"`
4. `cargo release --execute --package lalrpop --no-tag`

Drop the `--execute` flag to do a dry run and check for any issues first.

### Running code coverage tools

There are a few gotchas when running code coverage tooling on the lalrpop code
base.  Below are instructions to get it working using [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov).

The first issue is that we need to build an instrumented lalrpop binary to use
during test runs. Following the instructions [here](https://github.com/taiki-e/cargo-llvm-cov?tab=readme-ov-file#get-coverage-of-external-tests)
will enable an instrumented binary to be built and then found by cargo during
the test run.

Second, a lot of our work happens in build scripts, which llvm-cov ignores by
default.  To have it consider build scripts, pass the --include-build-script
argument in your call to to `llvm-cov report` like so:

```shell
cargo llvm-cov --include-build-script report --html
```

That will make sure that build script commands (particularly
lalrpop-test/build.rs) are part of the calculated coverage, which gives a
much more accurate picture.
