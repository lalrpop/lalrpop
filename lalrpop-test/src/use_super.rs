use super::util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__S {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use super::super::util::tok::Tok;
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

            use super::super::super::super::util::tok::Tok;
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
                match try!(__state0(&mut __tokens, __lookahead, ::std::marker::PhantomData::<()>)) {
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
                S(((), i32, ())),
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
            //     S = (*) "(" ")" [EOF]
            //     __S = (*) S [EOF]
            //
            //   "(" -> S2
            //
            //     S -> S1
            pub fn __state0<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym0 = (__loc1, (__tok), __loc2);
                        __result = try!(__state2(__tokens, __sym0, ::std::marker::PhantomData::<()>));
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
                        __Nonterminal::S(__sym0) => {
                            __result = try!(__state1(__tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 1
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
            pub fn __state1<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<>(__sym0);
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

            // State 2
            //     AllInputs = ["("]
            //     OptionalInputs = []
            //     FixedInputs = ["("]
            //     WillPushLen = 1
            //     WillPush = [")"]
            //     WillProduce = Some(S)
            //
            //     S = "(" (*) ")" [EOF]
            //
            //   ")" -> S3
            //
            pub fn __state2<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: ((), Tok, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state3(__tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
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
            //     AllInputs = ["(", ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["(", ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(S)
            //
            //     S = "(" ")" (*) [EOF]
            //
            //   [EOF] -> S = "(", ")" => ActionFn(1);
            //
            pub fn __state3<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: ((), Tok, ()),
                __sym1: ((), Tok, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action1::<>(__sym0, __sym1);
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
        }
        pub use self::__parse__S::parse_S;
    }
    mod __parse_table {

        mod __parse__S {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use super::super::super::super::util::tok::Tok;
            extern crate lalrpop_util as __lalrpop_util;
            use super::super::super::__ToTriple;
            #[allow(dead_code)]
            pub enum __Symbol<> {
                Term_22_28_22(Tok),
                Term_22_29_22(Tok),
                NtS(i32),
                Nt____S(i32),
            }
            const __ACTION: &'static [i32] = &[
                // State 0
                //     S = (*) "(" ")" [EOF]
                //     __S = (*) S [EOF]
                3, // on "(", goto 2
                0, // on ")", error
                // State 1
                //     __S = S (*) [EOF]
                0, // on "(", error
                0, // on ")", error
                // State 2
                //     S = "(" (*) ")" [EOF]
                0, // on "(", error
                4, // on ")", goto 3
                // State 3
                //     S = "(" ")" (*) [EOF]
                0, // on "(", error
                0, // on ")", error
            ];
            const __EOF_ACTION: &'static [i32] = &[
                0, // on EOF, error
                -2, // on EOF, reduce `__S = S => ActionFn(0);`
                0, // on EOF, error
                -1, // on EOF, reduce `S = "(", ")" => ActionFn(1);`
            ];
            const __GOTO: &'static [i32] = &[
                // State 0
                2, // on S, goto 1
                0, // on __S, error
                // State 1
                0, // on S, error
                0, // on __S, error
                // State 2
                0, // on S, error
                0, // on __S, error
                // State 3
                0, // on S, error
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
                        _ => {
                            return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: vec![],
                            });
                        }
                    };
                    loop {
                        let __state = *__states.last().unwrap() as usize;
                        let __action = __ACTION[__state * 2 + __integer];
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
                                _ => unreachable!(),
                            };
                            __states.push(__action - 1);
                            __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                            continue '__shift;
                        } else if __action < 0 {
                            if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                        if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                _: ::std::marker::PhantomData<()>,
            ) -> Option<Result<i32,__lalrpop_util::ParseError<(),Tok,()>>>
            {
                let __nonterminal = match -__action {
                    1 => {
                        // S = "(", ")" => ActionFn(1);
                        let __sym1 = __pop_Term_22_29_22(__symbols);
                        let __sym0 = __pop_Term_22_28_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action1::<>(__sym0, __sym1);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 2);
                        __symbols.push((__start, __Symbol::NtS(__nt), __end));
                        0
                    }
                    2 => {
                        // __S = S => ActionFn(0);
                        let __sym0 = __pop_NtS(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<>(__sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 2 + __nonterminal] - 1;
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
            fn __pop_NtS<
            >(
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
            ) -> ((), i32, ()) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtS(__v), __r) => (__l, __v, __r),
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
    (_, __0, _): ((), Tok, ()),
    (_, __1, _): ((), Tok, ()),
) -> i32
{
    super::ZERO
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
