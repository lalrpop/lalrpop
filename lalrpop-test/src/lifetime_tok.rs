#![allow(unused_imports)]
use lifetime_tok_lib::LtTok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use lifetime_tok_lib::LtTok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_Expr<
        'input,
        __TOKEN: __ToTriple<'input, Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<Vec<&'input str>, __ParseError<(),LtTok<'input>,()>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        let __lookbehind: () = ::std::default::Default::default();
        match try!(__state0(__lookbehind, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Expr(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        Expr(Vec<&'input str>),
        Other_2a(::std::vec::Vec<&'input str>),
        Other_2b(::std::vec::Vec<&'input str>),
        ____Expr(Vec<&'input str>),
    }

    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<((), LtTok<'input>, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LtTok<'input>, ())>,
    ) -> Result<((), Option<((), LtTok<'input>, ())>, __Nonterminal<'input>), __ParseError<(),LtTok<'input>,()>>
    {
        let mut __result: ((), Option<((), LtTok<'input>, ())>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, LtTok::Other(__tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state3(__lookbehind, __tokens, __sym0));
            }
            None => {
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action6(&__start, &__end);
                __result = (__lookbehind, __lookahead, __Nonterminal::Expr(__nt));
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
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Other_2b(__nt) => {
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
        'input,
        __TOKENS: Iterator<Item=Result<((), LtTok<'input>, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LtTok<'input>, ())>,
        __sym0: &mut Option<Vec<&'input str>>,
    ) -> Result<((), Option<((), LtTok<'input>, ())>, __Nonterminal<'input>), __ParseError<(),LtTok<'input>,()>>
    {
        let mut __result: ((), Option<((), LtTok<'input>, ())>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action0(__sym0, &__start, &__end);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<((), LtTok<'input>, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LtTok<'input>, ())>,
        __sym0: &mut Option<::std::vec::Vec<&'input str>>,
    ) -> Result<((), Option<((), LtTok<'input>, ())>, __Nonterminal<'input>), __ParseError<(),LtTok<'input>,()>>
    {
        let mut __result: ((), Option<((), LtTok<'input>, ())>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, LtTok::Other(__tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state4(__lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action7(__sym0, &__start, &__end);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
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

    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<((), LtTok<'input>, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<((), Option<((), LtTok<'input>, ())>, __Nonterminal<'input>), __ParseError<(),LtTok<'input>,()>>
    {
        let mut __result: ((), Option<((), LtTok<'input>, ())>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, LtTok::Other(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action4(__sym0, &__start, &__end);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Other_2b(__nt)));
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
        'input,
        __TOKENS: Iterator<Item=Result<((), LtTok<'input>, ()),()>>,
    >(
        __lookbehind: (),
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'input str>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<((), Option<((), LtTok<'input>, ())>, __Nonterminal<'input>), __ParseError<(),LtTok<'input>,()>>
    {
        let mut __result: ((), Option<((), LtTok<'input>, ())>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, LtTok::Other(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action5(__sym0, __sym1, &__start, &__end);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Other_2b(__nt)));
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
pub use self::__parse__Expr::parse_Expr;

pub fn __action0<
    'input,
>(
    __0: Vec<&'input str>,
    __lookbehind: &(),
    __lookahead: &(),
) -> Vec<&'input str>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    __0: ::std::vec::Vec<&'input str>,
    __lookbehind: &(),
    __lookahead: &(),
) -> Vec<&'input str>
{
    (__0)
}

pub fn __action2<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> ::std::vec::Vec<&'input str>
{
    vec![]
}

pub fn __action3<
    'input,
>(
    v: ::std::vec::Vec<&'input str>,
    __lookbehind: &(),
    __lookahead: &(),
) -> ::std::vec::Vec<&'input str>
{
    v
}

pub fn __action4<
    'input,
>(
    __0: &'input str,
    __lookbehind: &(),
    __lookahead: &(),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

pub fn __action5<
    'input,
>(
    v: ::std::vec::Vec<&'input str>,
    e: &'input str,
    __lookbehind: &(),
    __lookahead: &(),
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action6<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> Vec<&'input str>
{
    let __temp0 = __action2(
        __lookbehind,
        __lookahead,
    );
    __action1(
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action7<
    'input,
>(
    __0: ::std::vec::Vec<&'input str>,
    __lookbehind: &(),
    __lookahead: &(),
) -> Vec<&'input str>
{
    let __temp0 = __action3(
        __0,
        __lookbehind,
        __lookahead,
    );
    __action1(
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<((),LtTok<'input>,()),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for LtTok<'input> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),LtTok<'input>,()),()> {
        Ok(((), value, ()))
    }
}
impl<'input, > __ToTriple<'input, > for Result<(LtTok<'input>),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),LtTok<'input>,()),()> {
        value.map(|v| ((), v, ()))
    }
}
