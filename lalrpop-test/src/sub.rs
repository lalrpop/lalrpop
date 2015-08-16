#![allow(unused_imports)]
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;
#[allow(non_snake_case)]
pub fn parse_S<
    __TOKEN: __ToTriple<Error=()>,
    __TOKENS: IntoIterator<Item=__TOKEN>,
>(
    __tokens: __TOKENS,
) -> Result<i32, __ParseError<(),Tok,()>>
{
    let mut __tokens = __tokens.into_iter();
    let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
    let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
    match try!(__parse__S::__state0(None, __lookahead, &mut __tokens)) {
        (_, Some(__lookahead), _) => {
            Err(__ParseError::ExtraToken { token: __lookahead })
        }
        (_, None, __parse__S::__Nonterminal::____S(__nt)) => {
            Ok(__nt)
        }
        _ => unreachable!(),
    }
}

mod __parse__S {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        T(i32),
        E(i32),
        ____S(i32),
        S(i32),
    }

    // State 0
    //   E = (*) E "-" T [EOF]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T [EOF]
    //   E = (*) T ["-"]
    //   S = (*) E [EOF]
    //   T = (*) "(" E ")" [EOF]
    //   T = (*) "(" E ")" ["-"]
    //   T = (*) Num [EOF]
    //   T = (*) Num ["-"]
    //   __S = (*) S [EOF]
    //
    //   "(" -> Shift(S2)
    //   Num -> Shift(S3)
    //
    //   S -> S4
    //   T -> S1
    //   E -> S5
    pub fn __state0<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state2(__lookbehind, __lookahead, __tokens, __sym0));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state3(__lookbehind, __lookahead, __tokens, __sym0));
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
                __Nonterminal::S(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(__lookbehind, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::T(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__lookbehind, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::E(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(__lookbehind, __lookahead, __tokens, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   E = T (*) [EOF]
    //   E = T (*) ["-"]
    //
    //   EOF -> Reduce(E = T => Call(ActionFn(3));)
    //   "-" -> Reduce(E = T => Call(ActionFn(3));)
    //
    pub fn __state1<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(__sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::E(__nt)));
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
    //   E = (*) E "-" T [")"]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T [")"]
    //   E = (*) T ["-"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "(" E ")" ["-"]
    //   T = "(" (*) E ")" [EOF]
    //   T = "(" (*) E ")" ["-"]
    //   T = (*) Num [")"]
    //   T = (*) Num ["-"]
    //
    //   Num -> Shift(S9)
    //   "(" -> Shift(S7)
    //
    //   T -> S8
    //   E -> S6
    pub fn __state2<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state9(__lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state7(__lookbehind, __lookahead, __tokens, __sym1));
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
                __Nonterminal::T(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(__lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::E(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(__lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 3
    //   T = Num (*) [EOF]
    //   T = Num (*) ["-"]
    //
    //   EOF -> Reduce(T = Num => Call(ActionFn(4));)
    //   "-" -> Reduce(T = Num => Call(ActionFn(4));)
    //
    pub fn __state3<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(__sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   __S = S (*) [EOF]
    //
    //   EOF -> Reduce(__S = S => Call(ActionFn(0));)
    //
    pub fn __state4<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(__sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____S(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   E = E (*) "-" T [EOF]
    //   E = E (*) "-" T ["-"]
    //   S = E (*) [EOF]
    //
    //   EOF -> Reduce(S = E => Call(ActionFn(1));)
    //   "-" -> Shift(S10)
    //
    pub fn __state5<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state10(__lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action1(__sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::S(__nt)));
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

    // State 6
    //   E = E (*) "-" T [")"]
    //   E = E (*) "-" T ["-"]
    //   T = "(" E (*) ")" [EOF]
    //   T = "(" E (*) ")" ["-"]
    //
    //   "-" -> Shift(S12)
    //   ")" -> Shift(S11)
    //
    pub fn __state6<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state12(__lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state11(__lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
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

    // State 7
    //   E = (*) E "-" T [")"]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T [")"]
    //   E = (*) T ["-"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "(" E ")" ["-"]
    //   T = "(" (*) E ")" [")"]
    //   T = "(" (*) E ")" ["-"]
    //   T = (*) Num [")"]
    //   T = (*) Num ["-"]
    //
    //   Num -> Shift(S9)
    //   "(" -> Shift(S7)
    //
    //   T -> S8
    //   E -> S13
    pub fn __state7<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state9(__lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state7(__lookbehind, __lookahead, __tokens, __sym1));
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
                __Nonterminal::T(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(__lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::E(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(__lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 8
    //   E = T (*) [")"]
    //   E = T (*) ["-"]
    //
    //   ")" -> Reduce(E = T => Call(ActionFn(3));)
    //   "-" -> Reduce(E = T => Call(ActionFn(3));)
    //
    pub fn __state8<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(__sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::E(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   T = Num (*) [")"]
    //   T = Num (*) ["-"]
    //
    //   "-" -> Reduce(T = Num => Call(ActionFn(4));)
    //   ")" -> Reduce(T = Num => Call(ActionFn(4));)
    //
    pub fn __state9<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(__sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 10
    //   E = E "-" (*) T [EOF]
    //   E = E "-" (*) T ["-"]
    //   T = (*) "(" E ")" [EOF]
    //   T = (*) "(" E ")" ["-"]
    //   T = (*) Num [EOF]
    //   T = (*) Num ["-"]
    //
    //   "(" -> Shift(S2)
    //   Num -> Shift(S3)
    //
    //   T -> S14
    pub fn __state10<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state2(__lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state3(__lookbehind, __lookahead, __tokens, __sym2));
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
                __Nonterminal::T(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state14(__lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 11
    //   T = "(" E ")" (*) [EOF]
    //   T = "(" E ")" (*) ["-"]
    //
    //   "-" -> Reduce(T = "(", E, ")" => Call(ActionFn(5));)
    //   EOF -> Reduce(T = "(", E, ")" => Call(ActionFn(5));)
    //
    pub fn __state11<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) |
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(__sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   E = E "-" (*) T [")"]
    //   E = E "-" (*) T ["-"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "(" E ")" ["-"]
    //   T = (*) Num [")"]
    //   T = (*) Num ["-"]
    //
    //   Num -> Shift(S9)
    //   "(" -> Shift(S7)
    //
    //   T -> S15
    pub fn __state12<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state9(__lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state7(__lookbehind, __lookahead, __tokens, __sym2));
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
                __Nonterminal::T(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state15(__lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 13
    //   E = E (*) "-" T [")"]
    //   E = E (*) "-" T ["-"]
    //   T = "(" E (*) ")" [")"]
    //   T = "(" E (*) ")" ["-"]
    //
    //   ")" -> Shift(S16)
    //   "-" -> Shift(S12)
    //
    pub fn __state13<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state16(__lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state12(__lookbehind, __lookahead, __tokens, __sym1, __sym2));
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

    // State 14
    //   E = E "-" T (*) [EOF]
    //   E = E "-" T (*) ["-"]
    //
    //   EOF -> Reduce(E = E, "-", T => Call(ActionFn(2));)
    //   "-" -> Reduce(E = E, "-", T => Call(ActionFn(2));)
    //
    pub fn __state14<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(__sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::E(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //   E = E "-" T (*) [")"]
    //   E = E "-" T (*) ["-"]
    //
    //   ")" -> Reduce(E = E, "-", T => Call(ActionFn(2));)
    //   "-" -> Reduce(E = E, "-", T => Call(ActionFn(2));)
    //
    pub fn __state15<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(__sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::E(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 16
    //   T = "(" E ")" (*) [")"]
    //   T = "(" E ")" (*) ["-"]
    //
    //   "-" -> Reduce(T = "(", E, ")" => Call(ActionFn(5));)
    //   ")" -> Reduce(T = "(", E, ")" => Call(ActionFn(5));)
    //
    pub fn __state16<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __lookahead: Option<((), Tok, ())>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(__sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::T(__nt)));
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

pub fn __action0<
>(
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action1<
>(
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action2<
>(
    l: i32,
    _: Tok,
    r: i32,
) -> i32
{
    l - r
}

pub fn __action3<
>(
    t: i32,
) -> i32
{
    t - super::ZERO
}

pub fn __action4<
>(
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action5<
>(
    _: Tok,
    __0: i32,
    _: Tok,
) -> i32
{
    (__0)
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<((),Tok,()),Self::Error>;
}

impl<> __ToTriple<> for Tok {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Tok,()),()> {
        Ok(((), value, ()))
    }
}
impl<> __ToTriple<> for Result<(Tok),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Tok,()),()> {
        value.map(|v| ((), v, ()))
    }
}
