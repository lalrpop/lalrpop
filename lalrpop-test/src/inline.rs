extern crate lalrpop_util as __lalrpop_util;

mod __parse__E {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    extern crate lalrpop_util as __lalrpop_util;
    pub fn parse_E<
        'input,
    >(
        input: &'input str,
    ) -> Result<String, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let __ascent = __ascent::parse_E(
            input,
        );
        let __parse_table = __parse_table::parse_E(
            input,
        );
        assert_eq!(__ascent, __parse_table);
        return __ascent;
    }
    mod __ascent {

        mod __parse__E {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            extern crate lalrpop_util as __lalrpop_util;
            pub fn parse_E<
                'input,
            >(
                input: &'input str,
            ) -> Result<String, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __tokens = super::super::super::__intern_token::__Matcher::new(input);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match try!(__state0(input, &mut __tokens, __lookahead)) {
                    (Some(__lookahead), _) => {
                        Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead })
                    }
                    (None, __Nonterminal::____E((_, __nt, _))) => {
                        Ok(__nt)
                    }
                    _ => unreachable!(),
                }
            }

            #[allow(dead_code)]
            pub enum __Nonterminal<> {
                _28_29((usize, (), usize)),
                E((usize, String, usize)),
                OPT__L((usize, String, usize)),
                ____E((usize, String, usize)),
            }

            // State 0
            //     AllInputs = []
            //     OptionalInputs = []
            //     FixedInputs = []
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     E = (*) "&" E [EOF]
            //     E = (*) "&" "L" E [EOF]
            //     E = (*) "L" [EOF]
            //     __E = (*) E [EOF]
            //
            //   "&" -> S2
            //   "L" -> S3
            //
            //     E -> S1
            pub fn __state0<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state2(input, __tokens, __sym0));
                    }
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state3(input, __tokens, __sym0));
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
                            __result = try!(__state1(input, __tokens, __lookahead, __sym0));
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
            //     WillProduce = Some(__E)
            //
            //     __E = E (*) [EOF]
            //
            //   [EOF] -> __E = E => ActionFn(0);
            //
            pub fn __state1<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, String, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0(input, __sym0);
                        let __nt = __Nonterminal::____E((
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
            //     AllInputs = ["&"]
            //     OptionalInputs = []
            //     FixedInputs = ["&"]
            //     WillPushLen = 1
            //     WillPush = [E]
            //     WillProduce = Some(E)
            //
            //     E = (*) "&" E [EOF]
            //     E = "&" (*) E [EOF]
            //     E = (*) "&" "L" E [EOF]
            //     E = "&" (*) "L" E [EOF]
            //     E = (*) "L" [EOF]
            //
            //   "&" -> S2
            //   "L" -> S5
            //
            //     E -> S4
            pub fn __state2<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state2(input, __tokens, __sym1));
                    }
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(input, __tokens, __sym0, __sym1));
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
                            let __sym0 = __sym0.take().unwrap();
                            __result = try!(__state4(input, __tokens, __lookahead, __sym0, __sym1));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 3
            //     AllInputs = ["L"]
            //     OptionalInputs = []
            //     FixedInputs = ["L"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = "L" (*) [EOF]
            //
            //   [EOF] -> E = "L" => ActionFn(1);
            //
            pub fn __state3<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action1(input, __sym0);
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
            //     AllInputs = ["&", E]
            //     OptionalInputs = []
            //     FixedInputs = ["&", E]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = "&" E (*) [EOF]
            //
            //   [EOF] -> E = "&", E => ActionFn(7);
            //
            pub fn __state4<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, String, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action7(input, __sym0, __sym1);
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

            // State 5
            //     AllInputs = ["&", "L"]
            //     OptionalInputs = ["&"]
            //     FixedInputs = ["L"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = (*) "&" E [EOF]
            //     E = (*) "&" "L" E [EOF]
            //     E = "&" "L" (*) E [EOF]
            //     E = (*) "L" [EOF]
            //     E = "L" (*) [EOF]
            //
            //   "&" -> S2
            //   "L" -> S3
            //   [EOF] -> E = "L" => ActionFn(1);
            //
            //     E -> S6
            pub fn __state5<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: &mut Option<(usize, &'input str, usize)>,
                __sym1: (usize, &'input str, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state2(input, __tokens, __sym2));
                    }
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state3(input, __tokens, __sym2));
                    }
                    None => {
                        let __start = __sym1.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action1(input, __sym1);
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
                loop {
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::E(__sym2) => {
                            let __sym0 = __sym0.take().unwrap();
                            __result = try!(__state6(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 6
            //     AllInputs = ["&", "L", E]
            //     OptionalInputs = []
            //     FixedInputs = ["&", "L", E]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = "&" "L" E (*) [EOF]
            //
            //   [EOF] -> E = "&", "L", E => ActionFn(8);
            //
            pub fn __state6<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, String, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action8(input, __sym0, __sym1, __sym2);
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
        }
        pub use self::__parse__E::parse_E;
    }
    mod __parse_table {

        mod __parse__E {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            extern crate lalrpop_util as __lalrpop_util;
            #[allow(dead_code)]
            pub enum __Symbol<'input> {
                Term_22_26_22(&'input str),
                Term_22L_22(&'input str),
                Nt_28_29(()),
                NtE(String),
                NtOPT__L(String),
                Nt____E(String),
            }
            const __ACTION: &'static [i32] = &[
                // State 0
                //     E = (*) "&" E [EOF]
                //     E = (*) "&" "L" E [EOF]
                //     E = (*) "L" [EOF]
                //     __E = (*) E [EOF]
                3, // on "&", goto 2
                4, // on "L", goto 3
                // State 1
                //     __E = E (*) [EOF]
                0, // on "&", error
                0, // on "L", error
                // State 2
                //     E = (*) "&" E [EOF]
                //     E = "&" (*) E [EOF]
                //     E = (*) "&" "L" E [EOF]
                //     E = "&" (*) "L" E [EOF]
                //     E = (*) "L" [EOF]
                3, // on "&", goto 2
                6, // on "L", goto 5
                // State 3
                //     E = "L" (*) [EOF]
                0, // on "&", error
                0, // on "L", error
                // State 4
                //     E = "&" E (*) [EOF]
                0, // on "&", error
                0, // on "L", error
                // State 5
                //     E = (*) "&" E [EOF]
                //     E = (*) "&" "L" E [EOF]
                //     E = "&" "L" (*) E [EOF]
                //     E = (*) "L" [EOF]
                //     E = "L" (*) [EOF]
                3, // on "&", goto 2
                4, // on "L", goto 3
                // State 6
                //     E = "&" "L" E (*) [EOF]
                0, // on "&", error
                0, // on "L", error
            ];
            const __EOF_ACTION: &'static [i32] = &[
                0, // on EOF, error
                -7, // on EOF, reduce `__E = E => ActionFn(0);`
                0, // on EOF, error
                -2, // on EOF, reduce `E = "L" => ActionFn(1);`
                -3, // on EOF, reduce `E = "&", E => ActionFn(7);`
                -2, // on EOF, reduce `E = "L" => ActionFn(1);`
                -4, // on EOF, reduce `E = "&", "L", E => ActionFn(8);`
            ];
            const __GOTO: &'static [i32] = &[
                // State 0
                0, // on (), error
                2, // on E, goto 1
                0, // on OPT_L, error
                0, // on __E, error
                // State 1
                0, // on (), error
                0, // on E, error
                0, // on OPT_L, error
                0, // on __E, error
                // State 2
                0, // on (), error
                5, // on E, goto 4
                0, // on OPT_L, error
                0, // on __E, error
                // State 3
                0, // on (), error
                0, // on E, error
                0, // on OPT_L, error
                0, // on __E, error
                // State 4
                0, // on (), error
                0, // on E, error
                0, // on OPT_L, error
                0, // on __E, error
                // State 5
                0, // on (), error
                7, // on E, goto 6
                0, // on OPT_L, error
                0, // on __E, error
                // State 6
                0, // on (), error
                0, // on E, error
                0, // on OPT_L, error
                0, // on __E, error
            ];
            pub fn parse_E<
                'input,
            >(
                input: &'input str,
            ) -> Result<String, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __tokens = super::super::super::__intern_token::__Matcher::new(input);
                let mut __states = vec![0_i32];
                let mut __symbols = vec![];
                '__shift: loop {
                    let __lookahead = match __tokens.next() {
                        Some(Ok(v)) => v,
                        None => break '__shift,
                        Some(Err(e)) => return Err(e),
                    };
                    let __integer = match __lookahead {
                        (_, (0, _), _) if true => 0,
                        (_, (1, _), _) if true => 1,
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
                                    (0, __tok0) => __Symbol::Term_22_26_22(__tok0),
                                    _ => unreachable!(),
                                },
                                1 => match __lookahead.1 {
                                    (1, __tok0) => __Symbol::Term_22L_22(__tok0),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            __states.push(__action - 1);
                            __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                            continue '__shift;
                        } else if __action < 0 {
                            if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
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
                        if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
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
                'input,
            >(
                input: &'input str,
                __action: i32,
                __lookahead_start: Option<&usize>,
                __states: &mut ::std::vec::Vec<i32>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> Option<Result<String,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
            {
                let __nonterminal = match -__action {
                    1 => {
                        // () =  => ActionFn(5);
                        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action5(input, &__start, &__end);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 0);
                        __symbols.push((__start, __Symbol::Nt_28_29(__nt), __end));
                        0
                    }
                    2 => {
                        // E = "L" => ActionFn(1);
                        let __sym0 = __pop_Term_22L_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action1(input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtE(__nt), __end));
                        1
                    }
                    3 => {
                        // E = "&", E => ActionFn(7);
                        let __sym1 = __pop_NtE(__symbols);
                        let __sym0 = __pop_Term_22_26_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action7(input, __sym0, __sym1);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 2);
                        __symbols.push((__start, __Symbol::NtE(__nt), __end));
                        1
                    }
                    4 => {
                        // E = "&", "L", E => ActionFn(8);
                        let __sym2 = __pop_NtE(__symbols);
                        let __sym1 = __pop_Term_22L_22(__symbols);
                        let __sym0 = __pop_Term_22_26_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action8(input, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtE(__nt), __end));
                        1
                    }
                    5 => {
                        // OPT_L =  => ActionFn(6);
                        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action6(input, &__start, &__end);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 0);
                        __symbols.push((__start, __Symbol::NtOPT__L(__nt), __end));
                        2
                    }
                    6 => {
                        // OPT_L = "L" => ActionFn(4);
                        let __sym0 = __pop_Term_22L_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action4(input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtOPT__L(__nt), __end));
                        2
                    }
                    7 => {
                        // __E = E => ActionFn(0);
                        let __sym0 = __pop_NtE(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0(input, __sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 4 + __nonterminal] - 1;
                __states.push(__next_state);
                None
            }
            fn __pop_Term_22_26_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_26_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22L_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22L_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt_28_29<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, (), usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt_28_29(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtE<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, String, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtE(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtOPT__L<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, String, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtOPT__L(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____E<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, String, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt____E(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
        }
        pub use self::__parse__E::parse_E;
    }
}
pub use self::__parse__E::parse_E;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        38 => /* '&' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        76 => /* 'L' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    format!("L")
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
    (_, __1, _): (usize, String, usize),
) -> String
{
    format!("& {} {}", __0, __1)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> String
{
    format!("()")
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    format!("L")
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ()
{
    ()
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> String
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action5(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
) -> String
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action6(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, String, usize),
) -> String
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action4(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __temp0,
        __2,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
