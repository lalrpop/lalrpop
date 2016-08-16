use lalrpop_util::ParseError;
use generics_issue_104_lib::Generator;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Schema {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use lalrpop_util::ParseError;
    use generics_issue_104_lib::Generator;
    extern crate lalrpop_util as __lalrpop_util;
    pub fn parse_Schema<
        'input,
        T,
    >(
        input: &'input str,
    ) -> Result<String, __lalrpop_util::ParseError<usize,(usize, &'input str),String>> where
      T: Generator,
    {
        let __ascent = __ascent::parse_Schema::<T>(
            input,
        );
        let __parse_table = __parse_table::parse_Schema::<T>(
            input,
        );
        assert_eq!(__ascent, __parse_table);
        return __ascent;
    }
    mod __ascent {

        mod __parse__Schema {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use lalrpop_util::ParseError;
            use generics_issue_104_lib::Generator;
            extern crate lalrpop_util as __lalrpop_util;
            pub fn parse_Schema<
                'input,
                T,
            >(
                input: &'input str,
            ) -> Result<String, __lalrpop_util::ParseError<usize,(usize, &'input str),String>> where
              T: Generator,
            {
                let mut __tokens = super::super::super::__intern_token::__Matcher::new(input);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match try!(__state0(input, &mut __tokens, __lookahead, ::std::marker::PhantomData::<(T)>)) {
                    (Some(__lookahead), _) => {
                        Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead })
                    }
                    (None, __Nonterminal::____Schema((_, __nt, _))) => {
                        Ok(__nt)
                    }
                    _ => unreachable!(),
                }
            }

            #[allow(dead_code)]
            pub enum __Nonterminal<> {
                Schema((usize, String, usize)),
                ____Schema((usize, String, usize)),
            }

            // State 0
            //     AllInputs = []
            //     OptionalInputs = []
            //     FixedInputs = []
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Schema = (*) "grammar" "{" r#"[a-zA-Z0-9]*"# "}" [EOF]
            //     __Schema = (*) Schema [EOF]
            //
            //   "grammar" -> S2
            //
            //     Schema -> S1
            pub fn __state0<
                'input,
                T,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),String>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                _: ::std::marker::PhantomData<(T)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),String>> where
              T: Generator,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state2(input, __tokens, __sym0, ::std::marker::PhantomData::<(T)>));
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
                        __Nonterminal::Schema(__sym0) => {
                            __result = try!(__state1(input, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<(T)>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 1
            //     AllInputs = [Schema]
            //     OptionalInputs = []
            //     FixedInputs = [Schema]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(__Schema)
            //
            //     __Schema = Schema (*) [EOF]
            //
            //   [EOF] -> __Schema = Schema => ActionFn(0);
            //
            pub fn __state1<
                'input,
                T,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),String>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, String, usize),
                _: ::std::marker::PhantomData<(T)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),String>> where
              T: Generator,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<T>(input, __sym0);
                        let __nt = __Nonterminal::____Schema((
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
            //     AllInputs = ["grammar"]
            //     OptionalInputs = []
            //     FixedInputs = ["grammar"]
            //     WillPushLen = 3
            //     WillPush = ["{", r#"[a-zA-Z0-9]*"#, "}"]
            //     WillProduce = Some(Schema)
            //
            //     Schema = "grammar" (*) "{" r#"[a-zA-Z0-9]*"# "}" [EOF]
            //
            //   "{" -> S3
            //
            pub fn __state2<
                'input,
                T,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),String>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(T)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),String>> where
              T: Generator,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state3(input, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<(T)>));
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
            //     AllInputs = ["grammar", "{"]
            //     OptionalInputs = []
            //     FixedInputs = ["grammar", "{"]
            //     WillPushLen = 2
            //     WillPush = [r#"[a-zA-Z0-9]*"#, "}"]
            //     WillProduce = Some(Schema)
            //
            //     Schema = "grammar" "{" (*) r#"[a-zA-Z0-9]*"# "}" [EOF]
            //
            //   r#"[a-zA-Z0-9]*"# -> S4
            //
            pub fn __state3<
                'input,
                T,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),String>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(T)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),String>> where
              T: Generator,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (3, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state4(input, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(T)>));
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
            //     AllInputs = ["grammar", "{", r#"[a-zA-Z0-9]*"#]
            //     OptionalInputs = []
            //     FixedInputs = ["grammar", "{", r#"[a-zA-Z0-9]*"#]
            //     WillPushLen = 1
            //     WillPush = ["}"]
            //     WillProduce = Some(Schema)
            //
            //     Schema = "grammar" "{" r#"[a-zA-Z0-9]*"# (*) "}" [EOF]
            //
            //   "}" -> S5
            //
            pub fn __state4<
                'input,
                T,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),String>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(T)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),String>> where
              T: Generator,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(input, __tokens, __sym0, __sym1, __sym2, __sym3, ::std::marker::PhantomData::<(T)>));
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
            //     AllInputs = ["grammar", "{", r#"[a-zA-Z0-9]*"#, "}"]
            //     OptionalInputs = []
            //     FixedInputs = ["grammar", "{", r#"[a-zA-Z0-9]*"#, "}"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Schema)
            //
            //     Schema = "grammar" "{" r#"[a-zA-Z0-9]*"# "}" (*) [EOF]
            //
            //   [EOF] -> Schema = "grammar", "{", r#"[a-zA-Z0-9]*"#, "}" => ActionFn(1);
            //
            pub fn __state5<
                'input,
                T,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),String>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, &'input str, usize),
                __sym3: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(T)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize,(usize, &'input str),String>> where
              T: Generator,
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
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action1::<T>(input, __sym0, __sym1, __sym2, __sym3);
                        let __nt = __Nonterminal::Schema((
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
        pub use self::__parse__Schema::parse_Schema;
    }
    mod __parse_table {

        mod __parse__Schema {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use lalrpop_util::ParseError;
            use generics_issue_104_lib::Generator;
            extern crate lalrpop_util as __lalrpop_util;
            #[allow(dead_code)]
            pub enum __Symbol<'input> {
                Term_22grammar_22(&'input str),
                Term_22_7b_22(&'input str),
                Term_22_7d_22(&'input str),
                Termr_23_22_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(&'input str),
                NtSchema(String),
                Nt____Schema(String),
            }
            const __ACTION: &'static [i32] = &[
                // State 0
                //     Schema = (*) "grammar" "{" r#"[a-zA-Z0-9]*"# "}" [EOF]
                //     __Schema = (*) Schema [EOF]
                3, // on "grammar", goto 2
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z0-9]*"#, error
                // State 1
                //     __Schema = Schema (*) [EOF]
                0, // on "grammar", error
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z0-9]*"#, error
                // State 2
                //     Schema = "grammar" (*) "{" r#"[a-zA-Z0-9]*"# "}" [EOF]
                0, // on "grammar", error
                4, // on "{", goto 3
                0, // on "}", error
                0, // on r#"[a-zA-Z0-9]*"#, error
                // State 3
                //     Schema = "grammar" "{" (*) r#"[a-zA-Z0-9]*"# "}" [EOF]
                0, // on "grammar", error
                0, // on "{", error
                0, // on "}", error
                5, // on r#"[a-zA-Z0-9]*"#, goto 4
                // State 4
                //     Schema = "grammar" "{" r#"[a-zA-Z0-9]*"# (*) "}" [EOF]
                0, // on "grammar", error
                0, // on "{", error
                6, // on "}", goto 5
                0, // on r#"[a-zA-Z0-9]*"#, error
                // State 5
                //     Schema = "grammar" "{" r#"[a-zA-Z0-9]*"# "}" (*) [EOF]
                0, // on "grammar", error
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z0-9]*"#, error
            ];
            const __EOF_ACTION: &'static [i32] = &[
                0, // on EOF, error
                -2, // on EOF, reduce `__Schema = Schema => ActionFn(0);`
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                -1, // on EOF, reduce `Schema = "grammar", "{", r#"[a-zA-Z0-9]*"#, "}" => ActionFn(1);`
            ];
            const __GOTO: &'static [i32] = &[
                // State 0
                2, // on Schema, goto 1
                0, // on __Schema, error
                // State 1
                0, // on Schema, error
                0, // on __Schema, error
                // State 2
                0, // on Schema, error
                0, // on __Schema, error
                // State 3
                0, // on Schema, error
                0, // on __Schema, error
                // State 4
                0, // on Schema, error
                0, // on __Schema, error
                // State 5
                0, // on Schema, error
                0, // on __Schema, error
            ];
            pub fn parse_Schema<
                'input,
                T,
            >(
                input: &'input str,
            ) -> Result<String, __lalrpop_util::ParseError<usize,(usize, &'input str),String>> where
              T: Generator,
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
                        (_, (2, _), _) if true => 2,
                        (_, (3, _), _) if true => 3,
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
                                    (0, __tok0) => __Symbol::Term_22grammar_22(__tok0),
                                    _ => unreachable!(),
                                },
                                1 => match __lookahead.1 {
                                    (1, __tok0) => __Symbol::Term_22_7b_22(__tok0),
                                    _ => unreachable!(),
                                },
                                2 => match __lookahead.1 {
                                    (2, __tok0) => __Symbol::Term_22_7d_22(__tok0),
                                    _ => unreachable!(),
                                },
                                3 => match __lookahead.1 {
                                    (3, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__tok0),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            __states.push(__action - 1);
                            __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                            continue '__shift;
                        } else if __action < 0 {
                            if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<(T)>) {
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
                        if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<(T)>) {
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
                T,
            >(
                input: &'input str,
                __action: i32,
                __lookahead_start: Option<&usize>,
                __states: &mut ::std::vec::Vec<i32>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
                _: ::std::marker::PhantomData<(T)>,
            ) -> Option<Result<String,__lalrpop_util::ParseError<usize,(usize, &'input str),String>>> where
              T: Generator,
            {
                let __nonterminal = match -__action {
                    1 => {
                        // Schema = "grammar", "{", r#"[a-zA-Z0-9]*"#, "}" => ActionFn(1);
                        let __sym3 = __pop_Term_22_7d_22(__symbols);
                        let __sym2 = __pop_Termr_23_22_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__symbols);
                        let __sym1 = __pop_Term_22_7b_22(__symbols);
                        let __sym0 = __pop_Term_22grammar_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action1::<T>(input, __sym0, __sym1, __sym2, __sym3);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 4);
                        __symbols.push((__start, __Symbol::NtSchema(__nt), __end));
                        0
                    }
                    2 => {
                        // __Schema = Schema => ActionFn(0);
                        let __sym0 = __pop_NtSchema(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<T>(input, __sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 2 + __nonterminal] - 1;
                __states.push(__next_state);
                None
            }
            fn __pop_Term_22grammar_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22grammar_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_7b_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_7d_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Termr_23_22_5ba_2dzA_2dZ0_2d9_5d_2a_22_23<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtSchema<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, String, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtSchema(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____Schema<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, String, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt____Schema(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
        }
        pub use self::__parse__Schema::parse_Schema;
    }
}
pub use self::__parse__Schema::parse_Schema;
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
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        97 ... 102 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        103 => /* 'g' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        104 ... 122 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        123 => /* '{' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        125 => /* '}' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 4;
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
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
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
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        97 ... 108 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        110 ... 122 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        97 ... 108 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        110 ... 122 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
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
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),String>>;

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
    T,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String where
  T: Generator,
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
    T,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String where
  T: Generator,
{
    T::schema(id)
}

pub trait __ToTriple<'input, T, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, T, > __ToTriple<'input, T, > for (usize, (usize, &'input str), usize) {
    type Error = String;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),String> {
        Ok(value)
    }
}
impl<'input, T, > __ToTriple<'input, T, > for Result<(usize, (usize, &'input str), usize),String> {
    type Error = String;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),String> {
        value
    }
}
