extern crate lalrpop_util as __lalrpop_util;

mod __parse__E {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    extern crate lalrpop_util as __lalrpop_util;
    pub fn parse_E<
        'input,
    >(
        input: &'input str,
    ) -> Result<(Vec<&'input str>, &'input str, Vec<&'input str>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
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
            ) -> Result<(Vec<&'input str>, &'input str, Vec<&'input str>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
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
                    (None, __Nonterminal::____E((_, __nt, _))) => {
                        Ok(__nt)
                    }
                    _ => unreachable!(),
                }
            }

            #[allow(dead_code)]
            pub enum __Nonterminal<'input> {
                AT((usize, &'input str, usize)),
                AT_2a((usize, ::std::vec::Vec<&'input str>, usize)),
                AT_2b((usize, ::std::vec::Vec<&'input str>, usize)),
                E((usize, (Vec<&'input str>, &'input str, Vec<&'input str>), usize)),
                ET((usize, &'input str, usize)),
                ____E((usize, (Vec<&'input str>, &'input str, Vec<&'input str>), usize)),
            }

            // State 0
            //     AllInputs = []
            //     OptionalInputs = []
            //     FixedInputs = []
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     E = (*) "{" AT+ ET AT+ "}" [EOF]
            //     E = (*) "{" AT+ ET "}" [EOF]
            //     E = (*) "{" ET AT+ "}" [EOF]
            //     E = (*) "{" ET "}" [EOF]
            //     __E = (*) E [EOF]
            //
            //   "{" -> S2
            //
            //     E -> S1
            pub fn __state0<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                match __lookahead {
                    Some((__loc1, (3, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state2(input, __tokens, __sym0, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state1(input, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
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
                __sym0: (usize, (Vec<&'input str>, &'input str, Vec<&'input str>), usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<>(input, __sym0);
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
            //     AllInputs = ["{"]
            //     OptionalInputs = []
            //     FixedInputs = ["{"]
            //     WillPushLen = 2
            //     WillPush = [ET, "}"]
            //     WillProduce = Some(E)
            //
            //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["enum"]
            //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["type"]
            //     AT+ = (*) AT ["enum"]
            //     AT+ = (*) AT ["type"]
            //     AT+ = (*) AT+ AT ["enum"]
            //     AT+ = (*) AT+ AT ["type"]
            //     E = "{" (*) AT+ ET AT+ "}" [EOF]
            //     E = "{" (*) AT+ ET "}" [EOF]
            //     E = "{" (*) ET AT+ "}" [EOF]
            //     E = "{" (*) ET "}" [EOF]
            //     ET = (*) "enum" r#"[a-zA-Z]+"# "{" "}" ["type"]
            //     ET = (*) "enum" r#"[a-zA-Z]+"# "{" "}" ["}"]
            //
            //   "enum" -> S6
            //   "type" -> S7
            //
            //     AT -> S3
            //     AT+ -> S4
            //     ET -> S5
            pub fn __state2<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(input, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state7(input, __tokens, __sym1, ::std::marker::PhantomData::<()>));
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
                        __Nonterminal::AT(__sym1) => {
                            __result = try!(__state3(input, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::AT_2b(__sym1) => {
                            __result = try!(__state4(input, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::ET(__sym1) => {
                            let __sym0 = __sym0.take().unwrap();
                            __result = try!(__state5(input, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 3
            //     AllInputs = [AT]
            //     OptionalInputs = []
            //     FixedInputs = [AT]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(AT+)
            //
            //     AT+ = AT (*) ["enum", "type"]
            //
            //   ["enum", "type"] -> AT+ = AT => ActionFn(6);
            //
            pub fn __state3<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action6::<>(input, __sym0);
                        let __nt = __Nonterminal::AT_2b((
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
            //     AllInputs = ["{", AT+]
            //     OptionalInputs = ["{"]
            //     FixedInputs = [AT+]
            //     WillPushLen = 1
            //     WillPush = [AT]
            //     WillProduce = None
            //
            //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["enum", "type"]
            //     AT+ = AT+ (*) AT ["enum", "type"]
            //     E = "{" AT+ (*) ET AT+ "}" [EOF]
            //     E = "{" AT+ (*) ET "}" [EOF]
            //     ET = (*) "enum" r#"[a-zA-Z]+"# "{" "}" ["type"]
            //     ET = (*) "enum" r#"[a-zA-Z]+"# "{" "}" ["}"]
            //
            //   "enum" -> S6
            //   "type" -> S7
            //
            //     AT -> S8
            //     ET -> S9
            pub fn __state4<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, &'input str, usize)>,
                __sym1: (usize, ::std::vec::Vec<&'input str>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                match __lookahead {
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(input, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state7(input, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                        __Nonterminal::AT(__sym2) => {
                            __result = try!(__state8(input, __tokens, __lookahead, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        __Nonterminal::ET(__sym2) => {
                            let __sym0 = __sym0.take().unwrap();
                            __result = try!(__state9(input, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 5
            //     AllInputs = ["{", ET]
            //     OptionalInputs = []
            //     FixedInputs = ["{", ET]
            //     WillPushLen = 1
            //     WillPush = ["}"]
            //     WillProduce = Some(E)
            //
            //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["type"]
            //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["}"]
            //     AT+ = (*) AT ["type"]
            //     AT+ = (*) AT ["}"]
            //     AT+ = (*) AT+ AT ["type"]
            //     AT+ = (*) AT+ AT ["}"]
            //     E = "{" ET (*) AT+ "}" [EOF]
            //     E = "{" ET (*) "}" [EOF]
            //
            //   "type" -> S12
            //   "}" -> S13
            //
            //     AT -> S10
            //     AT+ -> S11
            pub fn __state5<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __sym0 = &mut Some(__sym0);
                let __sym1 = &mut Some(__sym1);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state12(input, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, (4, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        __result = try!(__state13(input, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
                    if __sym1.is_none() {
                        return Ok(__result);
                    }
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::AT(__sym2) => {
                            __result = try!(__state10(input, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::AT_2b(__sym2) => {
                            __result = try!(__state11(input, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 6
            //     AllInputs = ["enum"]
            //     OptionalInputs = []
            //     FixedInputs = ["enum"]
            //     WillPushLen = 3
            //     WillPush = [r#"[a-zA-Z]+"#, "{", "}"]
            //     WillProduce = Some(ET)
            //
            //     ET = "enum" (*) r#"[a-zA-Z]+"# "{" "}" ["type", "}"]
            //
            //   r#"[a-zA-Z]+"# -> S14
            //
            pub fn __state6<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(input, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
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

            // State 7
            //     AllInputs = ["type"]
            //     OptionalInputs = []
            //     FixedInputs = ["type"]
            //     WillPushLen = 2
            //     WillPush = [r#"[a-zA-Z]+"#, ";"]
            //     WillProduce = Some(AT)
            //
            //     AT = "type" (*) r#"[a-zA-Z]+"# ";" ["enum", "type"]
            //
            //   r#"[a-zA-Z]+"# -> S15
            //
            pub fn __state7<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state15(input, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
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
            //     AllInputs = [AT+, AT]
            //     OptionalInputs = []
            //     FixedInputs = [AT+, AT]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(AT+)
            //
            //     AT+ = AT+ AT (*) ["enum", "type"]
            //
            //   ["enum", "type"] -> AT+ = AT+, AT => ActionFn(7);
            //
            pub fn __state8<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, ::std::vec::Vec<&'input str>, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action7::<>(input, __sym0, __sym1);
                        let __nt = __Nonterminal::AT_2b((
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
            //     AllInputs = ["{", AT+, ET]
            //     OptionalInputs = []
            //     FixedInputs = ["{", AT+, ET]
            //     WillPushLen = 1
            //     WillPush = ["}"]
            //     WillProduce = Some(E)
            //
            //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["type"]
            //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["}"]
            //     AT+ = (*) AT ["type"]
            //     AT+ = (*) AT ["}"]
            //     AT+ = (*) AT+ AT ["type"]
            //     AT+ = (*) AT+ AT ["}"]
            //     E = "{" AT+ ET (*) AT+ "}" [EOF]
            //     E = "{" AT+ ET (*) "}" [EOF]
            //
            //   "type" -> S12
            //   "}" -> S17
            //
            //     AT -> S10
            //     AT+ -> S16
            pub fn __state9<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, ::std::vec::Vec<&'input str>, usize),
                __sym2: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __sym0 = &mut Some(__sym0);
                let __sym1 = &mut Some(__sym1);
                let __sym2 = &mut Some(__sym2);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state12(input, __tokens, __sym3, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, (4, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __sym2 = __sym2.take().unwrap();
                        __result = try!(__state17(input, __tokens, __sym0, __sym1, __sym2, __sym3, ::std::marker::PhantomData::<()>));
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
                    if __sym2.is_none() {
                        return Ok(__result);
                    }
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::AT(__sym3) => {
                            __result = try!(__state10(input, __tokens, __lookahead, __sym3, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::AT_2b(__sym3) => {
                            __result = try!(__state16(input, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 10
            //     AllInputs = [AT]
            //     OptionalInputs = []
            //     FixedInputs = [AT]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(AT+)
            //
            //     AT+ = AT (*) ["type", "}"]
            //
            //   ["type", "}"] -> AT+ = AT => ActionFn(6);
            //
            pub fn __state10<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (4, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action6::<>(input, __sym0);
                        let __nt = __Nonterminal::AT_2b((
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
            //     AllInputs = ["{", ET, AT+]
            //     OptionalInputs = ["{", ET]
            //     FixedInputs = [AT+]
            //     WillPushLen = 1
            //     WillPush = [AT]
            //     WillProduce = None
            //
            //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["type", "}"]
            //     AT+ = AT+ (*) AT ["type", "}"]
            //     E = "{" ET AT+ (*) "}" [EOF]
            //
            //   "type" -> S12
            //   "}" -> S19
            //
            //     AT -> S18
            pub fn __state11<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, &'input str, usize)>,
                __sym1: &mut Option<(usize, &'input str, usize)>,
                __sym2: (usize, ::std::vec::Vec<&'input str>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state12(input, __tokens, __sym3, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, (4, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        __result = try!(__state19(input, __tokens, __sym0, __sym1, __sym2, __sym3, ::std::marker::PhantomData::<()>));
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
                        __Nonterminal::AT(__sym3) => {
                            __result = try!(__state18(input, __tokens, __lookahead, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 12
            //     AllInputs = ["type"]
            //     OptionalInputs = []
            //     FixedInputs = ["type"]
            //     WillPushLen = 2
            //     WillPush = [r#"[a-zA-Z]+"#, ";"]
            //     WillProduce = Some(AT)
            //
            //     AT = "type" (*) r#"[a-zA-Z]+"# ";" ["type", "}"]
            //
            //   r#"[a-zA-Z]+"# -> S20
            //
            pub fn __state12<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state20(input, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
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
            //     AllInputs = ["{", ET, "}"]
            //     OptionalInputs = []
            //     FixedInputs = ["{", ET, "}"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = "{" ET "}" (*) [EOF]
            //
            //   [EOF] -> E = "{", ET, "}" => ActionFn(8);
            //
            pub fn __state13<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action8::<>(input, __sym0, __sym1, __sym2);
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

            // State 14
            //     AllInputs = ["enum", r#"[a-zA-Z]+"#]
            //     OptionalInputs = []
            //     FixedInputs = ["enum", r#"[a-zA-Z]+"#]
            //     WillPushLen = 2
            //     WillPush = ["{", "}"]
            //     WillProduce = Some(ET)
            //
            //     ET = "enum" r#"[a-zA-Z]+"# (*) "{" "}" ["type", "}"]
            //
            //   "{" -> S21
            //
            pub fn __state14<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (3, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state21(input, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
            //     AllInputs = ["type", r#"[a-zA-Z]+"#]
            //     OptionalInputs = []
            //     FixedInputs = ["type", r#"[a-zA-Z]+"#]
            //     WillPushLen = 1
            //     WillPush = [";"]
            //     WillProduce = Some(AT)
            //
            //     AT = "type" r#"[a-zA-Z]+"# (*) ";" ["enum", "type"]
            //
            //   ";" -> S22
            //
            pub fn __state15<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state22(input, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
            //     AllInputs = ["{", AT+, ET, AT+]
            //     OptionalInputs = ["{", AT+, ET]
            //     FixedInputs = [AT+]
            //     WillPushLen = 1
            //     WillPush = [AT]
            //     WillProduce = None
            //
            //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["type", "}"]
            //     AT+ = AT+ (*) AT ["type", "}"]
            //     E = "{" AT+ ET AT+ (*) "}" [EOF]
            //
            //   "type" -> S12
            //   "}" -> S23
            //
            //     AT -> S18
            pub fn __state16<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, &'input str, usize)>,
                __sym1: &mut Option<(usize, ::std::vec::Vec<&'input str>, usize)>,
                __sym2: &mut Option<(usize, &'input str, usize)>,
                __sym3: (usize, ::std::vec::Vec<&'input str>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym4 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state12(input, __tokens, __sym4, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, (4, __tok0), __loc2)) => {
                        let __sym4 = (__loc1, (__tok0), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __sym2 = __sym2.take().unwrap();
                        __result = try!(__state23(input, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, ::std::marker::PhantomData::<()>));
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
                        __Nonterminal::AT(__sym4) => {
                            __result = try!(__state18(input, __tokens, __lookahead, __sym3, __sym4, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 17
            //     AllInputs = ["{", AT+, ET, "}"]
            //     OptionalInputs = []
            //     FixedInputs = ["{", AT+, ET, "}"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = "{" AT+ ET "}" (*) [EOF]
            //
            //   [EOF] -> E = "{", AT+, ET, "}" => ActionFn(10);
            //
            pub fn __state17<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, ::std::vec::Vec<&'input str>, usize),
                __sym2: (usize, &'input str, usize),
                __sym3: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3);
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

            // State 18
            //     AllInputs = [AT+, AT]
            //     OptionalInputs = []
            //     FixedInputs = [AT+, AT]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(AT+)
            //
            //     AT+ = AT+ AT (*) ["type", "}"]
            //
            //   ["type", "}"] -> AT+ = AT+, AT => ActionFn(7);
            //
            pub fn __state18<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, ::std::vec::Vec<&'input str>, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (4, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action7::<>(input, __sym0, __sym1);
                        let __nt = __Nonterminal::AT_2b((
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

            // State 19
            //     AllInputs = ["{", ET, AT+, "}"]
            //     OptionalInputs = []
            //     FixedInputs = ["{", ET, AT+, "}"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = "{" ET AT+ "}" (*) [EOF]
            //
            //   [EOF] -> E = "{", ET, AT+, "}" => ActionFn(9);
            //
            pub fn __state19<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, ::std::vec::Vec<&'input str>, usize),
                __sym3: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3);
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

            // State 20
            //     AllInputs = ["type", r#"[a-zA-Z]+"#]
            //     OptionalInputs = []
            //     FixedInputs = ["type", r#"[a-zA-Z]+"#]
            //     WillPushLen = 1
            //     WillPush = [";"]
            //     WillProduce = Some(AT)
            //
            //     AT = "type" r#"[a-zA-Z]+"# (*) ";" ["type", "}"]
            //
            //   ";" -> S24
            //
            pub fn __state20<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state24(input, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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

            // State 21
            //     AllInputs = ["enum", r#"[a-zA-Z]+"#, "{"]
            //     OptionalInputs = []
            //     FixedInputs = ["enum", r#"[a-zA-Z]+"#, "{"]
            //     WillPushLen = 1
            //     WillPush = ["}"]
            //     WillProduce = Some(ET)
            //
            //     ET = "enum" r#"[a-zA-Z]+"# "{" (*) "}" ["type", "}"]
            //
            //   "}" -> S25
            //
            pub fn __state21<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (4, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state25(input, __tokens, __sym0, __sym1, __sym2, __sym3, ::std::marker::PhantomData::<()>));
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

            // State 22
            //     AllInputs = ["type", r#"[a-zA-Z]+"#, ";"]
            //     OptionalInputs = []
            //     FixedInputs = ["type", r#"[a-zA-Z]+"#, ";"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(AT)
            //
            //     AT = "type" r#"[a-zA-Z]+"# ";" (*) ["enum", "type"]
            //
            //   ["enum", "type"] -> AT = "type", r#"[a-zA-Z]+"#, ";" => ActionFn(2);
            //
            pub fn __state22<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::AT((
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

            // State 23
            //     AllInputs = ["{", AT+, ET, AT+, "}"]
            //     OptionalInputs = []
            //     FixedInputs = ["{", AT+, ET, AT+, "}"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(E)
            //
            //     E = "{" AT+ ET AT+ "}" (*) [EOF]
            //
            //   [EOF] -> E = "{", AT+, ET, AT+, "}" => ActionFn(11);
            //
            pub fn __state23<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, ::std::vec::Vec<&'input str>, usize),
                __sym2: (usize, &'input str, usize),
                __sym3: (usize, ::std::vec::Vec<&'input str>, usize),
                __sym4: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym4.2.clone();
                        let __nt = super::super::super::__action11::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
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

            // State 24
            //     AllInputs = ["type", r#"[a-zA-Z]+"#, ";"]
            //     OptionalInputs = []
            //     FixedInputs = ["type", r#"[a-zA-Z]+"#, ";"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(AT)
            //
            //     AT = "type" r#"[a-zA-Z]+"# ";" (*) ["type", "}"]
            //
            //   ["type", "}"] -> AT = "type", r#"[a-zA-Z]+"#, ";" => ActionFn(2);
            //
            pub fn __state24<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (4, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::AT((
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

            // State 25
            //     AllInputs = ["enum", r#"[a-zA-Z]+"#, "{", "}"]
            //     OptionalInputs = []
            //     FixedInputs = ["enum", r#"[a-zA-Z]+"#, "{", "}"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(ET)
            //
            //     ET = "enum" r#"[a-zA-Z]+"# "{" "}" (*) ["type", "}"]
            //
            //   ["type", "}"] -> ET = "enum", r#"[a-zA-Z]+"#, "{", "}" => ActionFn(3);
            //
            pub fn __state25<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, &'input str, usize),
                __sym3: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (4, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action3::<>(input, __sym0, __sym1, __sym2, __sym3);
                        let __nt = __Nonterminal::ET((
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
                Term_22_3b_22(&'input str),
                Term_22enum_22(&'input str),
                Term_22type_22(&'input str),
                Term_22_7b_22(&'input str),
                Term_22_7d_22(&'input str),
                Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(&'input str),
                NtAT(&'input str),
                NtAT_2a(::std::vec::Vec<&'input str>),
                NtAT_2b(::std::vec::Vec<&'input str>),
                NtE((Vec<&'input str>, &'input str, Vec<&'input str>)),
                NtET(&'input str),
                Nt____E((Vec<&'input str>, &'input str, Vec<&'input str>)),
            }
            const __ACTION: &'static [i32] = &[
                // State 0
                //     E = (*) "{" AT+ ET AT+ "}" [EOF]
                //     E = (*) "{" AT+ ET "}" [EOF]
                //     E = (*) "{" ET AT+ "}" [EOF]
                //     E = (*) "{" ET "}" [EOF]
                //     __E = (*) E [EOF]
                0, // on ";", error
                0, // on "enum", error
                0, // on "type", error
                3, // on "{", goto 2
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 1
                //     __E = E (*) [EOF]
                0, // on ";", error
                0, // on "enum", error
                0, // on "type", error
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 2
                //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["enum"]
                //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["type"]
                //     AT+ = (*) AT ["enum"]
                //     AT+ = (*) AT ["type"]
                //     AT+ = (*) AT+ AT ["enum"]
                //     AT+ = (*) AT+ AT ["type"]
                //     E = "{" (*) AT+ ET AT+ "}" [EOF]
                //     E = "{" (*) AT+ ET "}" [EOF]
                //     E = "{" (*) ET AT+ "}" [EOF]
                //     E = "{" (*) ET "}" [EOF]
                //     ET = (*) "enum" r#"[a-zA-Z]+"# "{" "}" ["type"]
                //     ET = (*) "enum" r#"[a-zA-Z]+"# "{" "}" ["}"]
                0, // on ";", error
                7, // on "enum", goto 6
                8, // on "type", goto 7
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 3
                //     AT+ = AT (*) ["enum", "type"]
                0, // on ";", error
                -4, // on "enum", reduce `AT+ = AT => ActionFn(6);`
                -4, // on "type", reduce `AT+ = AT => ActionFn(6);`
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 4
                //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["enum", "type"]
                //     AT+ = AT+ (*) AT ["enum", "type"]
                //     E = "{" AT+ (*) ET AT+ "}" [EOF]
                //     E = "{" AT+ (*) ET "}" [EOF]
                //     ET = (*) "enum" r#"[a-zA-Z]+"# "{" "}" ["type"]
                //     ET = (*) "enum" r#"[a-zA-Z]+"# "{" "}" ["}"]
                0, // on ";", error
                7, // on "enum", goto 6
                8, // on "type", goto 7
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 5
                //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["type"]
                //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["}"]
                //     AT+ = (*) AT ["type"]
                //     AT+ = (*) AT ["}"]
                //     AT+ = (*) AT+ AT ["type"]
                //     AT+ = (*) AT+ AT ["}"]
                //     E = "{" ET (*) AT+ "}" [EOF]
                //     E = "{" ET (*) "}" [EOF]
                0, // on ";", error
                0, // on "enum", error
                13, // on "type", goto 12
                0, // on "{", error
                14, // on "}", goto 13
                0, // on r#"[a-zA-Z]+"#, error
                // State 6
                //     ET = "enum" (*) r#"[a-zA-Z]+"# "{" "}" ["type", "}"]
                0, // on ";", error
                0, // on "enum", error
                0, // on "type", error
                0, // on "{", error
                0, // on "}", error
                15, // on r#"[a-zA-Z]+"#, goto 14
                // State 7
                //     AT = "type" (*) r#"[a-zA-Z]+"# ";" ["enum", "type"]
                0, // on ";", error
                0, // on "enum", error
                0, // on "type", error
                0, // on "{", error
                0, // on "}", error
                16, // on r#"[a-zA-Z]+"#, goto 15
                // State 8
                //     AT+ = AT+ AT (*) ["enum", "type"]
                0, // on ";", error
                -5, // on "enum", reduce `AT+ = AT+, AT => ActionFn(7);`
                -5, // on "type", reduce `AT+ = AT+, AT => ActionFn(7);`
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 9
                //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["type"]
                //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["}"]
                //     AT+ = (*) AT ["type"]
                //     AT+ = (*) AT ["}"]
                //     AT+ = (*) AT+ AT ["type"]
                //     AT+ = (*) AT+ AT ["}"]
                //     E = "{" AT+ ET (*) AT+ "}" [EOF]
                //     E = "{" AT+ ET (*) "}" [EOF]
                0, // on ";", error
                0, // on "enum", error
                13, // on "type", goto 12
                0, // on "{", error
                18, // on "}", goto 17
                0, // on r#"[a-zA-Z]+"#, error
                // State 10
                //     AT+ = AT (*) ["type", "}"]
                0, // on ";", error
                0, // on "enum", error
                -4, // on "type", reduce `AT+ = AT => ActionFn(6);`
                0, // on "{", error
                -4, // on "}", reduce `AT+ = AT => ActionFn(6);`
                0, // on r#"[a-zA-Z]+"#, error
                // State 11
                //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["type", "}"]
                //     AT+ = AT+ (*) AT ["type", "}"]
                //     E = "{" ET AT+ (*) "}" [EOF]
                0, // on ";", error
                0, // on "enum", error
                13, // on "type", goto 12
                0, // on "{", error
                20, // on "}", goto 19
                0, // on r#"[a-zA-Z]+"#, error
                // State 12
                //     AT = "type" (*) r#"[a-zA-Z]+"# ";" ["type", "}"]
                0, // on ";", error
                0, // on "enum", error
                0, // on "type", error
                0, // on "{", error
                0, // on "}", error
                21, // on r#"[a-zA-Z]+"#, goto 20
                // State 13
                //     E = "{" ET "}" (*) [EOF]
                0, // on ";", error
                0, // on "enum", error
                0, // on "type", error
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 14
                //     ET = "enum" r#"[a-zA-Z]+"# (*) "{" "}" ["type", "}"]
                0, // on ";", error
                0, // on "enum", error
                0, // on "type", error
                22, // on "{", goto 21
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 15
                //     AT = "type" r#"[a-zA-Z]+"# (*) ";" ["enum", "type"]
                23, // on ";", goto 22
                0, // on "enum", error
                0, // on "type", error
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 16
                //     AT = (*) "type" r#"[a-zA-Z]+"# ";" ["type", "}"]
                //     AT+ = AT+ (*) AT ["type", "}"]
                //     E = "{" AT+ ET AT+ (*) "}" [EOF]
                0, // on ";", error
                0, // on "enum", error
                13, // on "type", goto 12
                0, // on "{", error
                24, // on "}", goto 23
                0, // on r#"[a-zA-Z]+"#, error
                // State 17
                //     E = "{" AT+ ET "}" (*) [EOF]
                0, // on ";", error
                0, // on "enum", error
                0, // on "type", error
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 18
                //     AT+ = AT+ AT (*) ["type", "}"]
                0, // on ";", error
                0, // on "enum", error
                -5, // on "type", reduce `AT+ = AT+, AT => ActionFn(7);`
                0, // on "{", error
                -5, // on "}", reduce `AT+ = AT+, AT => ActionFn(7);`
                0, // on r#"[a-zA-Z]+"#, error
                // State 19
                //     E = "{" ET AT+ "}" (*) [EOF]
                0, // on ";", error
                0, // on "enum", error
                0, // on "type", error
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 20
                //     AT = "type" r#"[a-zA-Z]+"# (*) ";" ["type", "}"]
                25, // on ";", goto 24
                0, // on "enum", error
                0, // on "type", error
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 21
                //     ET = "enum" r#"[a-zA-Z]+"# "{" (*) "}" ["type", "}"]
                0, // on ";", error
                0, // on "enum", error
                0, // on "type", error
                0, // on "{", error
                26, // on "}", goto 25
                0, // on r#"[a-zA-Z]+"#, error
                // State 22
                //     AT = "type" r#"[a-zA-Z]+"# ";" (*) ["enum", "type"]
                0, // on ";", error
                -1, // on "enum", reduce `AT = "type", r#"[a-zA-Z]+"#, ";" => ActionFn(2);`
                -1, // on "type", reduce `AT = "type", r#"[a-zA-Z]+"#, ";" => ActionFn(2);`
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 23
                //     E = "{" AT+ ET AT+ "}" (*) [EOF]
                0, // on ";", error
                0, // on "enum", error
                0, // on "type", error
                0, // on "{", error
                0, // on "}", error
                0, // on r#"[a-zA-Z]+"#, error
                // State 24
                //     AT = "type" r#"[a-zA-Z]+"# ";" (*) ["type", "}"]
                0, // on ";", error
                0, // on "enum", error
                -1, // on "type", reduce `AT = "type", r#"[a-zA-Z]+"#, ";" => ActionFn(2);`
                0, // on "{", error
                -1, // on "}", reduce `AT = "type", r#"[a-zA-Z]+"#, ";" => ActionFn(2);`
                0, // on r#"[a-zA-Z]+"#, error
                // State 25
                //     ET = "enum" r#"[a-zA-Z]+"# "{" "}" (*) ["type", "}"]
                0, // on ";", error
                0, // on "enum", error
                -10, // on "type", reduce `ET = "enum", r#"[a-zA-Z]+"#, "{", "}" => ActionFn(3);`
                0, // on "{", error
                -10, // on "}", reduce `ET = "enum", r#"[a-zA-Z]+"#, "{", "}" => ActionFn(3);`
                0, // on r#"[a-zA-Z]+"#, error
            ];
            const __EOF_ACTION: &'static [i32] = &[
                0, // on EOF, error
                -11, // on EOF, reduce `__E = E => ActionFn(0);`
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                -6, // on EOF, reduce `E = "{", ET, "}" => ActionFn(8);`
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                -8, // on EOF, reduce `E = "{", AT+, ET, "}" => ActionFn(10);`
                0, // on EOF, error
                -7, // on EOF, reduce `E = "{", ET, AT+, "}" => ActionFn(9);`
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                -9, // on EOF, reduce `E = "{", AT+, ET, AT+, "}" => ActionFn(11);`
                0, // on EOF, error
                0, // on EOF, error
            ];
            const __GOTO: &'static [i32] = &[
                // State 0
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                2, // on E, goto 1
                0, // on ET, error
                0, // on __E, error
                // State 1
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 2
                4, // on AT, goto 3
                0, // on AT*, error
                5, // on AT+, goto 4
                0, // on E, error
                6, // on ET, goto 5
                0, // on __E, error
                // State 3
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 4
                9, // on AT, goto 8
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                10, // on ET, goto 9
                0, // on __E, error
                // State 5
                11, // on AT, goto 10
                0, // on AT*, error
                12, // on AT+, goto 11
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 6
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 7
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 8
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 9
                11, // on AT, goto 10
                0, // on AT*, error
                17, // on AT+, goto 16
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 10
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 11
                19, // on AT, goto 18
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 12
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 13
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 14
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 15
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 16
                19, // on AT, goto 18
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 17
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 18
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 19
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 20
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 21
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 22
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 23
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 24
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
                // State 25
                0, // on AT, error
                0, // on AT*, error
                0, // on AT+, error
                0, // on E, error
                0, // on ET, error
                0, // on __E, error
            ];
            pub fn parse_E<
                'input,
            >(
                input: &'input str,
            ) -> Result<(Vec<&'input str>, &'input str, Vec<&'input str>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
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
                        (_, (4, _), _) if true => 4,
                        (_, (5, _), _) if true => 5,
                        _ => {
                            return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: vec![],
                            });
                        }
                    };
                    loop {
                        let __state = *__states.last().unwrap() as usize;
                        let __action = __ACTION[__state * 6 + __integer];
                        if __action > 0 {
                            let __symbol = match __integer {
                                0 => match __lookahead.1 {
                                    (0, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                                    _ => unreachable!(),
                                },
                                1 => match __lookahead.1 {
                                    (1, __tok0) => __Symbol::Term_22enum_22(__tok0),
                                    _ => unreachable!(),
                                },
                                2 => match __lookahead.1 {
                                    (2, __tok0) => __Symbol::Term_22type_22(__tok0),
                                    _ => unreachable!(),
                                },
                                3 => match __lookahead.1 {
                                    (3, __tok0) => __Symbol::Term_22_7b_22(__tok0),
                                    _ => unreachable!(),
                                },
                                4 => match __lookahead.1 {
                                    (4, __tok0) => __Symbol::Term_22_7d_22(__tok0),
                                    _ => unreachable!(),
                                },
                                5 => match __lookahead.1 {
                                    (5, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__tok0),
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
                        if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                _: ::std::marker::PhantomData<()>,
            ) -> Option<Result<(Vec<&'input str>, &'input str, Vec<&'input str>),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
            {
                let __nonterminal = match -__action {
                    1 => {
                        // AT = "type", r#"[a-zA-Z]+"#, ";" => ActionFn(2);
                        let __sym2 = __pop_Term_22_3b_22(__symbols);
                        let __sym1 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__symbols);
                        let __sym0 = __pop_Term_22type_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<>(input, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtAT(__nt), __end));
                        0
                    }
                    2 => {
                        // AT* =  => ActionFn(4);
                        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action4::<>(input, &__start, &__end);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 0);
                        __symbols.push((__start, __Symbol::NtAT_2a(__nt), __end));
                        1
                    }
                    3 => {
                        // AT* = AT+ => ActionFn(5);
                        let __sym0 = __pop_NtAT_2b(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action5::<>(input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtAT_2a(__nt), __end));
                        1
                    }
                    4 => {
                        // AT+ = AT => ActionFn(6);
                        let __sym0 = __pop_NtAT(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action6::<>(input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtAT_2b(__nt), __end));
                        2
                    }
                    5 => {
                        // AT+ = AT+, AT => ActionFn(7);
                        let __sym1 = __pop_NtAT(__symbols);
                        let __sym0 = __pop_NtAT_2b(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action7::<>(input, __sym0, __sym1);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 2);
                        __symbols.push((__start, __Symbol::NtAT_2b(__nt), __end));
                        2
                    }
                    6 => {
                        // E = "{", ET, "}" => ActionFn(8);
                        let __sym2 = __pop_Term_22_7d_22(__symbols);
                        let __sym1 = __pop_NtET(__symbols);
                        let __sym0 = __pop_Term_22_7b_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action8::<>(input, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtE(__nt), __end));
                        3
                    }
                    7 => {
                        // E = "{", ET, AT+, "}" => ActionFn(9);
                        let __sym3 = __pop_Term_22_7d_22(__symbols);
                        let __sym2 = __pop_NtAT_2b(__symbols);
                        let __sym1 = __pop_NtET(__symbols);
                        let __sym0 = __pop_Term_22_7b_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 4);
                        __symbols.push((__start, __Symbol::NtE(__nt), __end));
                        3
                    }
                    8 => {
                        // E = "{", AT+, ET, "}" => ActionFn(10);
                        let __sym3 = __pop_Term_22_7d_22(__symbols);
                        let __sym2 = __pop_NtET(__symbols);
                        let __sym1 = __pop_NtAT_2b(__symbols);
                        let __sym0 = __pop_Term_22_7b_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 4);
                        __symbols.push((__start, __Symbol::NtE(__nt), __end));
                        3
                    }
                    9 => {
                        // E = "{", AT+, ET, AT+, "}" => ActionFn(11);
                        let __sym4 = __pop_Term_22_7d_22(__symbols);
                        let __sym3 = __pop_NtAT_2b(__symbols);
                        let __sym2 = __pop_NtET(__symbols);
                        let __sym1 = __pop_NtAT_2b(__symbols);
                        let __sym0 = __pop_Term_22_7b_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym4.2.clone();
                        let __nt = super::super::super::__action11::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 5);
                        __symbols.push((__start, __Symbol::NtE(__nt), __end));
                        3
                    }
                    10 => {
                        // ET = "enum", r#"[a-zA-Z]+"#, "{", "}" => ActionFn(3);
                        let __sym3 = __pop_Term_22_7d_22(__symbols);
                        let __sym2 = __pop_Term_22_7b_22(__symbols);
                        let __sym1 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__symbols);
                        let __sym0 = __pop_Term_22enum_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action3::<>(input, __sym0, __sym1, __sym2, __sym3);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 4);
                        __symbols.push((__start, __Symbol::NtET(__nt), __end));
                        4
                    }
                    11 => {
                        // __E = E => ActionFn(0);
                        let __sym0 = __pop_NtE(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<>(input, __sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 6 + __nonterminal] - 1;
                __states.push(__next_state);
                None
            }
            fn __pop_Term_22_3b_22<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, &'input str, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22enum_22<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, &'input str, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22enum_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22type_22<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, &'input str, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22type_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_7b_22<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, &'input str, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_7d_22<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, &'input str, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, &'input str, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtAT<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, &'input str, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtAT(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtAT_2a<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, ::std::vec::Vec<&'input str>, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtAT_2a(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtAT_2b<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, ::std::vec::Vec<&'input str>, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtAT_2b(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtE<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, (Vec<&'input str>, &'input str, Vec<&'input str>), usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtE(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtET<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, &'input str, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtET(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____E<
                'input,
            >(
                input: &'input str,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> (usize, (Vec<&'input str>, &'input str, Vec<&'input str>), usize)
            {
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
                        59 => /* ';' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        102 ... 115 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        123 => /* '{' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        125 => /* '}' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 6;
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
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
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
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 120 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        122 => /* 'z' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 8;
                            continue;
                        }
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
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
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
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
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 111 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        113 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
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
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 108 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        110 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
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
    (_, __0, _): (usize, (Vec<&'input str>, &'input str, Vec<&'input str>), usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    (__0, __1, __2)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<&'input str>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> ::std::vec::Vec<&'input str>
{
    v
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, e, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __start1 = __1.2.clone();
    let __end1 = __2.0.clone();
    let __temp0 = __action4(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action4(
        input,
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action1(
        input,
        __0,
        __temp0,
        __1,
        __temp1,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ::std::vec::Vec<&'input str>, usize),
    __3: (usize, &'input str, usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __start1 = __2.0.clone();
    let __end1 = __2.2.clone();
    let __temp0 = __action4(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action5(
        input,
        __2,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action1(
        input,
        __0,
        __temp0,
        __1,
        __temp1,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<&'input str>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __start1 = __2.2.clone();
    let __end1 = __3.0.clone();
    let __temp0 = __action5(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action4(
        input,
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action1(
        input,
        __0,
        __temp0,
        __2,
        __temp1,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<&'input str>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<&'input str>, usize),
    __4: (usize, &'input str, usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __start1 = __3.0.clone();
    let __end1 = __3.2.clone();
    let __temp0 = __action5(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action5(
        input,
        __3,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action1(
        input,
        __0,
        __temp0,
        __2,
        __temp1,
        __4,
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
