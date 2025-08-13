# Guards in macro

Sometimes, you may want two very similar set of declarations, but with a few changes. A typical example is when implementing a Rust-like language with optional trailing ";" in some control-flow expressions within blocks. In that case, you want to allow omitting the trailing ";" only after a block, because in that case the grammar is unambiguous.

This can be done naively by duplicating the rules, but this is not very DRY. Instead, we can use *LALRPOP's **guards in macros** feature to achieve this:

```lalrpop
pub Expr = ExprRestricted<"B">;
ExprNoBlock = ExprRestricted<"">;

// - B: if non-empty, include block expressions
ExprRestricted<B>: i32 = {
    <l:ExprRestricted<B>> "+" <r:Factor<"B">> => l + r,
    <l:ExprRestricted<B>> "-" <r:Factor<"B">> => l - r,
    Factor<B>,
};

Factor<B>: i32 = {
    <l:Factor<B>> "*" <r:Unary<"B">> => l * r,
    <l:Factor<B>> "/" <r:Unary<"B">> => l / r,
    Unary<B>,
};

Unary<B>: i32 = {
    Term<B>,
    "-" <e:Term<"B">> => -e,
}

Term<B>: i32 = {
    Num,
    "(" <e:Expr> ")" => e,
    "print" "(" <e:Expr> ")" => { println!("{}", e); 0 },
    Block if B != "",
};

Block: i32 = {
    "{" <s:Stmt*> <e:ExprNoBlock?> "}" => e.unwrap_or_else(|| s.last().copied().unwrap_or(0)),
}

Stmt: i32 = {
    <e:ExprNoBlock> ";" => e,
    <b:Block> ";"? => b,
};

Num: i32 = {
    r"[0-9]+" => i32::from_str(<>).unwrap(),
};
```

Every declaration that depends on the `B` parameter will be generated twice: once with `B` set to `""` and once with `B` set to `"B"`. The first one will be used for the `ExprNoBlock` rule, and the second one for the `Expr` rule.
A `Stmt` can then be either an expression that is not a block, and thus that requires the trailing `";"` or a block that may or may not have a trailing `";"`.

You can read the complete example in [`calculator10.lalrpop`](calculator 10):

```rust
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator10); // synthesized by LALRPOP

#[test]
fn calculator10() {
    test10("{ 1 }", 1);
    test10("{ 1; 2 }", 2);
    test10("{ print(1 + 1); 2 }", 2);
    test10("{ { 1 } }", 1);
    test10("{ { 1 } 2 }", 2);
}
```

[calculator10]: https://github.com/lalrpop/lalrpop/blob/master/doc/calculator/src/calculator10.lalrpop
