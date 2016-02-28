# Version 0.12 (not yet released)

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
