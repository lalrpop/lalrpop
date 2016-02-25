#![allow(unused_imports)]
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Items {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_Items<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<Vec<(usize, usize)>, __ParseError<usize,Tok,()>>
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
        _40L(usize),
        _40R(usize),
        Items(Vec<(usize, usize)>),
        Spanned_3c_22_2b_22_3e((usize, usize)),
        ____Items(Vec<(usize, usize)>),
    }

    pub fn __state0<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (usize, Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action6(&__start, &__end);
                __result = (__lookbehind, __lookahead, __Nonterminal::_40L(__nt));
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
                __Nonterminal::_40L(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Items(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(__lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<usize>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (usize, Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action5(&__start, &__end);
                __result = (__lookbehind, __lookahead, __Nonterminal::_40R(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_40R(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state3(__lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state2<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<Vec<(usize, usize)>>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (usize, Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Minus, __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state6(__lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::Plus, _)) => {
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action6(&__start, &__end);
                __result = (__lookbehind, __lookahead, __Nonterminal::_40L(__nt));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action0(__sym0, &__start, &__end);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Items(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_40L(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(__lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Spanned_3c_22_2b_22_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(__lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state3<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<usize>,
        __sym1: &mut Option<usize>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (usize, Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action1(__sym0, __sym1, &__start, &__end);
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

    pub fn __state4<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<usize>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (usize, Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Plus, __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state7(__lookbehind, __tokens, __sym0, __sym1));
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

    pub fn __state5<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<Vec<(usize, usize)>>,
        __sym1: &mut Option<(usize, usize)>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (usize, Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action2(__sym0, __sym1, &__start, &__end);
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

    pub fn __state6<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Vec<(usize, usize)>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
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
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action3(__sym0, __sym1, &__start, &__end);
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

    pub fn __state7<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<usize>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
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
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action5(&__start, &__end);
                __result = (__lookbehind, __lookahead, __Nonterminal::_40R(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_40R(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(__lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state8<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<usize>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<usize>,
    ) -> Result<(usize, Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (usize, Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action4(__sym0, __sym1, __sym2, &__start, &__end);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Spanned_3c_22_2b_22_3e(__nt)));
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
    __lookahead: &usize,
) -> Vec<(usize, usize)>
{
    (__0)
}

pub fn __action1<
>(
    __0: usize,
    __1: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(usize, usize)>
{
    vec![(__0, __1)]
}

pub fn __action2<
>(
    v: Vec<(usize, usize)>,
    e: (usize, usize),
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(usize, usize)>
{
    {
        let mut v = v;
        v.push(e);
        v
    }
}

pub fn __action3<
>(
    v: Vec<(usize, usize)>,
    _: Tok,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(usize, usize)>
{
    v
}

pub fn __action4<
>(
    __0: usize,
    _: Tok,
    __1: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> (usize, usize)
{
    (__0, __1)
}

pub fn __action5<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

pub fn __action6<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),Self::Error>;
}

impl<> __ToTriple<> for (usize, Tok, usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),()> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, Tok, usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),()> {
        value
    }
}
