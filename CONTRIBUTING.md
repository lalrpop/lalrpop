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

#### Semver policy

All releases should follow rust's semantic versioning guidelines, as described
at https://doc.rust-lang.org/cargo/reference/semver.html. Breaking changes in
our context include both api changes in the published library and binary builds
as well as changes that break previously working lalrpop grammars.

If a PR introduces a breaking change, the breaking-ness should not delay the
merge of the PR.  Instead, master will become breaking and the next release
from master will be a breaking change release.  However, to support continued
non-breaking releases and reduce the frequency of breaking releases, we will
create a branch off of the last commit before the breaking change and backport
future non-breaking changes to it.  For example, if the latest release was 0.22
and a breaking change is merged, we will create a branch named "0.22.x" from
the commit prior to the breaking change.  We will backport to that branch as
non-breaking changes are merged, and possible release 0.22.1, 0.22.2 etc from
it.

Not all changes will be backported.  For example, many code cleanups may not
justify a backport.  Changes that do not backport cleanly will only be
backported at maintainers descretion.  All backporting is best-effort and
subject to maintainer availability to perform a backport.

Except in rare and severe circumstances, the 0.22.x branch will only be
maintained until the release of version 0.23.  A critical bug may warrant a
later backport to the 0.22.x series, but in general, we will stop backporting
and focus efforts on the 0.23 series as soon as 0.23 is released.

Regarding Minumum Supported Rust Version (msrv) bumps, the guidance in the rust
community is that msrv bumps are not necessarily breaking, but should be bundled
with breaking changes if possible. Our msrv strategy follows this guidance,
attempting to bundle msrv bumps with breaking releases.  We prefer to keep the
msrv at least two versions behind stable (eg if the latest rust release is 1.83,
we prefer our msrv to be no more than 1.81).  In the event that there is a
prolonged time between breaking releases, it may be desirable to bump the msrv
in a non-breaking release.  In this case, we prefer to keep at least eight
releases behind stable (eg if the latest rust release is 1.83, we prefer to
bump the msrv in a non-breaking release to at most 1.75).

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
