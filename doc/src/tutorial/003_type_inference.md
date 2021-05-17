# Type inference

OK, now that we understand [the calculator1 example][calculator1], let's
look at some of the shorthands that LALRPOP offers to make it more concise.
This code is found in [the calculator2 demo][calculator2].

To start, let's look at the definition of `Term` we saw before:

```lalrpop
pub Term: i32 = {
    <n:Num> => n,
    "(" <t:Term> ")" => t,
};
```

The action code here is somewhat interesting. In both cases, it's not
doing any new work, it's just selecting a value that was produced by
another nonterminal. This turns out to be pretty common. So common,
in fact, that LALRPOP offers some shorthand notation for it. Here is
the definition of `Term` from the calculator2 demo:

```lalrpop
pub Term = { Num, "(" <Term> ")" };
```

Here, we have no action code at all. If there is no action code,
LALRPOP synthesizes action code which just takes the value of the
things being matched. In the case of the first alternative, `Num`,
there is only one thing being matched, so that means that `Term` will
produce the same value as the `Num` we parsed, whatever that was.

In the case of the second alternative, `"(" <Term> ")"`, there are
three things being matched. Here we use the angle brackets to select
which item(s) we want to take the value of --- we selected only one,
so the result is that we take the value of the `Term` we parsed. If we
selected more than one, the result would be a tuple of all the
selected items.  If we did not select any (i.e., `"(" Term ")"`), the
result would be a tuple of all the items, and hence the result would
be of type `(&'input str, i32, &'input str)`.

Speaking of types, you may have noticed that `Term` has no type
annotation. Since we didn't write out own action code, we can omit the
type annotation and let LALRPOP infer it for us. In this case, LALRPOP
can see that `Term` must have the same type as `Num`, and hence that
the type must be `i32`.

OK, let's look at the definition of `Num` we saw before from calculator1:

```lalrpop
Num: i32 = <s:r"[0-9]+"> => i32::from_str(s).unwrap();
```

This definition too can be made somewhat shorter. In calculator2, you will
find:

```lalrpop
Num: i32 = r"[0-9]+" => i32::from_str(<>).unwrap();
```

Here, instead of giving the regular expression a name `s`, we modified
the action code to use the funky expression `<>`. This is a shorthand
that says "synthesize names for the matched values and insert a
comma-separated list here". In this case, there is only one matched
value, `r"[0-9]+"`, and it produces a `&'input str`, so LALRPOP will
insert a synthetic variable for that value. Note that we still have
custom action code, so we still need a type annotation.

To control what values are selected when you use the `<>` expression
in your action code, you can use angle brackets as we saw before.
Here are some examples of alternatives and how they are expanded to
give you the idea:

| Alternative          | Equivalent to              |
| -----------          | -------------              |
| `A => bar(<>)`       | `<a:A> => bar(a)`          |
| `A B => bar(<>)`     | `<a:A> <b:B> => bar(a, b)` |
| `A B => (<>)`        | `<a:A> <b:B> => (a, b)`    |
| `<A> B => bar(<>)`   | `<a:A> B => bar(a)`        |
| `<p:A> B => bar(<>)` | `<p:A> B => bar(p)`        |
| `<A> <B> => bar(<>)` | `<a:A> <b:B> => bar(a, b)` |
| `<p:A> <q:B> => bar(<>)` | `<p:A> <q:B> => bar(p, q)` |
| `<p:A> B => Foo {<>}` | `<p:A> B => Foo {p:p}` |
| `<p:A> <q:B> => Foo {<>}` | `<p:A> <q:B> => Foo {p:p, q:q}` |

The `<>` expressions also works with struct constructors (like `Foo
{...}` in examples above). This works out well if the names of your
parsed values match the names of your struct fields.

[calculator1]: https://github.com/lalrpop/lalrpop/blob/master/doc/calculator/src/calculator1.lalrpop
[calculator2]: https://github.com/lalrpop/lalrpop/blob/master/doc/calculator/src/calculator2.lalrpop
