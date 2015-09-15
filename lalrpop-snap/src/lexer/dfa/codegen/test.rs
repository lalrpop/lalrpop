use diff;
use lexer::dfa::Precedence;
use lexer::dfa::test::dfa;
use lexer::dfa::codegen;
use rust::RustWrite;

const P1: Precedence = Precedence(1);
const P0: Precedence = Precedence(0);

#[test] #[ignore] // <-- state machine not 100% deterministic, I fear
fn codegen() {
    let dfa = dfa(&[
        /* 0 */ (r#"abc"#, P1),
        /* 1 */ (r#"[a-c]+"#, P0),
        /* 2 */ (r#" "#, P0),
        ]).unwrap();

    let mut buffer = vec![];
    codegen::compile_tokenize_fn("_", &dfa, &mut RustWrite::new(&mut buffer)).unwrap();
    let actual = String::from_utf8(buffer).unwrap();
    println!("{}", actual);

    let expected = r#"fn _tokenize<'input,_CHARS>(
    mut _chars: _CHARS,
) -> Option<(usize, usize)>
where _CHARS: Iterator<Item=(usize, char)>
{
    let mut _current_match: Option<(usize, usize)> = None;
    let mut _current_state: usize = 0;
    loop {
        match _current_state {
            0 => {
                let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
                match _ch {
                    ' ' => {
                        _current_match = Some((2, _index + 1));
                        _current_state = 3;
                        continue;
                    }
                    'a' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 2;
                        continue;
                    }
                    'b' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    'c' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    _ => {
                        return _current_match;
                    }
                }
            }
            1 => {
                let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
                match _ch {
                    'a' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    'b' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    'c' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    _ => {
                        return _current_match;
                    }
                }
            }
            2 => {
                let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
                match _ch {
                    'a' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    'b' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 5;
                        continue;
                    }
                    'c' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    _ => {
                        return _current_match;
                    }
                }
            }
            3 => {
                let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
                match _ch {
                    _ => {
                        return _current_match;
                    }
                }
            }
            4 => {
                let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
                match _ch {
                    _ => {
                        return _current_match;
                    }
                }
            }
            5 => {
                let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
                match _ch {
                    'a' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    'b' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    'c' => {
                        _current_match = Some((0, _index + 1));
                        _current_state = 6;
                        continue;
                    }
                    _ => {
                        return _current_match;
                    }
                }
            }
            6 => {
                let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
                match _ch {
                    'a' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    'b' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    'c' => {
                        _current_match = Some((1, _index + 1));
                        _current_state = 1;
                        continue;
                    }
                    _ => {
                        return _current_match;
                    }
                }
            }
            _ => { panic!("invalid state {}", _current_state); }
        }
    }
}
"#;

    if actual != expected {
        for diff in diff::lines(&actual, &expected) {
            match diff {
                diff::Result::Right(r)   => println!("- {}", r),
                diff::Result::Left(l)    => println!("+ {}", l),
                diff::Result::Both(l, _) => println!("  {}", l),
            }
        }
        panic!("bad output");
    }
}
