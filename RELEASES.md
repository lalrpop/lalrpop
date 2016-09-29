# master (not yet released)

# Version 0.12.1

Bug fix release. Major bugs addressed:

- ["LR1 TLS not installed"](https://github.com/nikomatsakis/lalrpop/issues/145);
- [False conflicts in the LR(1) code generator](https://github.com/nikomatsakis/lalrpop/issues/144).

Also, there is now a tutorial for writing custom lexers.
[Thanks @malleusinferni!](https://github.com/nikomatsakis/lalrpop/pull/131).

# Version 0.12 

**Enabled a new table-driven code-generator by default.** This generates
less code than the older recursive-ascent-based generation scheme, but
may parse less efficiently. To go back to the old scheme, annotate
the grammar declaration:

```
#[recursive_ascent] grammar;
```

Also, **the syntax for requesting LALR-generation has changed** to use
an annotation:

```
#[LALR] grammar;
```

We no longer **emit module-level attributes**, which means that unused
imports in your .lalrpop file may start getting warnings. [Thanks
@dflemstr!](https://github.com/nikomatsakis/lalrpop/pull/118)

An overflow bug in LALRPOP was
fixed. [Thanks @larsluthman!](https://github.com/nikomatsakis/lalrpop/pull/120)

We no longer depend on `time`, but now use
`std::time`. [Thanks @serprex!](https://github.com/nikomatsakis/lalrpop/pull/121)

There is now a `Configuration` object for use in your `build.rs`
scripts.  And,
[thanks to @dflemstr!](https://github.com/nikomatsakis/lalrpop/pull/108),
it permits you to configure the directory where LALRPOP output is
generated.

Fixed a bug in the LALRPOP option
parsing. [Thanks @Nemikolh!](https://github.com/nikomatsakis/lalrpop/pull/106)

Various typos and small corrections. Thanks [@reuben](https://github.com/nikomatsakis/lalrpop/pull/1103) and [@ashleygwilliams](https://github.com/nikomatsakis/lalrpop/pull/116)!

# Version 0.11

Updated to use the `regex-syntax` crate for regular expression
parsing instead of rolling our own parser. This means we can now
support the [same regular expression syntax as the regex crate](https://doc.rust-lang.org/regex/regex/index.html#syntax),
and in particular can support unicode character classes like `\p{Greek}`.
Note that some regex features -- such as non-greedy repetition and
named capture groups -- are still not supported (or just not meaningful).

Optimized LR(1) construction time by approximately 5x.

Improved handling of location tokens `@L` and `@R` so that they can be
freely used without ever causing parse conflicts.

# Version 0.10

Major update to LALRPOP error messages in cases of shift/reduce and
reduce/reduce conflicts. The messages now try to explain the problem
in terms of your grammar, as well as diagnosing common problem
scenarios and suggesting solutions.

Added a standalone LALRPOP executable.

We no longer generate incomplete files when grammar generation fails
(Issue #57).

# Version 0.9

Miscellaneous bug fixes, mostly. Processing for a `build.rs` file now
starts from the project directory, rather than being hardcoded to
start from `src`.

# Version 0.8

Add support for inlining nonterminals. Nonterminals can now be
annotated with `#[inline]`. If you do so, each use of the nonterminal
will be inlined into its place. This can be very helpful in addressing
shift-reduce or reduce-reduce conflicts, at the cost of a larger
grammar. We now inline `Foo*`, `Foo?`, and `(Foo Bar)` nonterminals by
default.

# Version 0.7

This is mostly a bug-fix release.

Various minor issues were addressed:

- Issue #25: Unbalanced parens in string literals appearing in code now work properly.
- Issue #32: Regular expression parsing consumed infinite memory when a `.` appeared.
- Issue #34: Automatic tokenizer generation did not play well with generic type parameters.

# Older versions

I hadn't yet started writing release notes, sorry.
