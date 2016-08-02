#![allow(unused_imports)]
#![allow(unused_variables)]
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Items {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Items<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<(usize, usize)>, __ParseError<usize,(usize, &'input str),()>>
    {
        let __ascent = __ascent::parse_Items(
            input,
        );
        let __parse_table = __parse_table::parse_Items(
            input,
        );
        assert_eq!(__ascent, __parse_table);
        return __ascent;
    }
    mod __ascent {

        mod __parse__Items {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use util::tok::Tok;
            extern crate lalrpop_util as __lalrpop_util;
            use self::__lalrpop_util::ParseError as __ParseError;
            pub fn parse_Items<
                'input,
            >(
                input: &'input str,
            ) -> Result<Vec<(usize, usize)>, __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __tokens = super::super::super::__intern_token::__Matcher::new(input);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match try!(__state0(input, &mut __tokens, __lookahead)) {
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
                _40L((usize, usize, usize)),
                _40R((usize, usize, usize)),
                Items((usize, Vec<(usize, usize)>, usize)),
                Spanned_3c_22_2b_22_3e((usize, (usize, usize), usize)),
                ____Items((usize, Vec<(usize, usize)>, usize)),
            }

            // State 0
            //     AllInputs = []
            //     OptionalInputs = []
            //     FixedInputs = []
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Items = (*) ["+"]
            //     Items = (*) ["-"]
            //     Items = (*) [EOF]
            //     Items = (*) Items Spanned<"+"> ["+"]
            //     Items = (*) Items Spanned<"+"> ["-"]
            //     Items = (*) Items Spanned<"+"> [EOF]
            //     Items = (*) Items "-" ["+"]
            //     Items = (*) Items "-" ["-"]
            //     Items = (*) Items "-" [EOF]
            //     __Items = (*) Items [EOF]
            //
            //   ["+"] -> Items =  => ActionFn(9);
            //   ["-"] -> Items =  => ActionFn(9);
            //   [EOF] -> Items =  => ActionFn(9);
            //
            //     Items -> S1
            pub fn __state0<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, (0, _), _)) |
                    Some((_, (1, _), _)) |
                    None => {
                        let __start: usize = ::std::default::Default::default();
                        let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action9(input, &__start, &__end);
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
                        __Nonterminal::Items(__sym0) => {
                            __result = try!(__state1(input, __tokens, __lookahead, __sym0));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 1
            //     AllInputs = [Items]
            //     OptionalInputs = []
            //     FixedInputs = [Items]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Items = Items (*) Spanned<"+"> ["+", "-", EOF]
            //     Items = Items (*) "-" ["+", "-", EOF]
            //     Spanned<"+"> = (*) "+" ["+", "-", EOF]
            //     __Items = Items (*) [EOF]
            //
            //   "+" -> S3
            //   "-" -> S4
            //   [EOF] -> __Items = Items => ActionFn(0);
            //
            //     Spanned<"+"> -> S2
            pub fn __state1<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, Vec<(usize, usize)>, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state3(input, __tokens, __sym1));
                    }
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state4(input, __tokens, __sym0, __sym1));
                        return Ok(__result);
                    }
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0(input, __sym0);
                        let __nt = __Nonterminal::____Items((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
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
                        __Nonterminal::Spanned_3c_22_2b_22_3e(__sym1) => {
                            __result = try!(__state2(input, __tokens, __lookahead, __sym0, __sym1));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 2
            //     AllInputs = [Items, Spanned<"+">]
            //     OptionalInputs = []
            //     FixedInputs = [Items, Spanned<"+">]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Items)
            //
            //     Items = Items Spanned<"+"> (*) ["+", "-", EOF]
            //
            //   ["+", "-", EOF] -> Items = Items, Spanned<"+"> => ActionFn(2);
            //
            pub fn __state2<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, Vec<(usize, usize)>, usize),
                __sym1: (usize, (usize, usize), usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, (0, _), _)) |
                    Some((_, (1, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action2(input, __sym0, __sym1);
                        let __nt = __Nonterminal::Items((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
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
            //     AllInputs = ["+"]
            //     OptionalInputs = []
            //     FixedInputs = ["+"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Spanned<"+">)
            //
            //     Spanned<"+"> = "+" (*) ["+", "-", EOF]
            //
            //   ["+", "-", EOF] -> Spanned<"+"> = "+" => ActionFn(10);
            //
            pub fn __state3<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (0, _), _)) |
                    Some((_, (1, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action10(input, __sym0);
                        let __nt = __Nonterminal::Spanned_3c_22_2b_22_3e((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
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
            //     AllInputs = [Items, "-"]
            //     OptionalInputs = []
            //     FixedInputs = [Items, "-"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Items)
            //
            //     Items = Items "-" (*) ["+", "-", EOF]
            //
            //   ["+", "-", EOF] -> Items = Items, "-" => ActionFn(3);
            //
            pub fn __state4<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Vec<(usize, usize)>, usize),
                __sym1: (usize, &'input str, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (0, _), _)) |
                    Some((_, (1, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action3(input, __sym0, __sym1);
                        let __nt = __Nonterminal::Items((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
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
    }
    mod __parse_table {

        mod __parse__Items {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use util::tok::Tok;
            extern crate lalrpop_util as __lalrpop_util;
            use self::__lalrpop_util::ParseError as __ParseError;
            #[allow(dead_code)]
            pub enum __Symbol<'input> {
                Term_22_2b_22(&'input str),
                Term_22_2d_22(&'input str),
                Nt_40L(usize),
                Nt_40R(usize),
                NtItems(Vec<(usize, usize)>),
                NtSpanned_3c_22_2b_22_3e((usize, usize)),
                Nt____Items(Vec<(usize, usize)>),
            }
            const __ACTION: &'static [i32] = &[
                // State 0
                //     Items = (*) ["+"]
                //     Items = (*) ["-"]
                //     Items = (*) [EOF]
                //     Items = (*) Items Spanned<"+"> ["+"]
                //     Items = (*) Items Spanned<"+"> ["-"]
                //     Items = (*) Items Spanned<"+"> [EOF]
                //     Items = (*) Items "-" ["+"]
                //     Items = (*) Items "-" ["-"]
                //     Items = (*) Items "-" [EOF]
                //     __Items = (*) Items [EOF]
                -3, // on "+", reduce `Items =  => ActionFn(9);`
                -3, // on "-", reduce `Items =  => ActionFn(9);`
                // State 1
                //     Items = Items (*) Spanned<"+"> ["+", "-", EOF]
                //     Items = Items (*) "-" ["+", "-", EOF]
                //     Spanned<"+"> = (*) "+" ["+", "-", EOF]
                //     __Items = Items (*) [EOF]
                4, // on "+", goto 3
                5, // on "-", goto 4
                // State 2
                //     Items = Items Spanned<"+"> (*) ["+", "-", EOF]
                -4, // on "+", reduce `Items = Items, Spanned<"+"> => ActionFn(2);`
                -4, // on "-", reduce `Items = Items, Spanned<"+"> => ActionFn(2);`
                // State 3
                //     Spanned<"+"> = "+" (*) ["+", "-", EOF]
                -6, // on "+", reduce `Spanned<"+"> = "+" => ActionFn(10);`
                -6, // on "-", reduce `Spanned<"+"> = "+" => ActionFn(10);`
                // State 4
                //     Items = Items "-" (*) ["+", "-", EOF]
                -5, // on "+", reduce `Items = Items, "-" => ActionFn(3);`
                -5, // on "-", reduce `Items = Items, "-" => ActionFn(3);`
            ];
            const __EOF_ACTION: &'static [i32] = &[
                -3, // on EOF, reduce `Items =  => ActionFn(9);`
                -7, // on EOF, reduce `__Items = Items => ActionFn(0);`
                -4, // on EOF, reduce `Items = Items, Spanned<"+"> => ActionFn(2);`
                -6, // on EOF, reduce `Spanned<"+"> = "+" => ActionFn(10);`
                -5, // on EOF, reduce `Items = Items, "-" => ActionFn(3);`
            ];
            const __GOTO: &'static [i32] = &[
                // State 0
                0, // on @L, error
                0, // on @R, error
                2, // on Items, goto 1
                0, // on Spanned<"+">, error
                0, // on __Items, error
                // State 1
                0, // on @L, error
                0, // on @R, error
                0, // on Items, error
                3, // on Spanned<"+">, goto 2
                0, // on __Items, error
                // State 2
                0, // on @L, error
                0, // on @R, error
                0, // on Items, error
                0, // on Spanned<"+">, error
                0, // on __Items, error
                // State 3
                0, // on @L, error
                0, // on @R, error
                0, // on Items, error
                0, // on Spanned<"+">, error
                0, // on __Items, error
                // State 4
                0, // on @L, error
                0, // on @R, error
                0, // on Items, error
                0, // on Spanned<"+">, error
                0, // on __Items, error
            ];
            pub fn parse_Items<
                'input,
            >(
                input: &'input str,
            ) -> Result<Vec<(usize, usize)>, __ParseError<usize,(usize, &'input str),()>>
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
                            return Err(__ParseError::UnrecognizedToken {
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
                                    (0, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                                    _ => unreachable!(),
                                },
                                1 => match __lookahead.1 {
                                    (1, __tok0) => __Symbol::Term_22_2d_22(__tok0),
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
                            return Err(__ParseError::UnrecognizedToken {
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
                        return Err(__ParseError::UnrecognizedToken {
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
            ) -> Option<Result<Vec<(usize, usize)>,__ParseError<usize,(usize, &'input str),()>>>
            {
                let __nonterminal = match -__action {
                    1 => {
                        // @L =  => ActionFn(6);
                        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action6(input, &__start, &__end);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 0);
                        __symbols.push((__start, __Symbol::Nt_40L(__nt), __end));
                        0
                    }
                    2 => {
                        // @R =  => ActionFn(5);
                        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action5(input, &__start, &__end);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 0);
                        __symbols.push((__start, __Symbol::Nt_40R(__nt), __end));
                        1
                    }
                    3 => {
                        // Items =  => ActionFn(9);
                        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action9(input, &__start, &__end);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 0);
                        __symbols.push((__start, __Symbol::NtItems(__nt), __end));
                        2
                    }
                    4 => {
                        // Items = Items, Spanned<"+"> => ActionFn(2);
                        let __sym1 = __pop_NtSpanned_3c_22_2b_22_3e(__symbols);
                        let __sym0 = __pop_NtItems(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action2(input, __sym0, __sym1);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 2);
                        __symbols.push((__start, __Symbol::NtItems(__nt), __end));
                        2
                    }
                    5 => {
                        // Items = Items, "-" => ActionFn(3);
                        let __sym1 = __pop_Term_22_2d_22(__symbols);
                        let __sym0 = __pop_NtItems(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action3(input, __sym0, __sym1);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 2);
                        __symbols.push((__start, __Symbol::NtItems(__nt), __end));
                        2
                    }
                    6 => {
                        // Spanned<"+"> = "+" => ActionFn(10);
                        let __sym0 = __pop_Term_22_2b_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action10(input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtSpanned_3c_22_2b_22_3e(__nt), __end));
                        3
                    }
                    7 => {
                        // __Items = Items => ActionFn(0);
                        let __sym0 = __pop_NtItems(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0(input, __sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 5 + __nonterminal] - 1;
                __states.push(__next_state);
                None
            }
            fn __pop_Term_22_2b_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2d_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt_40L<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, usize, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt_40L(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt_40R<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, usize, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt_40R(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtItems<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, Vec<(usize, usize)>, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtItems(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtSpanned_3c_22_2b_22_3e<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, (usize, usize), usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtSpanned_3c_22_2b_22_3e(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____Items<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, Vec<(usize, usize)>, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt____Items(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
        }
        pub use self::__parse__Items::parse_Items;
    }
}
pub use self::__parse__Items::parse_Items;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
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
                        43 => /* '+' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        45 => /* '-' */ {
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
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

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
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<(usize, usize)>, usize),
) -> Vec<(usize, usize)>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, usize, usize),
) -> Vec<(usize, usize)>
{
    vec![(__0, __1)]
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, Vec<(usize, usize)>, usize),
    (_, e, _): (usize, (usize, usize), usize),
) -> Vec<(usize, usize)>
{
    {
        let mut v = v;
        v.push(e);
        v
    }
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, Vec<(usize, usize)>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<(usize, usize)>
{
    v
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, usize, usize),
) -> (usize, usize)
{
    (__0, __1)
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: (usize, usize, usize),
) -> Vec<(usize, usize)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action6(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
        __0,
    )
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> (usize, usize)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action6(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __temp0,
        __0,
        __1,
    )
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(usize, usize)>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action5(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __temp0,
    )
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> (usize, usize)
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action5(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __0,
        __temp0,
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
