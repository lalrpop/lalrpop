# Version 0.10 (not yet released)

Major update to LALRPOP error messages in cases of shift/reduce and
reduce/reduce conflicts. The messages now try to explain the problem
in terms of your grammar, as well as diagnosing common problem
scenarios and suggesting solutions.

Added a standalone LALRPOP executable.

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
