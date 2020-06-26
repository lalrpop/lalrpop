# Passing state parameter

By default, the parser doesn't take any argument other than the input.
When building the AST, it might be useful to pass parameters to the parser, which might be needed to the construction of the tree.

Going back to the calculator4 example it is possible to pass an argument to the parser :


```rust
grammar(scale: i32);
```

```rust
Num: i32 = {
    r"[0-9]+" => i32::from_str(<>).unwrap()*scale,
};
```

Here the parser will accept a scale parameter that will scale every number encountered.

We can then call the parser with the state parameter :

```rust
#[test]
fn calculator8() {
    let scale = 2;
    let expr = calculator8::ExprParser::new()
        .parse(scale,"11 * 22 + 33")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}
```

For a more practical example with a custom tree structure, check out [this parser](https://github.com/lalrpop/lalrpop/blob/master/lalrpop-test/src/expr_arena.lalrpop) using [this structure](https://github.com/lalrpop/lalrpop/blob/master/lalrpop-test/src/expr_arena_ast.rs) to build the AST.


