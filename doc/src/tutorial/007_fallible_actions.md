# Returning errors from actions

Sometimes it can be useful to have action code that is able to return an error
instead of being expected to produce a value of type `T` directly. This happens
because we usually cannot reject all invalid input just by using grammar rules,
or rather, the work to do so would be too much.

Even in our calculator example, you can see that we're "cheating" the system:
Our grammar accepts an unlimited number of digits in the input, but the result
is parsed as a `i32`. This is an issue because the maximum number that `i32`
can represent is 2147483647. Try giving it a bigger number and it will panic
because it always expects the `i32` conversion to succeed.

If you are familiar with Rust's error handling story, you might think that we
can just make `Num` return an `Option<i32>` or even `Result<i32, ...>`, and you
would be right. However, that is not necessary, because if we look at the type
of `ExprParser::parse()`, we can see that it already returns a `Result<i32,
ParseError>`. So the goal is to "hook" into this existing error machinery and
create action code that can return errors.

LALRPOP supports this very easily by defining action code with `=>?` instead of
`=>`. The returned value is then assumed to be a `Result<T, ParseError>`
instead of a plain `T`:

```lalrpop
Num: i32 = {
    r"[0-9]+" =>? i32::from_str(<>)
        .map_err(|_| ParseError::User {
            error: "number is too big"
        })
};
```

In addition, we have to add `use lalrpop_util::ParseError;` to the top of the
file so that we have access to the `ParseError` type. You can find the full
source as [`calculator6.lalrpop`][calculator6]. This allows you to nicely
handle the errors:

```rust
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator6);

#[test]
fn calculator6() {
    // Number is one bigger than std::i32::MAX
    let expr = calculator6::ExprsParser::new().parse("2147483648");
    assert!(expr.is_err());
}
```

No panics!

You can even go a step further and define your own error type, for example an
enum with all possible errors. This allows you to distinguish between different
errors more easily, without relying on strings.

For that, let's say we want to define two errors: One if the input number was
too big, and another one if the input number was not even - we're changing the
calculator to only accept even numbers for now.

We first define our error enum in `main.rs`:

```rust
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Calculator6Error {
    InputTooBig,
    OddNumber,
}
```

Then we import it into our grammar and tell LALRPOP to use it as the user error
type, so we change the top of the file to:

```lalrpop
use std::str::FromStr;
use crate::ast::{Expr, Opcode};

use super::Calculator6Error;

use lalrpop_util::ParseError;

grammar;

extern {
    type Error = Calculator6Error;
}
...
```

We can also change the rule for `Num` to make use of our new error:

```lalrpop
Num: i32 = {
    r"[0-9]+" =>? i32::from_str(<>)
        .map_err(|_| ParseError::User {
            error: Calculator6Error::InputTooBig
        })
        .and_then(|i| if i % 2 == 0 {
            Ok(i)
        } else {
            Err(ParseError::User {
                error: Calculator6Error::OddNumber
            })
        })
};
```

And finally we can see if it works:

```rust
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator6b);

#[test]
fn calculator6b() {
    use lalrpop_util::ParseError;

    let expr = calculator6b::ExprsParser::new().parse("2147483648");
    assert!(expr.is_err());
    assert_eq!(expr.unwrap_err(), ParseError::User { error: Calculator6Error::InputTooBig });

    let expr = calculator6b::ExprsParser::new().parse("3");
    assert!(expr.is_err());
    assert_eq!(expr.unwrap_err(), ParseError::User { error: Calculator6Error::OddNumber });
}
```

There we go! You can find the full grammar in [`calculator6b.lalrpop`][calculator6b].

[calculator6]: https://github.com/lalrpop/lalrpop/blob/master/doc/calculator/src/calculator6.lalrpop
[calculator6b]: https://github.com/lalrpop/lalrpop/blob/master/doc/calculator/src/calculator6b.lalrpop
