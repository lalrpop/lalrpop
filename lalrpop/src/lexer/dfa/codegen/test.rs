use diff;
use lexer::dfa::Precedence;
use lexer::dfa::test::dfa;
use lexer::dfa::codegen;
use rust::RustWrite;

const P1: Precedence = Precedence(1);
const P0: Precedence = Precedence(0);

#[test]
fn codegen() {
    let dfa = dfa(&[
        /* 0 */ (r#"abc"#, P1),
        /* 1 */ (r#"[a-c]+"#, P0),
        /* 3 */ (r#" +"#, P0),
        /* 4 */ (r#">>"#, P0),
        /* 5 */ (r#">"#, P0),
        ]).unwrap();

    let mut buffer = vec![];
    codegen::compile("_", &dfa, &mut RustWrite::new(&mut buffer)).unwrap();
    let actual = String::from_utf8(buffer).unwrap();
    println!("{}", actual);

    let expected = r#"\
fn _stateDFA0<'input,_CHARS>(
    mut _chars: _CHARS,
    _current_match: Option<(usize, usize)>,
) -> Option<(usize, usize)>
where _CHARS: Iterator<Item=(usize, char)>
{
    let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
    match _ch {
        ' ' => {
            _stateDFA2(
                _chars,
                Some((2, _index + 1)),
            )
        }
        '>' => {
            _stateDFA3(
                _chars,
                Some((4, _index + 1)),
            )
        }
        'a' => {
            _stateDFA4(
                _chars,
                Some((1, _index + 1)),
            )
        }
        'b' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        'c' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        _ => {
            _current_match
        }
    }
}
fn _stateDFA1<'input,_CHARS>(
    mut _chars: _CHARS,
    _current_match: Option<(usize, usize)>,
) -> Option<(usize, usize)>
where _CHARS: Iterator<Item=(usize, char)>
{
    let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
    match _ch {
        'a' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        'b' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        'c' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        _ => {
            _current_match
        }
    }
}
fn _stateDFA2<'input,_CHARS>(
    mut _chars: _CHARS,
    _current_match: Option<(usize, usize)>,
) -> Option<(usize, usize)>
where _CHARS: Iterator<Item=(usize, char)>
{
    let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
    match _ch {
        ' ' => {
            _stateDFA6(
                _chars,
                Some((2, _index + 1)),
            )
        }
        _ => {
            _current_match
        }
    }
}
fn _stateDFA3<'input,_CHARS>(
    mut _chars: _CHARS,
    _current_match: Option<(usize, usize)>,
) -> Option<(usize, usize)>
where _CHARS: Iterator<Item=(usize, char)>
{
    let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
    match _ch {
        '>' => {
            _stateDFA7(
                _chars,
                Some((3, _index + 1)),
            )
        }
        _ => {
            _current_match
        }
    }
}
fn _stateDFA4<'input,_CHARS>(
    mut _chars: _CHARS,
    _current_match: Option<(usize, usize)>,
) -> Option<(usize, usize)>
where _CHARS: Iterator<Item=(usize, char)>
{
    let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
    match _ch {
        'a' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        'b' => {
            _stateDFA8(
                _chars,
                Some((1, _index + 1)),
            )
        }
        'c' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        _ => {
            _current_match
        }
    }
}
fn _stateDFA5<'input,_CHARS>(
    mut _chars: _CHARS,
    _current_match: Option<(usize, usize)>,
) -> Option<(usize, usize)>
where _CHARS: Iterator<Item=(usize, char)>
{
    let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
    match _ch {
        _ => {
            _current_match
        }
    }
}
fn _stateDFA6<'input,_CHARS>(
    mut _chars: _CHARS,
    _current_match: Option<(usize, usize)>,
) -> Option<(usize, usize)>
where _CHARS: Iterator<Item=(usize, char)>
{
    let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
    match _ch {
        ' ' => {
            _stateDFA6(
                _chars,
                Some((2, _index + 1)),
            )
        }
        _ => {
            _current_match
        }
    }
}
fn _stateDFA7<'input,_CHARS>(
    mut _chars: _CHARS,
    _current_match: Option<(usize, usize)>,
) -> Option<(usize, usize)>
where _CHARS: Iterator<Item=(usize, char)>
{
    let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
    match _ch {
        _ => {
            _current_match
        }
    }
}
fn _stateDFA8<'input,_CHARS>(
    mut _chars: _CHARS,
    _current_match: Option<(usize, usize)>,
) -> Option<(usize, usize)>
where _CHARS: Iterator<Item=(usize, char)>
{
    let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
    match _ch {
        'a' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        'b' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        'c' => {
            _stateDFA9(
                _chars,
                Some((0, _index + 1)),
            )
        }
        _ => {
            _current_match
        }
    }
}
fn _stateDFA9<'input,_CHARS>(
    mut _chars: _CHARS,
    _current_match: Option<(usize, usize)>,
) -> Option<(usize, usize)>
where _CHARS: Iterator<Item=(usize, char)>
{
    let (_index, _ch) = match _chars.next() { Some(p) => p, None => return _current_match };
    match _ch {
        'a' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        'b' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        'c' => {
            _stateDFA1(
                _chars,
                Some((1, _index + 1)),
            )
        }
        _ => {
            _current_match
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
    }
}
