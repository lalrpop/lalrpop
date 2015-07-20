#![allow(unused_imports)]
use super::util::tok::Tok;

#[allow(non_snake_case)]
pub fn parse_S<
    __TOKENS: IntoIterator<Item=Tok>,
>(
    __tokens: __TOKENS,
) -> Result<i32, Option<Tok>>
{
    let mut __tokens = __tokens.into_iter();
    let mut __tokens = __tokens.map(|t| ((), t, ()));
    let __lookahead = __tokens.next();
    match __parse__S::__state0(None, __lookahead, &mut __tokens) {
        Err(None) => {
            Err(None)
        }
        Err(Some(__lookahead)) => {
            Err(Some(__lookahead.1))
        }
        Ok((_, Some(__lookahead), _)) => {
            Err(Some(__lookahead.1))
        }
        Ok((_, None, __parse__S::__Nonterminal::____S(__nt))) => {
            Ok(__nt)
        }
        Ok(_) => unreachable!(),
    }
}

mod __parse__S {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use super::super::util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;

    pub enum __Nonterminal<> {
        S(i32),
        ____S(i32),
    }

    // State 0
    //   S = (*) "(" ")" [EOF]
    //   __S = (*) S [EOF]
    //
    //   "(" -> Shift(S2)
    //
    //   S -> S1
    pub fn __state0<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state2(__lookbehind, __lookahead, __tokens, __sym0));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::S(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__lookbehind, __lookahead, __tokens, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __S = S (*) [EOF]
    //
    //   EOF -> Reduce(__S = S => Call(ActionFn(0));)
    //
    pub fn __state1<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(__sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____S(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 2
    //   S = "(" (*) ")" [EOF]
    //
    //   ")" -> Shift(S3)
    //
    pub fn __state2<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state3(__lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 3
    //   S = "(" ")" (*) [EOF]
    //
    //   EOF -> Reduce(S = "(", ")" => Call(ActionFn(1));)
    //
    pub fn __state3<
        __TOKENS: Iterator<Item=((), Tok, ())>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), Option<((), Tok, ())>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action1(__sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::S(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }
}

pub fn __action0<
>(
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action1<
>(
    __0: Tok,
    __1: Tok,
) -> i32
{
    super::ZERO
}
