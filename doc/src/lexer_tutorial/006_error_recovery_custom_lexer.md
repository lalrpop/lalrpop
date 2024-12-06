# Error recovery with a custom lexer

Features described in
[Error recovery](https://lalrpop.github.io/lalrpop/tutorial/008_error_recovery.html)
work well with the custom lexer, however error recovery only supports
recovering from parser errors - not lexer errors.  This page shows an approach
to recover from lexer errors.

Enable error recovery:

```diff
use lalrpop_util::ErrorRecovery;

- grammar<'err>(errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>);
+ grammar<'err>(errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>);
```

Now error recovery will work for parser errors, however if your lexer
encounters an unexpected token, that's a *lexer* error, not a *parser* error
so the parser is unable to recover from it.

```plaintext
print (a - $);

// Err(User { error: InvalidToken })
```

It's caused by the inconsistency in the lexer stream, as lalrpop parser
always expects a normal token stream. To fix this, you need to add a
workaround in the lexer part to process the "error recovery" inside
the lexer, for example

```diff
impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
        .next()
-       .map(|(token, span)| Ok((span.start, token?, span.end)))
+       .map(|(token, span)|
+           match token {
+               Ok(token) => Ok((span.start, token, span.end)),
+               Err(_) => Ok((span.start, Token::Error, span.end)),
+               // or specify your lexical error to parse error
+           }
+       )
    }
}
```

and then put a new member into `Token` enum to represent the error token
(or your custom error type).

```rust
pub enum Token {
    //...

    // Dont forget the comma
+   Error
}
```

Like NaN is a number in JavaScript, we have now an `Token::Error` which is
a token in our lexer.  This token can be passed to the parser, but if it is
not used in the parser grammar, will not parse.  That transforms the lexer
error into a recoverable parser error allowing you to have error recovery in
this case.

Also, you might need to custom your own error, visit the section on fallible
actions for details.

```lalrpop
! => {
    let error = ErrorRecovery {
        error: ParseError::User {
            error: LexicalError::SomeCustomError,
        },
        dropped_tokens: Vec::new(), // or specify the dropped tokens
    };
    errors.push(error);

    Expression::Error
}
```
