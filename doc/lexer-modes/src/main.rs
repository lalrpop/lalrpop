use std::rc::Rc;

use lexer_modes::lexer::Lexer;
use lexer_modes::ListParser;
use lexer_modes::Value;

fn main() {
    // Values are encoded by a length, a colon, and then the value.
    // The value doesn't need to be followed by whitespace, but it
    // may.
    let input = &b"3:foo5:xyzzy"[..];

    let lexer = Lexer::new(input);
    let mode = Rc::clone(&lexer.mode);

    let values = ListParser::new().parse(&mode, lexer).unwrap();

    assert_eq!(
        &values[..],
        &[
            Value {
                value: b"foo".to_vec()
            },
            Value {
                value: b"xyzzy".to_vec()
            },
        ][..]
    );

    // Whitespace can also occur in the value.
    let input = &b"3:f   5:    y"[..];

    let lexer = Lexer::new(input);
    let mode = Rc::clone(&lexer.mode);

    let values = ListParser::new().parse(&mode, lexer).unwrap();

    assert_eq!(
        &values[..],
        &[
            Value {
                value: b"f  ".to_vec()
            },
            Value {
                value: b"    y".to_vec()
            },
        ][..]
    );
}
