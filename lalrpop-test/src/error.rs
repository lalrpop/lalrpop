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
        match try!(__state0(&mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____Items((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        Items((usize, Vec<(usize, usize)>, usize)),
        ____Items((usize, Vec<(usize, usize)>, usize)),
    }

    // State 0
    //   Items = (*) [EOF]
    //   Items = (*) ["+"]
    //   Items = (*) ["-"]
    //   Items = (*) Items "+" [EOF]
    //   Items = (*) Items "+" ["+"]
    //   Items = (*) Items "+" ["-"]
    //   Items = (*) Items "-" [EOF]
    //   Items = (*) Items "-" ["+"]
    //   Items = (*) Items "-" ["-"]
    //   __Items = (*) Items [EOF]
    //
    //   EOF -> Reduce(Items =  => ActionFn(1);)
    //   "+" -> Reduce(Items =  => ActionFn(1);)
    //   "-" -> Reduce(Items =  => ActionFn(1);)
    //
    //   Items -> S1
    pub fn __state0<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __start: usize = ::std::default::Default::default();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action1(&__start, &__end);
                let __nt = __Nonterminal::Items((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Items(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Items = Items (*) "+" [EOF]
    //   Items = Items (*) "+" ["+"]
    //   Items = Items (*) "+" ["-"]
    //   Items = Items (*) "-" [EOF]
    //   Items = Items (*) "-" ["+"]
    //   Items = Items (*) "-" ["-"]
    //   __Items = Items (*) [EOF]
    //
    //   EOF -> Reduce(__Items = Items => ActionFn(0);)
    //   "+" -> Shift(S2)
    //   "-" -> Shift(S3)
    //
    pub fn __state1<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<(usize, Vec<(usize, usize)>, usize)>,
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state2(__tokens, __sym0, __sym1));
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state3(__tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(__sym0);
                let __nt = __Nonterminal::____Items((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
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

    // State 2
    //   Items = Items "+" (*) [EOF]
    //   Items = Items "+" (*) ["+"]
    //   Items = Items "+" (*) ["-"]
    //
    //   EOF -> Reduce(Items = Items, "+" => ActionFn(2);)
    //   "+" -> Reduce(Items = Items, "+" => ActionFn(2);)
    //   "-" -> Reduce(Items = Items, "+" => ActionFn(2);)
    //
    pub fn __state2<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Vec<(usize, usize)>, usize)>,
        __sym1: &mut Option<(usize, Tok, usize)>,
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
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
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = try!(super::__action2(__sym0, __sym1));
                let __nt = __Nonterminal::Items((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 3
    //   Items = Items "-" (*) [EOF]
    //   Items = Items "-" (*) ["+"]
    //   Items = Items "-" (*) ["-"]
    //
    //   EOF -> Reduce(Items = Items, "-" => ActionFn(3);)
    //   "+" -> Reduce(Items = Items, "-" => ActionFn(3);)
    //   "-" -> Reduce(Items = Items, "-" => ActionFn(3);)
    //
    pub fn __state3<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Vec<(usize, usize)>, usize)>,
        __sym1: &mut Option<(usize, Tok, usize)>,
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
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
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = try!(super::__action3(__sym0, __sym1));
                let __nt = __Nonterminal::Items((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
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
    (_, __0, _): (usize, Vec<(usize, usize)>, usize),
) -> Vec<(usize, usize)>
{
    (__0)
}

pub fn __action1<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(usize, usize)>
{
    vec![]
}

pub fn __action2<
>(
    (_, __0, _): (usize, Vec<(usize, usize)>, usize),
    (_, __1, _): (usize, Tok, usize),
) -> Result<Vec<(usize, usize)>,__ParseError<usize,Tok,char>>
{
    Err(ParseError::User { error: '+' })
}

pub fn __action3<
>(
    (_, v, _): (usize, Vec<(usize, usize)>, usize),
    (_, _, _): (usize, Tok, usize),
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
