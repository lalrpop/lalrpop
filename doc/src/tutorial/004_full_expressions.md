# Handling full expressions

Now we are ready to extend our calculator to cover the full range of
arithmetic expressions (well, at least the ones you learned in
elementary school). Here is
[the next calculator example, calculator3][calculator3]:

```lalrpop
use std::str::FromStr;

grammar;

pub Expr: i32 = {
    <l:Expr> "+" <r:Factor> => l + r,
    <l:Expr> "-" <r:Factor> => l - r,
    Factor,
};

Factor: i32 = {
    <l:Factor> "*" <r:Term> => l * r,
    <l:Factor> "/" <r:Term> => l / r,
    Term,
};

Term: i32 = {
    Num,
    "(" <Expr> ")",
};

Num: i32 = {
    r"[0-9]+" => i32::from_str(<>).unwrap(),
};
```

Perhaps the most interesting thing about this example is the way it
encodes precedence. The idea of precedence of course is that in an
expression like `2+3*4`, we want to do the multiplication first, and
then the addition. LALRPOP doesn't have any built-in features for
giving precedence to operators, mostly because I consider those to be
creepy, but it's pretty straightforward to express precedence in your
grammar by structuring it in tiers -- for example, here we have the
nonterminal `Expr`, which covers all expressions. It consists of a series
of factors that are added or subtracted from one another. A `Factor`
is then a series of terms that are multiplied or divided. Finally, a
`Term` is either a single number or, using parenthesis, an entire expr.

Abstracting from this example, the typical pattern for encoding
precedence is to have one nonterminal per precedence level, where you
begin with the operators of lowest precedence (`+`, `-`), add in the
next highest precedence level (`*`, `/`), and finish with the bare
"atomic" expressions like `Num`. Finally, you add in a parenthesized
version of your top-level as an atomic expression, which lets people
reset.

To see why this works, consider the two possible parse trees for
something like `2+3*4`:

```
2 + 3   *    4          2   +  3   *    4
| | |   |    |          |   |  |   |    |
| | +-Factor-+    OR    +-Expr-+   |    |
| |     |                   |      |    |
+-Expr -+                   +----Factor-+
```

In the first one, we give multiplication higher precedence, and in the
second one, we (incorrectly) give addition higher precedence. If you
look at the grammar now, you can see that the second one is
impossible: a `Factor` cannot have an `Expr` as its left-hand side.
This is the purpose of the tiers: to force the parser into the
precedence you want.

Finally, note that we only write `pub` before the nonterminal we're 
interested in parsing (`Expr`) and not any of the helpers. Nonterminals
marked `pub` have extra code generated, like the `new()` method used to
access the parser from other modules. If you get a warning about an 
unused `new()` method on `FooParser`, drop the `pub` from nonterminal
`Foo`.

[calculator3]: https://github.com/lalrpop/lalrpop/blob/master/doc/calculator/src/calculator3.lalrpop
