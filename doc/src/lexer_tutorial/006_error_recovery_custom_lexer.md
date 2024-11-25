# Error recovery with custom lexer

Features described in
[Error recovery](https://lalrpop.github.io/lalrpop/tutorial/008_error_recovery.html)
works well with the custom lexer, like the lexer built by yourself or Logos. Let's
also take Logos as an example. However, you need to do a little extra work to make
it work. In `grammar.lalrpop`, change the signature of `grammar`

```diff
use lalrpop_util::ErrorRecovery;

- grammar<'err>(errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>);
+ grammar<'err>(errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>);
```

Now, everything should work as you wish. But still, there's a little problem here
that you may encounter. If you try tweaking the toy script in previous chapter,
with an illegal token to trigger `LexicalError::InvalidToken`, the parser will
panic at the lexical error

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
(or your custom error type)

```rust
pub enum Token {
    //...

    // Dont forget the comma
+   Error
}
```

You see, like NaN is a number in JavaScript, we have now an `Token::Error` is
a token in our lexer. That's something you can do with a custom lexer. And with
the builtin lexer, you would probably need to exhaust the possible errors in the
lexer which is a little bit more complicated and ugly.

Also, you might need to custom your own error, turn to fallible actions for details.

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
