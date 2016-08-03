use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__S {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    pub fn parse_S<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<i32, __lalrpop_util::ParseError<(),Tok,()>> where
      __TOKENS: Clone,
    {
        let __ascent = __ascent::parse_S(
            __tokens0.clone(),
        );
        let __parse_table = __parse_table::parse_S(
            __tokens0.clone(),
        );
        assert_eq!(__ascent, __parse_table);
        return __ascent;
    }
    mod __ascent {

        mod __parse__S {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use util::tok::Tok;
            extern crate lalrpop_util as __lalrpop_util;
            use super::super::super::__ToTriple;
            pub fn parse_S<
                __TOKEN: __ToTriple<Error=()>,
                __TOKENS: IntoIterator<Item=__TOKEN>,
            >(
                __tokens0: __TOKENS,
            ) -> Result<i32, __lalrpop_util::ParseError<(),Tok,()>>
            {
                let __tokens = __tokens0.into_iter();
                let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match try!(__state0(&mut __tokens, __lookahead)) {
                    (Some(__lookahead), _) => {
                        Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead })
                    }
                    (None, __Nonterminal::____S((_, __nt, _))) => {
                        Ok(__nt)
                    }
                    _ => unreachable!(),
                }
            }

            #[allow(dead_code)]
            pub enum __Nonterminal<> {
                E(((), i32, ())),
                S(((), i32, ())),
                T(((), i32, ())),
                ____S(((), i32, ())),
            }

            // State 0
            //     AllInputs = []
            //     OptionalInputs = []
            //     FixedInputs = []
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     E = (*) E "-" T ["-"]
            //     E = (*) E "-" T [EOF]
            //     E = (*) T ["-"]
            //     E = (*) T [EOF]
            //     S = (*) E [EOF]
            //     T = (*) "(" E ")" ["-"]
            //     T = (*) "(" E ")" [EOF]
            //     T = (*) Num ["-"]
            //     T = (*) Num [EOF]
            //     __S = (*) S [EOF]
            //
            //   "(" -> S4
            //   Num -> S5
            //
            //     E -> S1
            //     S -> S2
            //     T -> S3
            pub fn __state0<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym0 = (__loc1, (__tok), __loc2);
                        __result = try!(__state4(__tokens, __sym0));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(__tokens, __sym0));
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
                loop {
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::E(__sym0) => {
                            __result = try!(__state1(__tokens, __lookahead, __sym0));
                        }
                        __Nonterminal::S(__sym0) => {
                            __result = try!(__state2(__tokens, __lookahead, __sym0));
                        }
                        __Nonterminal::T(__sym0) => {
                            __result = try!(__state3(__tokens, __lookahead, __sym0));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 1
            //     AllInputs = [E]
            //     OptionalInputs = []
            //     FixedInputs = [E]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     E = E (*) "-" T ["-", EOF]
            //     S = E (*) [EOF]
            //
            //   "-" -> S6
            //   [EOF] -> S = E => ActionFn(1);
            //
            pub fn __state1<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state6(__tokens, __sym0, __sym1));
                        return Ok(__result);
                    }
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action1(__sym0);
                        let __nt = __Nonterminal::S((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }

            // State 2
            //     AllInputs = [S]
            //     OptionalInputs = []
            //     FixedInputs = [S]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(__S)
            //
            //     __S = S (*) [EOF]
            //
            //   [EOF] -> __S = S => ActionFn(0);
            //
            pub fn __state2<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0(__sym0);
                        let __nt = __Nonterminal::____S((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }

            // State 3
            //     AllInputs = [T]
            //     OptionalInputs = []
            //     FixedInputs = [T]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = T (*) ["-", EOF]
            //
            //   ["-", EOF] -> E = T => ActionFn(3);
            //
            pub fn __state3<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3(__sym0);
                        let __nt = __Nonterminal::E((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }

            // State 4
            //     AllInputs = ["("]
            //     OptionalInputs = []
            //     FixedInputs = ["("]
            //     WillPushLen = 2
            //     WillPush = [E, ")"]
            //     WillProduce = Some(T)
            //
            //     E = (*) E "-" T [")"]
            //     E = (*) E "-" T ["-"]
            //     E = (*) T [")"]
            //     E = (*) T ["-"]
            //     T = (*) "(" E ")" [")"]
            //     T = (*) "(" E ")" ["-"]
            //     T = "(" (*) E ")" ["-", EOF]
            //     T = (*) Num [")"]
            //     T = (*) Num ["-"]
            //
            //   "(" -> S9
            //   Num -> S10
            //
            //     E -> S7
            //     T -> S8
            pub fn __state4<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: ((), Tok, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state9(__tokens, __sym1));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state10(__tokens, __sym1));
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
                loop {
                    if __sym0.is_none() {
                        return Ok(__result);
                    }
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::E(__sym1) => {
                            __result = try!(__state7(__tokens, __lookahead, __sym0, __sym1));
                        }
                        __Nonterminal::T(__sym1) => {
                            __result = try!(__state8(__tokens, __lookahead, __sym1));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 5
            //     AllInputs = [Num]
            //     OptionalInputs = []
            //     FixedInputs = [Num]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(T)
            //
            //     T = Num (*) ["-", EOF]
            //
            //   ["-", EOF] -> T = Num => ActionFn(4);
            //
            pub fn __state5<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action4(__sym0);
                        let __nt = __Nonterminal::T((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }

            // State 6
            //     AllInputs = [E, "-"]
            //     OptionalInputs = []
            //     FixedInputs = [E, "-"]
            //     WillPushLen = 1
            //     WillPush = [T]
            //     WillProduce = Some(E)
            //
            //     E = E "-" (*) T ["-", EOF]
            //     T = (*) "(" E ")" ["-", EOF]
            //     T = (*) Num ["-", EOF]
            //
            //   "(" -> S4
            //   Num -> S5
            //
            //     T -> S11
            pub fn __state6<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
                __sym1: ((), Tok, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state4(__tokens, __sym2));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(__tokens, __sym2));
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
                loop {
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::T(__sym2) => {
                            __result = try!(__state11(__tokens, __lookahead, __sym0, __sym1, __sym2));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 7
            //     AllInputs = ["(", E]
            //     OptionalInputs = ["("]
            //     FixedInputs = [E]
            //     WillPushLen = 1
            //     WillPush = [")"]
            //     WillProduce = None
            //
            //     E = E (*) "-" T [")", "-"]
            //     T = "(" E (*) ")" ["-", EOF]
            //
            //   ")" -> S12
            //   "-" -> S13
            //
            pub fn __state7<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: &mut Option<((), Tok, ())>,
                __sym1: ((), i32, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state12(__tokens, __sym0, __sym1, __sym2));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state13(__tokens, __sym1, __sym2));
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }

            // State 8
            //     AllInputs = [T]
            //     OptionalInputs = []
            //     FixedInputs = [T]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = T (*) [")", "-"]
            //
            //   [")", "-"] -> E = T => ActionFn(3);
            //
            pub fn __state8<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3(__sym0);
                        let __nt = __Nonterminal::E((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }

            // State 9
            //     AllInputs = ["("]
            //     OptionalInputs = []
            //     FixedInputs = ["("]
            //     WillPushLen = 2
            //     WillPush = [E, ")"]
            //     WillProduce = Some(T)
            //
            //     E = (*) E "-" T [")"]
            //     E = (*) E "-" T ["-"]
            //     E = (*) T [")"]
            //     E = (*) T ["-"]
            //     T = (*) "(" E ")" [")"]
            //     T = (*) "(" E ")" ["-"]
            //     T = "(" (*) E ")" [")", "-"]
            //     T = (*) Num [")"]
            //     T = (*) Num ["-"]
            //
            //   "(" -> S9
            //   Num -> S10
            //
            //     E -> S14
            //     T -> S8
            pub fn __state9<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: ((), Tok, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state9(__tokens, __sym1));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state10(__tokens, __sym1));
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
                loop {
                    if __sym0.is_none() {
                        return Ok(__result);
                    }
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::E(__sym1) => {
                            __result = try!(__state14(__tokens, __lookahead, __sym0, __sym1));
                        }
                        __Nonterminal::T(__sym1) => {
                            __result = try!(__state8(__tokens, __lookahead, __sym1));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 10
            //     AllInputs = [Num]
            //     OptionalInputs = []
            //     FixedInputs = [Num]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(T)
            //
            //     T = Num (*) [")", "-"]
            //
            //   [")", "-"] -> T = Num => ActionFn(4);
            //
            pub fn __state10<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action4(__sym0);
                        let __nt = __Nonterminal::T((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }

            // State 11
            //     AllInputs = [E, "-", T]
            //     OptionalInputs = []
            //     FixedInputs = [E, "-", T]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = E "-" T (*) ["-", EOF]
            //
            //   ["-", EOF] -> E = E, "-", T => ActionFn(2);
            //
            pub fn __state11<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                __sym1: ((), Tok, ()),
                __sym2: ((), i32, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2(__sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::E((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }

            // State 12
            //     AllInputs = ["(", E, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["(", E, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(T)
            //
            //     T = "(" E ")" (*) ["-", EOF]
            //
            //   ["-", EOF] -> T = "(", E, ")" => ActionFn(5);
            //
            pub fn __state12<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: ((), Tok, ()),
                __sym1: ((), i32, ()),
                __sym2: ((), Tok, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5(__sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::T((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }

            // State 13
            //     AllInputs = [E, "-"]
            //     OptionalInputs = []
            //     FixedInputs = [E, "-"]
            //     WillPushLen = 1
            //     WillPush = [T]
            //     WillProduce = Some(E)
            //
            //     E = E "-" (*) T [")", "-"]
            //     T = (*) "(" E ")" [")", "-"]
            //     T = (*) Num [")", "-"]
            //
            //   "(" -> S9
            //   Num -> S10
            //
            //     T -> S15
            pub fn __state13<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
                __sym1: ((), Tok, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state9(__tokens, __sym2));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state10(__tokens, __sym2));
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
                loop {
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::T(__sym2) => {
                            __result = try!(__state15(__tokens, __lookahead, __sym0, __sym1, __sym2));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 14
            //     AllInputs = ["(", E]
            //     OptionalInputs = ["("]
            //     FixedInputs = [E]
            //     WillPushLen = 1
            //     WillPush = [")"]
            //     WillProduce = None
            //
            //     E = E (*) "-" T [")", "-"]
            //     T = "(" E (*) ")" [")", "-"]
            //
            //   ")" -> S16
            //   "-" -> S13
            //
            pub fn __state14<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: &mut Option<((), Tok, ())>,
                __sym1: ((), i32, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state16(__tokens, __sym0, __sym1, __sym2));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state13(__tokens, __sym1, __sym2));
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }

            // State 15
            //     AllInputs = [E, "-", T]
            //     OptionalInputs = []
            //     FixedInputs = [E, "-", T]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = E "-" T (*) [")", "-"]
            //
            //   [")", "-"] -> E = E, "-", T => ActionFn(2);
            //
            pub fn __state15<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                __sym1: ((), Tok, ()),
                __sym2: ((), i32, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2(__sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::E((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }

            // State 16
            //     AllInputs = ["(", E, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["(", E, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(T)
            //
            //     T = "(" E ")" (*) [")", "-"]
            //
            //   [")", "-"] -> T = "(", E, ")" => ActionFn(5);
            //
            pub fn __state16<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: ((), Tok, ()),
                __sym1: ((), i32, ()),
                __sym2: ((), Tok, ()),
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5(__sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::T((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![],
                        });
                    }
                }
            }
        }
        pub use self::__parse__S::parse_S;
    }
    mod __parse_table {

        mod __parse__S {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use util::tok::Tok;
            extern crate lalrpop_util as __lalrpop_util;
            use super::super::super::__ToTriple;
            #[allow(dead_code)]
            pub enum __Symbol<> {
                Term_22_28_22(Tok),
                Term_22_29_22(Tok),
                Term_22_2d_22(Tok),
                TermNum(i32),
                NtE(i32),
                NtS(i32),
                NtT(i32),
                Nt____S(i32),
            }
            const __ACTION: &'static [i32] = &[
                // State 0
                //     E = (*) E "-" T ["-"]
                //     E = (*) E "-" T [EOF]
                //     E = (*) T ["-"]
                //     E = (*) T [EOF]
                //     S = (*) E [EOF]
                //     T = (*) "(" E ")" ["-"]
                //     T = (*) "(" E ")" [EOF]
                //     T = (*) Num ["-"]
                //     T = (*) Num [EOF]
                //     __S = (*) S [EOF]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "-", error
                6, // on Num, goto 5
                // State 1
                //     E = E (*) "-" T ["-", EOF]
                //     S = E (*) [EOF]
                0, // on "(", error
                0, // on ")", error
                7, // on "-", goto 6
                0, // on Num, error
                // State 2
                //     __S = S (*) [EOF]
                0, // on "(", error
                0, // on ")", error
                0, // on "-", error
                0, // on Num, error
                // State 3
                //     E = T (*) ["-", EOF]
                0, // on "(", error
                0, // on ")", error
                -2, // on "-", reduce `E = T => ActionFn(3);`
                0, // on Num, error
                // State 4
                //     E = (*) E "-" T [")"]
                //     E = (*) E "-" T ["-"]
                //     E = (*) T [")"]
                //     E = (*) T ["-"]
                //     T = (*) "(" E ")" [")"]
                //     T = (*) "(" E ")" ["-"]
                //     T = "(" (*) E ")" ["-", EOF]
                //     T = (*) Num [")"]
                //     T = (*) Num ["-"]
                10, // on "(", goto 9
                0, // on ")", error
                0, // on "-", error
                11, // on Num, goto 10
                // State 5
                //     T = Num (*) ["-", EOF]
                0, // on "(", error
                0, // on ")", error
                -4, // on "-", reduce `T = Num => ActionFn(4);`
                0, // on Num, error
                // State 6
                //     E = E "-" (*) T ["-", EOF]
                //     T = (*) "(" E ")" ["-", EOF]
                //     T = (*) Num ["-", EOF]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "-", error
                6, // on Num, goto 5
                // State 7
                //     E = E (*) "-" T [")", "-"]
                //     T = "(" E (*) ")" ["-", EOF]
                0, // on "(", error
                13, // on ")", goto 12
                14, // on "-", goto 13
                0, // on Num, error
                // State 8
                //     E = T (*) [")", "-"]
                0, // on "(", error
                -2, // on ")", reduce `E = T => ActionFn(3);`
                -2, // on "-", reduce `E = T => ActionFn(3);`
                0, // on Num, error
                // State 9
                //     E = (*) E "-" T [")"]
                //     E = (*) E "-" T ["-"]
                //     E = (*) T [")"]
                //     E = (*) T ["-"]
                //     T = (*) "(" E ")" [")"]
                //     T = (*) "(" E ")" ["-"]
                //     T = "(" (*) E ")" [")", "-"]
                //     T = (*) Num [")"]
                //     T = (*) Num ["-"]
                10, // on "(", goto 9
                0, // on ")", error
                0, // on "-", error
                11, // on Num, goto 10
                // State 10
                //     T = Num (*) [")", "-"]
                0, // on "(", error
                -4, // on ")", reduce `T = Num => ActionFn(4);`
                -4, // on "-", reduce `T = Num => ActionFn(4);`
                0, // on Num, error
                // State 11
                //     E = E "-" T (*) ["-", EOF]
                0, // on "(", error
                0, // on ")", error
                -1, // on "-", reduce `E = E, "-", T => ActionFn(2);`
                0, // on Num, error
                // State 12
                //     T = "(" E ")" (*) ["-", EOF]
                0, // on "(", error
                0, // on ")", error
                -5, // on "-", reduce `T = "(", E, ")" => ActionFn(5);`
                0, // on Num, error
                // State 13
                //     E = E "-" (*) T [")", "-"]
                //     T = (*) "(" E ")" [")", "-"]
                //     T = (*) Num [")", "-"]
                10, // on "(", goto 9
                0, // on ")", error
                0, // on "-", error
                11, // on Num, goto 10
                // State 14
                //     E = E (*) "-" T [")", "-"]
                //     T = "(" E (*) ")" [")", "-"]
                0, // on "(", error
                17, // on ")", goto 16
                14, // on "-", goto 13
                0, // on Num, error
                // State 15
                //     E = E "-" T (*) [")", "-"]
                0, // on "(", error
                -1, // on ")", reduce `E = E, "-", T => ActionFn(2);`
                -1, // on "-", reduce `E = E, "-", T => ActionFn(2);`
                0, // on Num, error
                // State 16
                //     T = "(" E ")" (*) [")", "-"]
                0, // on "(", error
                -5, // on ")", reduce `T = "(", E, ")" => ActionFn(5);`
                -5, // on "-", reduce `T = "(", E, ")" => ActionFn(5);`
                0, // on Num, error
            ];
            const __EOF_ACTION: &'static [i32] = &[
                0, // on EOF, error
                -3, // on EOF, reduce `S = E => ActionFn(1);`
                -6, // on EOF, reduce `__S = S => ActionFn(0);`
                -2, // on EOF, reduce `E = T => ActionFn(3);`
                0, // on EOF, error
                -4, // on EOF, reduce `T = Num => ActionFn(4);`
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                -1, // on EOF, reduce `E = E, "-", T => ActionFn(2);`
                -5, // on EOF, reduce `T = "(", E, ")" => ActionFn(5);`
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
            ];
            const __GOTO: &'static [i32] = &[
                // State 0
                2, // on E, goto 1
                3, // on S, goto 2
                4, // on T, goto 3
                0, // on __S, error
                // State 1
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
                // State 2
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
                // State 3
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
                // State 4
                8, // on E, goto 7
                0, // on S, error
                9, // on T, goto 8
                0, // on __S, error
                // State 5
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
                // State 6
                0, // on E, error
                0, // on S, error
                12, // on T, goto 11
                0, // on __S, error
                // State 7
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
                // State 8
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
                // State 9
                15, // on E, goto 14
                0, // on S, error
                9, // on T, goto 8
                0, // on __S, error
                // State 10
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
                // State 11
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
                // State 12
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
                // State 13
                0, // on E, error
                0, // on S, error
                16, // on T, goto 15
                0, // on __S, error
                // State 14
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
                // State 15
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
                // State 16
                0, // on E, error
                0, // on S, error
                0, // on T, error
                0, // on __S, error
            ];
            pub fn parse_S<
                __TOKEN: __ToTriple<Error=()>,
                __TOKENS: IntoIterator<Item=__TOKEN>,
            >(
                __tokens0: __TOKENS,
            ) -> Result<i32, __lalrpop_util::ParseError<(),Tok,()>>
            {
                let __tokens = __tokens0.into_iter();
                let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
                let mut __states = vec![0_i32];
                let mut __symbols = vec![];
                '__shift: loop {
                    let __lookahead = match __tokens.next() {
                        Some(Ok(v)) => v,
                        None => break '__shift,
                        Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                    };
                    let __integer = match __lookahead {
                        (_, Tok::LParen, _) if true => 0,
                        (_, Tok::RParen, _) if true => 1,
                        (_, Tok::Minus, _) if true => 2,
                        (_, Tok::Num(_), _) if true => 3,
                        _ => {
                            return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: vec![],
                            });
                        }
                    };
                    loop {
                        let __state = *__states.last().unwrap() as usize;
                        let __action = __ACTION[__state * 4 + __integer];
                        if __action > 0 {
                            let __symbol = match __integer {
                                0 => match __lookahead.1 {
                                    __tok @ Tok::LParen => __Symbol::Term_22_28_22(__tok),
                                    _ => unreachable!(),
                                },
                                1 => match __lookahead.1 {
                                    __tok @ Tok::RParen => __Symbol::Term_22_29_22(__tok),
                                    _ => unreachable!(),
                                },
                                2 => match __lookahead.1 {
                                    __tok @ Tok::Minus => __Symbol::Term_22_2d_22(__tok),
                                    _ => unreachable!(),
                                },
                                3 => match __lookahead.1 {
                                    Tok::Num(__tok0) => __Symbol::TermNum(__tok0),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            __states.push(__action - 1);
                            __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                            continue '__shift;
                        } else if __action < 0 {
                            if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                                return r;
                            }
                        } else {
                            return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: vec![],
                            });
                        }
                    }
                }
                loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __EOF_ACTION[__state];
                    if __action < 0 {
                        if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols) {
                            return r;
                        }
                    } else {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: None,
                            expected: vec![],
                        });
                    }
                }
            }
            pub fn __reduce<
            >(
                __action: i32,
                __lookahead_start: Option<&()>,
                __states: &mut ::std::vec::Vec<i32>,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> Option<Result<i32,__lalrpop_util::ParseError<(),Tok,()>>>
            {
                let __nonterminal = match -__action {
                    1 => {
                        // E = E, "-", T => ActionFn(2);
                        let __sym2 = __pop_NtT(__symbols);
                        let __sym1 = __pop_Term_22_2d_22(__symbols);
                        let __sym0 = __pop_NtE(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2(__sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtE(__nt), __end));
                        0
                    }
                    2 => {
                        // E = T => ActionFn(3);
                        let __sym0 = __pop_NtT(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3(__sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtE(__nt), __end));
                        0
                    }
                    3 => {
                        // S = E => ActionFn(1);
                        let __sym0 = __pop_NtE(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action1(__sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtS(__nt), __end));
                        1
                    }
                    4 => {
                        // T = Num => ActionFn(4);
                        let __sym0 = __pop_TermNum(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action4(__sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtT(__nt), __end));
                        2
                    }
                    5 => {
                        // T = "(", E, ")" => ActionFn(5);
                        let __sym2 = __pop_Term_22_29_22(__symbols);
                        let __sym1 = __pop_NtE(__symbols);
                        let __sym0 = __pop_Term_22_28_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5(__sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtT(__nt), __end));
                        2
                    }
                    6 => {
                        // __S = S => ActionFn(0);
                        let __sym0 = __pop_NtS(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0(__sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 4 + __nonterminal] - 1;
                __states.push(__next_state);
                None
            }
            fn __pop_Term_22_28_22<
            >(
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
            ) -> ((), Tok, ()) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_29_22<
            >(
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
            ) -> ((), Tok, ()) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2d_22<
            >(
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
            ) -> ((), Tok, ()) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_TermNum<
            >(
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
            ) -> ((), i32, ()) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::TermNum(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtE<
            >(
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
            ) -> ((), i32, ()) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtE(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtS<
            >(
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
            ) -> ((), i32, ()) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtS(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtT<
            >(
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
            ) -> ((), i32, ()) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtT(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____S<
            >(
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
            ) -> ((), i32, ()) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt____S(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
        }
        pub use self::__parse__S::parse_S;
    }
}
pub use self::__parse__S::parse_S;

pub fn __action0<
>(
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

pub fn __action1<
>(
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

pub fn __action2<
>(
    (_, l, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    (_, r, _): ((), i32, ()),
) -> i32
{
    l - r
}

pub fn __action3<
>(
    (_, t, _): ((), i32, ()),
) -> i32
{
    t - super::ZERO
}

pub fn __action4<
>(
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

pub fn __action5<
>(
    (_, _, _): ((), Tok, ()),
    (_, __0, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
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
