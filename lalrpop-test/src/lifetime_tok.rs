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
        match try!(__state0(None, &mut __tokens, __lookahead)) {
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
        ____Expr(Vec<&'input str>),
    }

    // State 0
    //   Expr = (*) Other* [EOF]
    //   Other* = (*) [EOF]
    //   Other* = (*) [Other]
    //   Other* = (*) Other* Other [EOF]
    //   Other* = (*) Other* Other [Other]
    //   __Expr = (*) Expr [EOF]
    //
    //   EOF -> Reduce(Other* =  => Call(ActionFn(2));)
    //   Other -> Reduce(Other* =  => Call(ActionFn(2));)
    //
    //   Expr -> S1
    //   Other* -> S2
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<((), LtTok<'input>, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LtTok<'input>, ())>,
    ) -> Result<(Option<()>, Option<((), LtTok<'input>, ())>, __Nonterminal<'input>), __ParseError<(),LtTok<'input>,()>>
    {
        let mut __result: (Option<()>, Option<((), LtTok<'input>, ())>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, LtTok::Other(_), _)) => {
                let __nt = super::__action2(&__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Other_2a(__nt));
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
                __Nonterminal::Other_2a(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(__lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __Expr = Expr (*) [EOF]
    //
    //   EOF -> Reduce(__Expr = Expr => Call(ActionFn(0));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<((), LtTok<'input>, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LtTok<'input>, ())>,
        __sym0: &mut Option<Vec<&'input str>>,
    ) -> Result<(Option<()>, Option<((), LtTok<'input>, ())>, __Nonterminal<'input>), __ParseError<(),LtTok<'input>,()>>
    {
        let mut __result: (Option<()>, Option<((), LtTok<'input>, ())>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(__sym0, &__lookbehind, &__lookahead);
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

    // State 2
    //   Expr = Other* (*) [EOF]
    //   Other* = Other* (*) Other [EOF]
    //   Other* = Other* (*) Other [Other]
    //
    //   EOF -> Reduce(Expr = Other* => Call(ActionFn(1));)
    //   Other -> Shift(S3)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<((), LtTok<'input>, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LtTok<'input>, ())>,
        __sym0: &mut Option<::std::vec::Vec<&'input str>>,
    ) -> Result<(Option<()>, Option<((), LtTok<'input>, ())>, __Nonterminal<'input>), __ParseError<(),LtTok<'input>,()>>
    {
        let mut __result: (Option<()>, Option<((), LtTok<'input>, ())>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, LtTok::Other(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state3(__lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action1(__sym0, &__lookbehind, &__lookahead);
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

    // State 3
    //   Other* = Other* Other (*) [EOF]
    //   Other* = Other* Other (*) [Other]
    //
    //   EOF -> Reduce(Other* = Other*, Other => Call(ActionFn(3));)
    //   Other -> Reduce(Other* = Other*, Other => Call(ActionFn(3));)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<((), LtTok<'input>, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'input str>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<()>, Option<((), LtTok<'input>, ())>, __Nonterminal<'input>), __ParseError<(),LtTok<'input>,()>>
    {
        let mut __result: (Option<()>, Option<((), LtTok<'input>, ())>, __Nonterminal<'input>);
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
                let __nt = super::__action3(__sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Other_2a(__nt)));
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
    __lookbehind: &Option<()>,
    __lookahead: &Option<((), LtTok<'input>, ())>,
) -> Vec<&'input str>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    __0: ::std::vec::Vec<&'input str>,
    __lookbehind: &Option<()>,
    __lookahead: &Option<((), LtTok<'input>, ())>,
) -> Vec<&'input str>
{
    (__0)
}

pub fn __action2<
    'input,
>(
    __lookbehind: &Option<()>,
    __lookahead: &Option<((), LtTok<'input>, ())>,
) -> ::std::vec::Vec<&'input str>
{
    vec![]
}

pub fn __action3<
    'input,
>(
    v: ::std::vec::Vec<&'input str>,
    e: &'input str,
    __lookbehind: &Option<()>,
    __lookahead: &Option<((), LtTok<'input>, ())>,
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
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
