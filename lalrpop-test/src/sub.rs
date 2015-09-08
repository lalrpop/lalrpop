#![allow(unused_imports)]
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__S {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
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
        match try!(__state0(None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____S(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        ____S(i32),
        E(i32),
        T(i32),
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
    //   "(" -> Shift(S3)
    //   Num -> Shift(S4)
    //
    //   E -> S1
    //   S -> S2
    //   T -> S5
    pub fn __state0<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok));
                __result = try!(__state3(__lookbehind, __tokens, __sym0));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(__lookbehind, __tokens, __sym0));
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
                __Nonterminal::E(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::S(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(__lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::T(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(__lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   E = E (*) "-" T [EOF]
    //   E = E (*) "-" T ["-"]
    //   S = E (*) [EOF]
    //
    //   "-" -> Shift(S6)
    //   EOF -> Reduce(S = E => Call(ActionFn(1));)
    //
    pub fn __state1<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state6(__lookbehind, __tokens, __sym0, __sym1));
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

    // State 2
    //   __S = S (*) [EOF]
    //
    //   EOF -> Reduce(__S = S => Call(ActionFn(0));)
    //
    pub fn __state2<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
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

    // State 3
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
    //   "(" -> Shift(S10)
    //   Num -> Shift(S7)
    //
    //   E -> S8
    //   T -> S9
    pub fn __state3<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state10(__lookbehind, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(__lookbehind, __tokens, __sym1));
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
                __Nonterminal::E(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(__lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(__lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   T = Num (*) [EOF]
    //   T = Num (*) ["-"]
    //
    //   "-" -> Reduce(T = Num => Call(ActionFn(4));)
    //   EOF -> Reduce(T = Num => Call(ActionFn(4));)
    //
    pub fn __state4<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, Tok::Minus(..), _)) |
            None => {
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

    // State 5
    //   E = T (*) [EOF]
    //   E = T (*) ["-"]
    //
    //   EOF -> Reduce(E = T => Call(ActionFn(3));)
    //   "-" -> Reduce(E = T => Call(ActionFn(3));)
    //
    pub fn __state5<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
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

    // State 6
    //   E = E "-" (*) T [EOF]
    //   E = E "-" (*) T ["-"]
    //   T = (*) "(" E ")" [EOF]
    //   T = (*) "(" E ")" ["-"]
    //   T = (*) Num [EOF]
    //   T = (*) Num ["-"]
    //
    //   Num -> Shift(S4)
    //   "(" -> Shift(S3)
    //
    //   T -> S11
    pub fn __state6<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state4(__lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state3(__lookbehind, __tokens, __sym2));
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
                    __result = try!(__state11(__lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   T = Num (*) [")"]
    //   T = Num (*) ["-"]
    //
    //   "-" -> Reduce(T = Num => Call(ActionFn(4));)
    //   ")" -> Reduce(T = Num => Call(ActionFn(4));)
    //
    pub fn __state7<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
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

    // State 8
    //   E = E (*) "-" T [")"]
    //   E = E (*) "-" T ["-"]
    //   T = "(" E (*) ")" [EOF]
    //   T = "(" E (*) ")" ["-"]
    //
    //   "-" -> Shift(S13)
    //   ")" -> Shift(S12)
    //
    pub fn __state8<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state13(__lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state12(__lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 9
    //   E = T (*) [")"]
    //   E = T (*) ["-"]
    //
    //   ")" -> Reduce(E = T => Call(ActionFn(3));)
    //   "-" -> Reduce(E = T => Call(ActionFn(3));)
    //
    pub fn __state9<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
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

    // State 10
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
    //   "(" -> Shift(S10)
    //   Num -> Shift(S7)
    //
    //   E -> S14
    //   T -> S9
    pub fn __state10<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state10(__lookbehind, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(__lookbehind, __tokens, __sym1));
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
                __Nonterminal::E(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(__lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(__lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 11
    //   E = E "-" T (*) [EOF]
    //   E = E "-" T (*) ["-"]
    //
    //   "-" -> Reduce(E = E, "-", T => Call(ActionFn(2));)
    //   EOF -> Reduce(E = E, "-", T => Call(ActionFn(2));)
    //
    pub fn __state11<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) |
            None => {
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

    // State 12
    //   T = "(" E ")" (*) [EOF]
    //   T = "(" E ")" (*) ["-"]
    //
    //   EOF -> Reduce(T = "(", E, ")" => Call(ActionFn(5));)
    //   "-" -> Reduce(T = "(", E, ")" => Call(ActionFn(5));)
    //
    pub fn __state12<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            None |
            Some((_, Tok::Minus(..), _)) => {
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

    // State 13
    //   E = E "-" (*) T [")"]
    //   E = E "-" (*) T ["-"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "(" E ")" ["-"]
    //   T = (*) Num [")"]
    //   T = (*) Num ["-"]
    //
    //   Num -> Shift(S7)
    //   "(" -> Shift(S10)
    //
    //   T -> S15
    pub fn __state13<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state7(__lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state10(__lookbehind, __tokens, __sym2));
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
                    __result = try!(__state15(__lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 14
    //   E = E (*) "-" T [")"]
    //   E = E (*) "-" T ["-"]
    //   T = "(" E (*) ")" [")"]
    //   T = "(" E (*) ")" ["-"]
    //
    //   "-" -> Shift(S13)
    //   ")" -> Shift(S16)
    //
    pub fn __state14<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state13(__lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state16(__lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 15
    //   E = E "-" T (*) [")"]
    //   E = E "-" T (*) ["-"]
    //
    //   "-" -> Reduce(E = E, "-", T => Call(ActionFn(2));)
    //   ")" -> Reduce(E = E, "-", T => Call(ActionFn(2));)
    //
    pub fn __state15<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::RParen(..), _)) => {
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
    //   ")" -> Reduce(T = "(", E, ")" => Call(ActionFn(5));)
    //   "-" -> Reduce(T = "(", E, ")" => Call(ActionFn(5));)
    //
    pub fn __state16<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Minus(..), _)) => {
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
pub use self::__parse__S::parse_S;

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
