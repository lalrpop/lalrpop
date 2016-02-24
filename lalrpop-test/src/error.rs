#![allow(unused_imports)]
use util::tok::Tok;
use lalrpop_util::ParseError;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Items {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    use lalrpop_util::ParseError;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_Items<
        __TOKEN: __ToTriple<Error=char>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<Vec<(usize, usize)>, __ParseError<usize,Tok,char>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        let __lookbehind: usize = ::std::default::Default::default();
        match try!(__state0(__lookbehind, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Items(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        Items(Vec<(usize, usize)>),
        ____Items(Vec<(usize, usize)>),
    }

    pub fn __state0<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (usize, Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __nt = super::__action1(&__lookbehind, __lookahead.as_ref().map(|o| &o.0));
                __result = (__lookbehind, __lookahead, __Nonterminal::Items(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Items(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<Vec<(usize, usize)>>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (usize, Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Plus, __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state2(__lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Minus, __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state3(__lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(__sym0, &__lookbehind, __lookahead.as_ref().map(|o| &o.0));
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Items(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state2<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Vec<(usize, usize)>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (usize, Option<(usize, Tok, usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = try!(super::__action2(__sym0, __sym1, &__lookbehind, __lookahead.as_ref().map(|o| &o.0)));
                return Ok((__lookbehind, __lookahead, __Nonterminal::Items(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state3<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Vec<(usize, usize)>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (usize, Option<(usize, Tok, usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = try!(super::__action3(__sym0, __sym1, &__lookbehind, __lookahead.as_ref().map(|o| &o.0)));
                return Ok((__lookbehind, __lookahead, __Nonterminal::Items(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Items::parse_Items;

pub fn __action0<
>(
    __0: Vec<(usize, usize)>,
    __lookbehind: &usize,
    __lookahead: Option<&usize>,
) -> Vec<(usize, usize)>
{
    (__0)
}

pub fn __action1<
>(
    __lookbehind: &usize,
    __lookahead: Option<&usize>,
) -> Vec<(usize, usize)>
{
    vec![]
}

pub fn __action2<
>(
    __0: Vec<(usize, usize)>,
    __1: Tok,
    __lookbehind: &usize,
    __lookahead: Option<&usize>,
) -> Result<Vec<(usize, usize)>,__ParseError<usize,Tok,char>>
{
    Err(ParseError::User { error: '+' })
}

pub fn __action3<
>(
    v: Vec<(usize, usize)>,
    _: Tok,
    __lookbehind: &usize,
    __lookahead: Option<&usize>,
) -> Result<Vec<(usize, usize)>,__ParseError<usize,Tok,char>>
{
    Ok(v)
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),Self::Error>;
}

impl<> __ToTriple<> for (usize, Tok, usize) {
    type Error = char;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),char> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, Tok, usize),char> {
    type Error = char;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),char> {
        value
    }
}
