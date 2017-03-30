extern crate lalrpop_util as __lalrpop_util;

mod __parse__Query {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    extern crate lalrpop_util as __lalrpop_util;
    pub fn parse_Query<
        'input,
    >(
        input: &'input str,
    ) -> Result<String, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let __ascent = __ascent::parse_Query(
            input,
        );
        let __parse_table = __parse_table::parse_Query(
            input,
        );
        assert_eq!(__ascent, __parse_table);
        return __ascent;
    }
    mod __ascent {

        mod __parse__Query {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            extern crate lalrpop_util as __lalrpop_util;
            pub fn parse_Query<
                'input,
            >(
                input: &'input str,
            ) -> Result<String, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
            {
                let mut __tokens = super::super::super::__intern_token::__Matcher::new(input);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match try!(__state0(input, &mut __tokens, __lookahead, ::std::marker::PhantomData::<()>)) {
                    (Some(__lookahead), _) => {
                        Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead })
                    }
                    (None, __Nonterminal::____Query((_, __nt, _))) => {
                        Ok(__nt)
                    }
                    _ => unreachable!(),
                }
            }

            #[allow(dead_code)]
            pub enum __Nonterminal<> {
                Keyword((usize, String, usize)),
                Query((usize, String, usize)),
                Table((usize, String, usize)),
                ____Query((usize, String, usize)),
            }

            // State 0
            //     AllInputs = []
            //     OptionalInputs = []
            //     FixedInputs = []
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Keyword = (*) "INSERT" [r#"(?i)[a-z]+"#]
            //     Keyword = (*) r#"(?i)select"# [r#"(?i)[a-z]+"#]
            //     Keyword = (*) UPDATE [r#"(?i)[a-z]+"#]
            //     Query = (*) Keyword Table [EOF]
            //     __Query = (*) Query [EOF]
            //
            //   "INSERT" -> S3
            //   r#"(?i)select"# -> S4
            //   UPDATE -> S5
            //
            //     Keyword -> S1
            //     Query -> S2
            pub fn __state0<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state3(input, __tokens, __sym0, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state4(input, __tokens, __sym0, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, (3, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(input, __tokens, __sym0, ::std::marker::PhantomData::<()>));
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###""INSERT""###.to_string(),
                                r###"r#"(?i)select"#"###.to_string(),
                                r###"UPDATE"###.to_string(),
                            ]
                        });
                    }
                }
                loop {
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::Keyword(__sym0) => {
                            __result = try!(__state1(input, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Query(__sym0) => {
                            __result = try!(__state2(input, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 1
            //     AllInputs = [Keyword]
            //     OptionalInputs = []
            //     FixedInputs = [Keyword]
            //     WillPushLen = 1
            //     WillPush = [Table]
            //     WillProduce = Some(Query)
            //
            //     Query = Keyword (*) Table [EOF]
            //     Table = (*) r#"(?i)[a-z]+"# [EOF]
            //
            //   r#"(?i)[a-z]+"# -> S7
            //
            //     Table -> S6
            pub fn __state1<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, String, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state7(input, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###"r#"(?i)[a-z]+"#"###.to_string(),
                            ]
                        });
                    }
                }
                loop {
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::Table(__sym1) => {
                            __result = try!(__state6(input, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 2
            //     AllInputs = [Query]
            //     OptionalInputs = []
            //     FixedInputs = [Query]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(__Query)
            //
            //     __Query = Query (*) [EOF]
            //
            //   [EOF] -> __Query = Query => ActionFn(0);
            //
            pub fn __state2<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, String, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<>(input, __sym0);
                        let __nt = __Nonterminal::____Query((
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
                            expected: vec![
                            ]
                        });
                    }
                }
            }

            // State 3
            //     AllInputs = ["INSERT"]
            //     OptionalInputs = []
            //     FixedInputs = ["INSERT"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Keyword)
            //
            //     Keyword = "INSERT" (*) [r#"(?i)[a-z]+"#]
            //
            //   [r#"(?i)[a-z]+"#] -> Keyword = "INSERT" => ActionFn(2);
            //
            pub fn __state3<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (0, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action2::<>(input, __sym0);
                        let __nt = __Nonterminal::Keyword((
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
                            expected: vec![
                                r###"r#"(?i)[a-z]+"#"###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 4
            //     AllInputs = [r#"(?i)select"#]
            //     OptionalInputs = []
            //     FixedInputs = [r#"(?i)select"#]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Keyword)
            //
            //     Keyword = r#"(?i)select"# (*) [r#"(?i)[a-z]+"#]
            //
            //   [r#"(?i)[a-z]+"#] -> Keyword = r#"(?i)select"# => ActionFn(1);
            //
            pub fn __state4<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (0, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action1::<>(input, __sym0);
                        let __nt = __Nonterminal::Keyword((
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
                            expected: vec![
                                r###"r#"(?i)[a-z]+"#"###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 5
            //     AllInputs = [UPDATE]
            //     OptionalInputs = []
            //     FixedInputs = [UPDATE]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Keyword)
            //
            //     Keyword = UPDATE (*) [r#"(?i)[a-z]+"#]
            //
            //   [r#"(?i)[a-z]+"#] -> Keyword = UPDATE => ActionFn(3);
            //
            pub fn __state5<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (0, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<>(input, __sym0);
                        let __nt = __Nonterminal::Keyword((
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
                            expected: vec![
                                r###"r#"(?i)[a-z]+"#"###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 6
            //     AllInputs = [Keyword, Table]
            //     OptionalInputs = []
            //     FixedInputs = [Keyword, Table]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Query)
            //
            //     Query = Keyword Table (*) [EOF]
            //
            //   [EOF] -> Query = Keyword, Table => ActionFn(5);
            //
            pub fn __state6<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, String, usize),
                __sym1: (usize, String, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action5::<>(input, __sym0, __sym1);
                        let __nt = __Nonterminal::Query((
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
                            expected: vec![
                            ]
                        });
                    }
                }
            }

            // State 7
            //     AllInputs = [r#"(?i)[a-z]+"#]
            //     OptionalInputs = []
            //     FixedInputs = [r#"(?i)[a-z]+"#]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Table)
            //
            //     Table = r#"(?i)[a-z]+"# (*) [EOF]
            //
            //   [EOF] -> Table = r#"(?i)[a-z]+"# => ActionFn(4);
            //
            pub fn __state7<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
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
                        let __nt = super::super::super::__action4::<>(input, __sym0);
                        let __nt = __Nonterminal::Table((
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
                            expected: vec![
                            ]
                        });
                    }
                }
            }
        }
        pub use self::__parse__Query::parse_Query;
    }
    mod __parse_table {

        mod __parse__Query {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            extern crate lalrpop_util as __lalrpop_util;
            #[allow(dead_code)]
            pub enum __Symbol<'input> {
                Term_22INSERT_22(&'input str),
                Termr_23_22_28_3fi_29_5ba_2dz_5d_2b_22_23(&'input str),
                Termr_23_22_28_3fi_29select_22_23(&'input str),
                TermUPDATE(&'input str),
                Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
                NtKeyword(String),
                NtQuery(String),
                NtTable(String),
                Nt____Query(String),
            }
            const __ACTION: &'static [i32] = &[
                // State 0
                //     Keyword = (*) "INSERT" [r#"(?i)[a-z]+"#]
                //     Keyword = (*) r#"(?i)select"# [r#"(?i)[a-z]+"#]
                //     Keyword = (*) UPDATE [r#"(?i)[a-z]+"#]
                //     Query = (*) Keyword Table [EOF]
                //     __Query = (*) Query [EOF]
                4,  // on "INSERT", goto 3
                0,  // on r#"(?i)[a-z]+"#, error
                5,  // on r#"(?i)select"#, goto 4
                6,  // on UPDATE, goto 5
                0,  // on error, error

                // State 1
                //     Query = Keyword (*) Table [EOF]
                //     Table = (*) r#"(?i)[a-z]+"# [EOF]
                0,  // on "INSERT", error
                8,  // on r#"(?i)[a-z]+"#, goto 7
                0,  // on r#"(?i)select"#, error
                0,  // on UPDATE, error
                0,  // on error, error

                // State 2
                //     __Query = Query (*) [EOF]
                0,  // on "INSERT", error
                0,  // on r#"(?i)[a-z]+"#, error
                0,  // on r#"(?i)select"#, error
                0,  // on UPDATE, error
                0,  // on error, error

                // State 3
                //     Keyword = "INSERT" (*) [r#"(?i)[a-z]+"#]
                0,  // on "INSERT", error
                -2,  // on r#"(?i)[a-z]+"#, reduce `Keyword = "INSERT" => ActionFn(2);`
                0,  // on r#"(?i)select"#, error
                0,  // on UPDATE, error
                0,  // on error, error

                // State 4
                //     Keyword = r#"(?i)select"# (*) [r#"(?i)[a-z]+"#]
                0,  // on "INSERT", error
                -1,  // on r#"(?i)[a-z]+"#, reduce `Keyword = r#"(?i)select"# => ActionFn(1);`
                0,  // on r#"(?i)select"#, error
                0,  // on UPDATE, error
                0,  // on error, error

                // State 5
                //     Keyword = UPDATE (*) [r#"(?i)[a-z]+"#]
                0,  // on "INSERT", error
                -3,  // on r#"(?i)[a-z]+"#, reduce `Keyword = UPDATE => ActionFn(3);`
                0,  // on r#"(?i)select"#, error
                0,  // on UPDATE, error
                0,  // on error, error

                // State 6
                //     Query = Keyword Table (*) [EOF]
                0,  // on "INSERT", error
                0,  // on r#"(?i)[a-z]+"#, error
                0,  // on r#"(?i)select"#, error
                0,  // on UPDATE, error
                0,  // on error, error

                // State 7
                //     Table = r#"(?i)[a-z]+"# (*) [EOF]
                0,  // on "INSERT", error
                0,  // on r#"(?i)[a-z]+"#, error
                0,  // on r#"(?i)select"#, error
                0,  // on UPDATE, error
                0,  // on error, error

            ];
            const __EOF_ACTION: &'static [i32] = &[
                0,  // on EOF, error

                0,  // on EOF, error

                -6,  // on EOF, reduce `__Query = Query => ActionFn(0);`

                0,  // on EOF, error

                0,  // on EOF, error

                0,  // on EOF, error

                -4,  // on EOF, reduce `Query = Keyword, Table => ActionFn(5);`

                -5,  // on EOF, reduce `Table = r#"(?i)[a-z]+"# => ActionFn(4);`

            ];
            const __GOTO: &'static [i32] = &[
                // State 0
                2,  // on Keyword, goto 1
                3,  // on Query, goto 2
                0,  // on Table, error
                0,  // on __Query, error

                // State 1
                0,  // on Keyword, error
                0,  // on Query, error
                7,  // on Table, goto 6
                0,  // on __Query, error

                // State 2
                0,  // on Keyword, error
                0,  // on Query, error
                0,  // on Table, error
                0,  // on __Query, error

                // State 3
                0,  // on Keyword, error
                0,  // on Query, error
                0,  // on Table, error
                0,  // on __Query, error

                // State 4
                0,  // on Keyword, error
                0,  // on Query, error
                0,  // on Table, error
                0,  // on __Query, error

                // State 5
                0,  // on Keyword, error
                0,  // on Query, error
                0,  // on Table, error
                0,  // on __Query, error

                // State 6
                0,  // on Keyword, error
                0,  // on Query, error
                0,  // on Table, error
                0,  // on __Query, error

                // State 7
                0,  // on Keyword, error
                0,  // on Query, error
                0,  // on Table, error
                0,  // on __Query, error

            ];
            fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
                const __TERMINAL: &'static [&'static str] = &[
                    r###""INSERT""###,
                    r###"r#"(?i)[a-z]+"#"###,
                    r###"r#"(?i)select"#"###,
                    r###"UPDATE"###,
                ];
                __ACTION[(__state * 5)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
                    if state == 0 {
                        None
                    } else {
                        Some(terminal.to_string())
                    }
                }).collect()
            }
            pub fn parse_Query<
                'input,
            >(
                input: &'input str,
            ) -> Result<String, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
            {
                let mut __tokens = super::super::super::__intern_token::__Matcher::new(input);
                let mut __states = vec![0_i32];
                let mut __symbols = vec![];
                let mut __integer;
                let mut __lookahead;
                let mut __last_location = Default::default();
                '__shift: loop {
                    __lookahead = match __tokens.next() {
                        Some(Ok(v)) => v,
                        None => break '__shift,
                        Some(Err(e)) => return Err(e),
                    };
                    __last_location = __lookahead.2.clone();
                    __integer = match __lookahead.1 {
                        (1, _) if true => 0,
                        (0, _) if true => 1,
                        (2, _) if true => 2,
                        (3, _) if true => 3,
                        _ => {
                            let __state = *__states.last().unwrap() as usize;
                            let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: __expected_tokens(__state),
                            };
                            return Err(__error);
                        }
                    };
                    '__inner: loop {
                        let __state = *__states.last().unwrap() as usize;
                        let __action = __ACTION[__state * 5 + __integer];
                        if __action > 0 {
                            let __symbol = match __integer {
                                0 => match __lookahead.1 {
                                    (1, __tok0) => __Symbol::Term_22INSERT_22((__tok0)),
                                    _ => unreachable!(),
                                },
                                1 => match __lookahead.1 {
                                    (0, __tok0) => __Symbol::Termr_23_22_28_3fi_29_5ba_2dz_5d_2b_22_23((__tok0)),
                                    _ => unreachable!(),
                                },
                                2 => match __lookahead.1 {
                                    (2, __tok0) => __Symbol::Termr_23_22_28_3fi_29select_22_23((__tok0)),
                                    _ => unreachable!(),
                                },
                                3 => match __lookahead.1 {
                                    (3, __tok0) => __Symbol::TermUPDATE((__tok0)),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            __states.push(__action - 1);
                            __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                            continue '__shift;
                        } else if __action < 0 {
                            if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                                return r;
                            }
                        } else {
                            let __state = *__states.last().unwrap() as usize;
                            let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: __expected_tokens(__state),
                            };
                            return Err(__error)
                        }
                    }
                }
                loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __EOF_ACTION[__state];
                    if __action < 0 {
                        if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            return r;
                        }
                    } else {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: None,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
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
                _: ::std::marker::PhantomData<()>,
            ) -> Option<Result<String,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
            {
                let __nonterminal = match -__action {
                    1 => {
                        // Keyword = r#"(?i)select"# => ActionFn(1);
                        let __sym0 = __pop_Termr_23_22_28_3fi_29select_22_23(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action1::<>(input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtKeyword(__nt), __end));
                        0
                    }
                    2 => {
                        // Keyword = "INSERT" => ActionFn(2);
                        let __sym0 = __pop_Term_22INSERT_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action2::<>(input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtKeyword(__nt), __end));
                        0
                    }
                    3 => {
                        // Keyword = UPDATE => ActionFn(3);
                        let __sym0 = __pop_TermUPDATE(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<>(input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtKeyword(__nt), __end));
                        0
                    }
                    4 => {
                        // Query = Keyword, Table => ActionFn(5);
                        let __sym1 = __pop_NtTable(__symbols);
                        let __sym0 = __pop_NtKeyword(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action5::<>(input, __sym0, __sym1);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 2);
                        __symbols.push((__start, __Symbol::NtQuery(__nt), __end));
                        1
                    }
                    5 => {
                        // Table = r#"(?i)[a-z]+"# => ActionFn(4);
                        let __sym0 = __pop_Termr_23_22_28_3fi_29_5ba_2dz_5d_2b_22_23(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action4::<>(input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtTable(__nt), __end));
                        2
                    }
                    6 => {
                        // __Query = Query => ActionFn(0);
                        let __sym0 = __pop_NtQuery(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<>(input, __sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 4 + __nonterminal] - 1;
                __states.push(__next_state);
                None
            }
            fn __pop_Term_22INSERT_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22INSERT_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Termr_23_22_28_3fi_29_5ba_2dz_5d_2b_22_23<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Termr_23_22_28_3fi_29_5ba_2dz_5d_2b_22_23(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Termr_23_22_28_3fi_29select_22_23<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Termr_23_22_28_3fi_29select_22_23(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_TermUPDATE<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::TermUPDATE(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Termerror<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtKeyword<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, String, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtKeyword(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtQuery<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, String, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtQuery(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtTable<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, String, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtTable(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____Query<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, String, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt____Query(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
        }
        pub use self::__parse__Query::parse_Query;
    }
}
pub use self::__parse__Query::parse_Query;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^[A-Za-zſ-ſK-K]+",
                "^(?i:insert)",
                "^(?i:select)",
                "^(?i:update)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^[A-Za-zſ-ſK-K]+").unwrap(),
                __regex::Regex::new("^(?i:insert)").unwrap(),
                __regex::Regex::new("^(?i:select)").unwrap(),
                __regex::Regex::new("^(?i:update)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
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
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 4 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
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
    String::from("SELECT")
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from("INSERT")
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from("UPDATE")
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
    (_, __1, _): (usize, String, usize),
) -> String
{
    format!("{} {}", __0, __1)
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
