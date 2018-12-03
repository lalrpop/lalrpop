# Crash course on parsers

If you've never worked with a parser generator before, or aren't
really familiar with context-free grammars, this section is just a
*very brief* introduction into the basic idea. Basically a grammar is
a nice way of writing out what kinds of inputs are legal.  In our
example, we want to support parenthesized numbers, so things like
`123`, `(123)`, etc. We can express this with a simple grammar like:

```
Term = Num | "(" Term ")"
```

Here we say we are trying to parse a *term*, and a term can either be
a number (`Num`) or some other term enclosing in parentheses (here I
did not define what a number is, but in the real LALRPOP example we'll
do that with a regular expression).  Now imagine a potential input
like `((123))`. We can show how this would be parsed by writing out
something called a "parse tree":

```
(  (  1  2  3  )  )
|  |  |     |  |  |
|  |  +-Num-+  |  |
|  |     |     |  |
|  |   Term    |  |
|  |     |     |  |
|  +---Term----+  |
|        |        |
+------Term-------+
```

Here you can see that we parsed `((123))` by finding a `Num` in the
middle, calling that `Num` a `Term`, and matching up the parentheses
to form two more terms on top of that.

Note that this parse tree is not a data structure but more a
visualization of the parse. I mean, you *can* build up a parse tree as
a data structure, but typically you don't want to: it is more detailed
than you need. For example, you may not be that interested in the
no-op conversion from a `Num` to a `Term`. The other weird thing about
a parse tree is that it is intimately tied to your grammar, but often
you have some existing data structures you would like to parse into --
so if you built up a parse tree, you'd then have to convert from the
parse tree into those data structures, and that might be annoying.

Therefore, what a parser generator usually does, instead is lets you
choose how to represent each node in the parse tree, and how to do the
conversions. You give each nonterminal a type, which can be any Rust
type, and you write code that will execute each time a new node in the
parse tree would have been constructed. In fact, in the examples that follow, we'll
eventually build up something like a parse tree, but in the beginning, we won't
do that at all. Instead, we'll represent each number and term as an `i32`,
and we'll propagate this value around.

To make this a bit more concrete, here's a version of the grammar above
written in LALRPOP notation (we'll revisit this again in more detail of course).
You can see that the `Term` nonterminal has been given the type `i32`,
and that each of the definitions has some code that follows a `=>` symbol.
This is the code that will execute to convert from the thing that was matched
(like a number, or a parenthesized term) into an `i32`:

```lalrpop
Term: i32 = {
    Num => /* ... number code ... */,
    "(" Term ")" => /* ... parenthesized code ... */,
};
```

OK, that's enough background, let's do this for real!
