# Cheatsheet

Users of Lalrpop has compiled the following cheatsheet table as a quick way to
look up useful Lalrpop-isms. If you are looking for a specific piece of
functionality, use this table to jump to the right section.

| name | snipped | description | tutorial |
|---|---|---|---|
| position | `<@L> T <@R>` | uhm, I think it gives the index of the parser in the input | - |
| error_recovery | `! => { ... }` | recovers from parser errors | [Error recovery](tutorial/008_error_recovery.md) |
| grammar_parameter | `grammar(scale: isize);` | input parameters usable in the generated parser | [Passing state parameter](tutorial/009_state_parameter.md) |
| ?? | `Num => func(<i32>)` | maybe automatic number parsing?? | - |
| custom_error | `"e" =>? Err(ParseError::User { error: "an error" })` | makes an action fallible | [Fallible actions](tutorial/007_fallible_actions.md) |
| custom_macros | `Comma<T> =  { ... }` | allows a non-terminal to be generic about some parameters | [Macros](tutorial/006_macros.md) |
| quantifier_macros | `<Num?> <Num*> <Num+>` |  a non-terminal which can appear 0..1, 0+, 1+ times | [Macros](tutorial/006_macros.md) |
| tuple_macro | `<a:(<Num> ",")*>` | applies a quantifier to a group of matches | [Macros](tutorial/006_macros.md) |
| extern | `extern { }` | allows to override some parts of the generated parser | [Writing a custom lexer](lexer_tutorial/003_writing_custom_lexer.md) |
| extern_error | `type Error = MyError;` | sets the error to use in the `ParseError::User` variant | [Writing a custom lexer](lexer_tutorial/003_writing_custom_lexer.md) |
| extern_location | `type Location = MyLoc;` | sets the type to for locations instead of `usize` | [Writing a custom lexer](lexer_tutorial/003_writing_custom_lexer.md) |
| extern_tok | `enum Tok { }` | set the type to use for tokens?? | [Using tokens with references](lexer_tutorial/004_token_references.md) |
| auto_parameters | `<>` | in an action it turns into a list of parameters | [Type inference](tutorial/003_type_inference.md) |
|conditional actions | `<T> if T == "a" => (),` | - | - |
|precedence| `#[precedence(level="0")]` | - | [Handling full expressions](tutorial/004_full_expressions.md) |
