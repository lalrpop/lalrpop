# Location Tracking

Sometimes it may be helpful to know where in the input stream a particular
token was encountered. For example, this can be useful when generating
user-visible messages that reference specific points in the input.

This is achieved via the location tracking macros, `@L` and `@R`.  `@L`
binds the start position - in bytes - of the token immediately to its right
in the input stream. Similarly, `@R` binds the index of the last byte _plus
one_ of the token immediately to its left.  Both symbols bind as type `usize`

Here's an example rule using location tracking macros:

```lalrpop
Symbol = {
    <start: @L> <s: r"a-z"> <end: @R> => {
        // `start` is the byte location of the start of our string
        // `s` is the string itself
        // `end` is the byte location of the end
    }
}
```

You can also see another example in [this test](https://github.com/lalrpop/lalrpop/blob/master/lalrpop-test/src/intern_tok.lalrpop),
where location tracking is wrapped in a macro.

Since `@R2` binds to the index of the byte immediately to the right of the
token, this makes the bound values suitable to immediately construct a [Range](https://doc.rust-lang.org/std/ops/struct.Range.html)
which is bounded inclusively above and exclusively below:

```lalrpop
Symbol: Range<usize> = {
    <start: @L> <s: r"a-z"> <end: @R> => {
        start..end
    }
}
```
