### TLDR;

Before doing anything else, run `cargo build -p lalrpop`!

From now on use `cargo build/test -p lalrpop --features=test` instead of
`cargo build -p lalrpop`.

Just before your final commit, run `sh update_lrgrammar.sh`


### Contributor's "How to"

Just as any language that is written in itself, LALRPOP has to solve chicken-and-egg problem.
To compile LALRPOP we need a working version of LALRPOP. To have a working version we need
to compile it first.

In the past, LALRPOP used snapshots of its older self to compile itself (you might notice
[lalrpop-snap](https://crates.io/crates/lalrpop-snap) on crates.io). Unfortunately, this meant end-users
had to build **both** `lalrpop` and `lalrpop-snap` crates and nobody liked this double-building.
This approach has been abandoned a while ago.

Today's approach is different - `lalrpop` crate includes already auto-generated parser
in `lalrpop/src/parser/lrgrammar.rs` that's end-users use.
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

Now you need to tell cargo that you're using `lalrpop/src/parser/lrgrammar.lalrpop` to generate
a new `lrgrammar.rs` inside `{CARGO_OUT_DIR}` and use it for the compilation:

```sh
$ cargo build/test -p lalrpop --features=test
# or
$ cargo build/test -p lalrpop --all-features
```

When this flag is passed, cargo uses not `lalrpop/src/parser/lrgrammar.rs` but the newly generated
`lrgrammar.rs` instead (generating it if needed). From now on you use this command.

Once you're done with your work, all the tests are passed, and you are ready to finally commit
you changes run

```sh
$ sh update_lrgrammar.sh
```

This script will replace `lalrpop/src/parser/lrgrammar.rs` with the newly generated one, renaming
the old one to `lalrpop/src/parser/lrgrammar.rs.bak` just in case.

Now feel free to `git commit` and `git push` your changes.
