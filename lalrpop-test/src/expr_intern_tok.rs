#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Expr<
        'input,
    >(
        scale: i32,
        input: &'input str,
    ) -> Result<i32, __ParseError<usize,(usize, &'input str),()>>
    {
        let __ascent = __ascent::parse_Expr(
            scale,
            input,
        );
        let __parse_table = __parse_table::parse_Expr(
            scale,
            input,
        );
        assert_eq!(__ascent, __parse_table);
        return __ascent;
    }
    mod __ascent {

        mod __parse__Expr {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use std::str::FromStr;
            use util::tok::Tok;
            extern crate lalrpop_util as __lalrpop_util;
            use self::__lalrpop_util::ParseError as __ParseError;
            pub fn parse_Expr<
                'input,
            >(
                scale: i32,
                input: &'input str,
            ) -> Result<i32, __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __tokens = super::super::super::__intern_token::__Matcher::new(input);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match try!(__state0(scale, input, &mut __tokens, __lookahead)) {
                    (Some(__lookahead), _) => {
                        Err(__ParseError::ExtraToken { token: __lookahead })
                    }
                    (None, __Nonterminal::____Expr((_, __nt, _))) => {
                        Ok(__nt)
                    }
                    _ => unreachable!(),
                }
            }

            #[allow(dead_code)]
            pub enum __Nonterminal<> {
                Expr((usize, i32, usize)),
                Factor((usize, i32, usize)),
                Num((usize, i32, usize)),
                Term((usize, i32, usize)),
                ____Expr((usize, i32, usize)),
            }

            // State 0
            //     AllInputs = []
            //     OptionalInputs = []
            //     FixedInputs = []
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Expr = (*) Expr "+" Factor ["+"]
            //     Expr = (*) Expr "+" Factor ["-"]
            //     Expr = (*) Expr "+" Factor [EOF]
            //     Expr = (*) Expr "-" Factor ["+"]
            //     Expr = (*) Expr "-" Factor ["-"]
            //     Expr = (*) Expr "-" Factor [EOF]
            //     Expr = (*) Factor ["+"]
            //     Expr = (*) Factor ["-"]
            //     Expr = (*) Factor [EOF]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["+"]
            //     Factor = (*) Factor "*" Term ["-"]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "*" Term [EOF]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["+"]
            //     Factor = (*) Factor "/" Term ["-"]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Factor "/" Term [EOF]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["+"]
            //     Factor = (*) Term ["-"]
            //     Factor = (*) Term ["/"]
            //     Factor = (*) Term [EOF]
            //     Num = (*) r#"[0-9]+"# ["*"]
            //     Num = (*) r#"[0-9]+"# ["+"]
            //     Num = (*) r#"[0-9]+"# ["-"]
            //     Num = (*) r#"[0-9]+"# ["/"]
            //     Num = (*) r#"[0-9]+"# [EOF]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+"]
            //     Term = (*) Num ["-"]
            //     Term = (*) Num ["/"]
            //     Term = (*) Num [EOF]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+"]
            //     Term = (*) "(" Expr ")" ["-"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) "(" Expr ")" [EOF]
            //     __Expr = (*) Expr [EOF]
            //
            //   "(" -> S5
            //   r#"[0-9]+"# -> S6
            //
            //     Expr -> S1
            //     Factor -> S2
            //     Num -> S3
            //     Term -> S4
            pub fn __state0<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(scale, input, __tokens, __sym0));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(scale, input, __tokens, __sym0));
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
                        __Nonterminal::Expr(__sym0) => {
                            __result = try!(__state1(scale, input, __tokens, __lookahead, __sym0));
                        }
                        __Nonterminal::Factor(__sym0) => {
                            __result = try!(__state2(scale, input, __tokens, __lookahead, __sym0));
                        }
                        __Nonterminal::Num(__sym0) => {
                            __result = try!(__state3(scale, input, __tokens, __lookahead, __sym0));
                        }
                        __Nonterminal::Term(__sym0) => {
                            __result = try!(__state4(scale, input, __tokens, __lookahead, __sym0));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 1
            //     AllInputs = [Expr]
            //     OptionalInputs = []
            //     FixedInputs = [Expr]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Expr = Expr (*) "+" Factor ["+", "-", EOF]
            //     Expr = Expr (*) "-" Factor ["+", "-", EOF]
            //     __Expr = Expr (*) [EOF]
            //
            //   "+" -> S7
            //   "-" -> S8
            //   [EOF] -> __Expr = Expr => ActionFn(0);
            //
            pub fn __state1<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (3, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state7(scale, input, __tokens, __sym0, __sym1));
                        return Ok(__result);
                    }
                    Some((__loc1, (4, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state8(scale, input, __tokens, __sym0, __sym1));
                        return Ok(__result);
                    }
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0(scale, input, __sym0);
                        let __nt = __Nonterminal::____Expr((
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

            // State 2
            //     AllInputs = [Factor]
            //     OptionalInputs = []
            //     FixedInputs = [Factor]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Expr = Factor (*) ["+", "-", EOF]
            //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
            //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
            //
            //   "*" -> S9
            //   "/" -> S10
            //   ["+", "-", EOF] -> Expr = Factor => ActionFn(3);
            //
            pub fn __state2<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state9(scale, input, __tokens, __sym0, __sym1));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state10(scale, input, __tokens, __sym0, __sym1));
                        return Ok(__result);
                    }
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3(scale, input, __sym0);
                        let __nt = __Nonterminal::Expr((
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
            //     AllInputs = [Num]
            //     OptionalInputs = []
            //     FixedInputs = [Num]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = Num (*) ["*", "+", "-", "/", EOF]
            //
            //   ["*", "+", "-", "/", EOF] -> Term = Num => ActionFn(7);
            //
            pub fn __state3<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action7(scale, input, __sym0);
                        let __nt = __Nonterminal::Term((
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
            //     AllInputs = [Term]
            //     OptionalInputs = []
            //     FixedInputs = [Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = Term (*) ["*", "+", "-", "/", EOF]
            //
            //   ["*", "+", "-", "/", EOF] -> Factor = Term => ActionFn(6);
            //
            pub fn __state4<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action6(scale, input, __sym0);
                        let __nt = __Nonterminal::Factor((
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

            // State 5
            //     AllInputs = ["("]
            //     OptionalInputs = []
            //     FixedInputs = ["("]
            //     WillPushLen = 2
            //     WillPush = [Expr, ")"]
            //     WillProduce = Some(Term)
            //
            //     Expr = (*) Expr "+" Factor [")"]
            //     Expr = (*) Expr "+" Factor ["+"]
            //     Expr = (*) Expr "+" Factor ["-"]
            //     Expr = (*) Expr "-" Factor [")"]
            //     Expr = (*) Expr "-" Factor ["+"]
            //     Expr = (*) Expr "-" Factor ["-"]
            //     Expr = (*) Factor [")"]
            //     Expr = (*) Factor ["+"]
            //     Expr = (*) Factor ["-"]
            //     Factor = (*) Factor "*" Term [")"]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["+"]
            //     Factor = (*) Factor "*" Term ["-"]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term [")"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["+"]
            //     Factor = (*) Factor "/" Term ["-"]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term [")"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["+"]
            //     Factor = (*) Term ["-"]
            //     Factor = (*) Term ["/"]
            //     Num = (*) r#"[0-9]+"# [")"]
            //     Num = (*) r#"[0-9]+"# ["*"]
            //     Num = (*) r#"[0-9]+"# ["+"]
            //     Num = (*) r#"[0-9]+"# ["-"]
            //     Num = (*) r#"[0-9]+"# ["/"]
            //     Term = (*) Num [")"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+"]
            //     Term = (*) Num ["-"]
            //     Term = (*) Num ["/"]
            //     Term = (*) "(" Expr ")" [")"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+"]
            //     Term = (*) "(" Expr ")" ["-"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = "(" (*) Expr ")" ["*", "+", "-", "/", EOF]
            //
            //   "(" -> S15
            //   r#"[0-9]+"# -> S16
            //
            //     Expr -> S11
            //     Factor -> S12
            //     Num -> S13
            //     Term -> S14
            pub fn __state5<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
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
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state15(scale, input, __tokens, __sym1));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(scale, input, __tokens, __sym1));
                    }
                    _ => {
                        return Err(__ParseError::UnrecognizedToken {
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
                        __Nonterminal::Expr(__sym1) => {
                            __result = try!(__state11(scale, input, __tokens, __lookahead, __sym0, __sym1));
                        }
                        __Nonterminal::Factor(__sym1) => {
                            __result = try!(__state12(scale, input, __tokens, __lookahead, __sym1));
                        }
                        __Nonterminal::Num(__sym1) => {
                            __result = try!(__state13(scale, input, __tokens, __lookahead, __sym1));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state14(scale, input, __tokens, __lookahead, __sym1));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 6
            //     AllInputs = [r#"[0-9]+"#]
            //     OptionalInputs = []
            //     FixedInputs = [r#"[0-9]+"#]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Num)
            //
            //     Num = r#"[0-9]+"# (*) ["*", "+", "-", "/", EOF]
            //
            //   ["*", "+", "-", "/", EOF] -> Num = r#"[0-9]+"# => ActionFn(9);
            //
            pub fn __state6<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
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
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action9(scale, input, __sym0);
                        let __nt = __Nonterminal::Num((
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

            // State 7
            //     AllInputs = [Expr, "+"]
            //     OptionalInputs = []
            //     FixedInputs = [Expr, "+"]
            //     WillPushLen = 1
            //     WillPush = [Factor]
            //     WillProduce = Some(Expr)
            //
            //     Expr = Expr "+" (*) Factor ["+", "-", EOF]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["+", "-", EOF]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["+", "-", EOF]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["+", "-", EOF]
            //     Factor = (*) Term ["/"]
            //     Num = (*) r#"[0-9]+"# ["*"]
            //     Num = (*) r#"[0-9]+"# ["+", "-", EOF]
            //     Num = (*) r#"[0-9]+"# ["/"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+", "-", EOF]
            //     Term = (*) Num ["/"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+", "-", EOF]
            //     Term = (*) "(" Expr ")" ["/"]
            //
            //   "(" -> S5
            //   r#"[0-9]+"# -> S6
            //
            //     Factor -> S17
            //     Num -> S3
            //     Term -> S4
            pub fn __state7<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, i32, usize),
                __sym1: (usize, &'input str, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                let __sym0 = &mut Some(__sym0);
                let __sym1 = &mut Some(__sym1);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(scale, input, __tokens, __sym2));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(scale, input, __tokens, __sym2));
                    }
                    _ => {
                        return Err(__ParseError::UnrecognizedToken {
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
                        __Nonterminal::Factor(__sym2) => {
                            __result = try!(__state17(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                        }
                        __Nonterminal::Num(__sym2) => {
                            __result = try!(__state3(scale, input, __tokens, __lookahead, __sym2));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state4(scale, input, __tokens, __lookahead, __sym2));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 8
            //     AllInputs = [Expr, "-"]
            //     OptionalInputs = []
            //     FixedInputs = [Expr, "-"]
            //     WillPushLen = 1
            //     WillPush = [Factor]
            //     WillProduce = Some(Expr)
            //
            //     Expr = Expr "-" (*) Factor ["+", "-", EOF]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["+", "-", EOF]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["+", "-", EOF]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["+", "-", EOF]
            //     Factor = (*) Term ["/"]
            //     Num = (*) r#"[0-9]+"# ["*"]
            //     Num = (*) r#"[0-9]+"# ["+", "-", EOF]
            //     Num = (*) r#"[0-9]+"# ["/"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+", "-", EOF]
            //     Term = (*) Num ["/"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+", "-", EOF]
            //     Term = (*) "(" Expr ")" ["/"]
            //
            //   "(" -> S5
            //   r#"[0-9]+"# -> S6
            //
            //     Factor -> S18
            //     Num -> S3
            //     Term -> S4
            pub fn __state8<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, i32, usize),
                __sym1: (usize, &'input str, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                let __sym0 = &mut Some(__sym0);
                let __sym1 = &mut Some(__sym1);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(scale, input, __tokens, __sym2));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(scale, input, __tokens, __sym2));
                    }
                    _ => {
                        return Err(__ParseError::UnrecognizedToken {
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
                        __Nonterminal::Factor(__sym2) => {
                            __result = try!(__state18(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                        }
                        __Nonterminal::Num(__sym2) => {
                            __result = try!(__state3(scale, input, __tokens, __lookahead, __sym2));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state4(scale, input, __tokens, __lookahead, __sym2));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 9
            //     AllInputs = [Factor, "*"]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "*"]
            //     WillPushLen = 1
            //     WillPush = [Term]
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "*" (*) Term ["*", "+", "-", "/", EOF]
            //     Num = (*) r#"[0-9]+"# ["*", "+", "-", "/", EOF]
            //     Term = (*) Num ["*", "+", "-", "/", EOF]
            //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
            //
            //   "(" -> S5
            //   r#"[0-9]+"# -> S6
            //
            //     Num -> S3
            //     Term -> S19
            pub fn __state9<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, i32, usize),
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
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(scale, input, __tokens, __sym2));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(scale, input, __tokens, __sym2));
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
                        __Nonterminal::Num(__sym2) => {
                            __result = try!(__state3(scale, input, __tokens, __lookahead, __sym2));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state19(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 10
            //     AllInputs = [Factor, "/"]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "/"]
            //     WillPushLen = 1
            //     WillPush = [Term]
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "/" (*) Term ["*", "+", "-", "/", EOF]
            //     Num = (*) r#"[0-9]+"# ["*", "+", "-", "/", EOF]
            //     Term = (*) Num ["*", "+", "-", "/", EOF]
            //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
            //
            //   "(" -> S5
            //   r#"[0-9]+"# -> S6
            //
            //     Num -> S3
            //     Term -> S20
            pub fn __state10<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, i32, usize),
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
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(scale, input, __tokens, __sym2));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(scale, input, __tokens, __sym2));
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
                        __Nonterminal::Num(__sym2) => {
                            __result = try!(__state3(scale, input, __tokens, __lookahead, __sym2));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state20(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 11
            //     AllInputs = ["(", Expr]
            //     OptionalInputs = ["("]
            //     FixedInputs = [Expr]
            //     WillPushLen = 1
            //     WillPush = [")"]
            //     WillProduce = None
            //
            //     Expr = Expr (*) "+" Factor [")", "+", "-"]
            //     Expr = Expr (*) "-" Factor [")", "+", "-"]
            //     Term = "(" Expr (*) ")" ["*", "+", "-", "/", EOF]
            //
            //   ")" -> S21
            //   "+" -> S22
            //   "-" -> S23
            //
            pub fn __state11<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, &'input str, usize)>,
                __sym1: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state21(scale, input, __tokens, __sym0, __sym1, __sym2));
                        return Ok(__result);
                    }
                    Some((__loc1, (3, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state22(scale, input, __tokens, __sym1, __sym2));
                        return Ok(__result);
                    }
                    Some((__loc1, (4, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state23(scale, input, __tokens, __sym1, __sym2));
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

            // State 12
            //     AllInputs = [Factor]
            //     OptionalInputs = []
            //     FixedInputs = [Factor]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Expr = Factor (*) [")", "+", "-"]
            //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
            //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
            //
            //   "*" -> S24
            //   "/" -> S25
            //   [")", "+", "-"] -> Expr = Factor => ActionFn(3);
            //
            pub fn __state12<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state24(scale, input, __tokens, __sym0, __sym1));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state25(scale, input, __tokens, __sym0, __sym1));
                        return Ok(__result);
                    }
                    Some((_, (1, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3(scale, input, __sym0);
                        let __nt = __Nonterminal::Expr((
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

            // State 13
            //     AllInputs = [Num]
            //     OptionalInputs = []
            //     FixedInputs = [Num]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = Num (*) [")", "*", "+", "-", "/"]
            //
            //   [")", "*", "+", "-", "/"] -> Term = Num => ActionFn(7);
            //
            pub fn __state13<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action7(scale, input, __sym0);
                        let __nt = __Nonterminal::Term((
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

            // State 14
            //     AllInputs = [Term]
            //     OptionalInputs = []
            //     FixedInputs = [Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = Term (*) [")", "*", "+", "-", "/"]
            //
            //   [")", "*", "+", "-", "/"] -> Factor = Term => ActionFn(6);
            //
            pub fn __state14<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action6(scale, input, __sym0);
                        let __nt = __Nonterminal::Factor((
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

            // State 15
            //     AllInputs = ["("]
            //     OptionalInputs = []
            //     FixedInputs = ["("]
            //     WillPushLen = 2
            //     WillPush = [Expr, ")"]
            //     WillProduce = Some(Term)
            //
            //     Expr = (*) Expr "+" Factor [")"]
            //     Expr = (*) Expr "+" Factor ["+"]
            //     Expr = (*) Expr "+" Factor ["-"]
            //     Expr = (*) Expr "-" Factor [")"]
            //     Expr = (*) Expr "-" Factor ["+"]
            //     Expr = (*) Expr "-" Factor ["-"]
            //     Expr = (*) Factor [")"]
            //     Expr = (*) Factor ["+"]
            //     Expr = (*) Factor ["-"]
            //     Factor = (*) Factor "*" Term [")"]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["+"]
            //     Factor = (*) Factor "*" Term ["-"]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term [")"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["+"]
            //     Factor = (*) Factor "/" Term ["-"]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term [")"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["+"]
            //     Factor = (*) Term ["-"]
            //     Factor = (*) Term ["/"]
            //     Num = (*) r#"[0-9]+"# [")"]
            //     Num = (*) r#"[0-9]+"# ["*"]
            //     Num = (*) r#"[0-9]+"# ["+"]
            //     Num = (*) r#"[0-9]+"# ["-"]
            //     Num = (*) r#"[0-9]+"# ["/"]
            //     Term = (*) Num [")"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+"]
            //     Term = (*) Num ["-"]
            //     Term = (*) Num ["/"]
            //     Term = (*) "(" Expr ")" [")"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+"]
            //     Term = (*) "(" Expr ")" ["-"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = "(" (*) Expr ")" [")", "*", "+", "-", "/"]
            //
            //   "(" -> S15
            //   r#"[0-9]+"# -> S16
            //
            //     Expr -> S26
            //     Factor -> S12
            //     Num -> S13
            //     Term -> S14
            pub fn __state15<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
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
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state15(scale, input, __tokens, __sym1));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(scale, input, __tokens, __sym1));
                    }
                    _ => {
                        return Err(__ParseError::UnrecognizedToken {
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
                        __Nonterminal::Expr(__sym1) => {
                            __result = try!(__state26(scale, input, __tokens, __lookahead, __sym0, __sym1));
                        }
                        __Nonterminal::Factor(__sym1) => {
                            __result = try!(__state12(scale, input, __tokens, __lookahead, __sym1));
                        }
                        __Nonterminal::Num(__sym1) => {
                            __result = try!(__state13(scale, input, __tokens, __lookahead, __sym1));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state14(scale, input, __tokens, __lookahead, __sym1));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 16
            //     AllInputs = [r#"[0-9]+"#]
            //     OptionalInputs = []
            //     FixedInputs = [r#"[0-9]+"#]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Num)
            //
            //     Num = r#"[0-9]+"# (*) [")", "*", "+", "-", "/"]
            //
            //   [")", "*", "+", "-", "/"] -> Num = r#"[0-9]+"# => ActionFn(9);
            //
            pub fn __state16<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
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
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action9(scale, input, __sym0);
                        let __nt = __Nonterminal::Num((
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

            // State 17
            //     AllInputs = [Expr, "+", Factor]
            //     OptionalInputs = [Expr, "+"]
            //     FixedInputs = [Factor]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Expr = Expr "+" Factor (*) ["+", "-", EOF]
            //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
            //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
            //
            //   "*" -> S9
            //   "/" -> S10
            //   ["+", "-", EOF] -> Expr = Expr, "+", Factor => ActionFn(2);
            //
            pub fn __state17<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, i32, usize)>,
                __sym1: &mut Option<(usize, &'input str, usize)>,
                __sym2: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state9(scale, input, __tokens, __sym2, __sym3));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state10(scale, input, __tokens, __sym2, __sym3));
                        return Ok(__result);
                    }
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    None => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2(scale, input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Expr((
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

            // State 18
            //     AllInputs = [Expr, "-", Factor]
            //     OptionalInputs = [Expr, "-"]
            //     FixedInputs = [Factor]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Expr = Expr "-" Factor (*) ["+", "-", EOF]
            //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
            //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
            //
            //   "*" -> S9
            //   "/" -> S10
            //   ["+", "-", EOF] -> Expr = Expr, "-", Factor => ActionFn(1);
            //
            pub fn __state18<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, i32, usize)>,
                __sym1: &mut Option<(usize, &'input str, usize)>,
                __sym2: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state9(scale, input, __tokens, __sym2, __sym3));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state10(scale, input, __tokens, __sym2, __sym3));
                        return Ok(__result);
                    }
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    None => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1(scale, input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Expr((
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

            // State 19
            //     AllInputs = [Factor, "*", Term]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "*", Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "*" Term (*) ["*", "+", "-", "/", EOF]
            //
            //   ["*", "+", "-", "/", EOF] -> Factor = Factor, "*", Term => ActionFn(4);
            //
            pub fn __state19<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, i32, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action4(scale, input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Factor((
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

            // State 20
            //     AllInputs = [Factor, "/", Term]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "/", Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "/" Term (*) ["*", "+", "-", "/", EOF]
            //
            //   ["*", "+", "-", "/", EOF] -> Factor = Factor, "/", Term => ActionFn(5);
            //
            pub fn __state20<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, i32, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5(scale, input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Factor((
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

            // State 21
            //     AllInputs = ["(", Expr, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["(", Expr, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = "(" Expr ")" (*) ["*", "+", "-", "/", EOF]
            //
            //   ["*", "+", "-", "/", EOF] -> Term = "(", Expr, ")" => ActionFn(8);
            //
            pub fn __state21<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, i32, usize),
                __sym2: (usize, &'input str, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action8(scale, input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Term((
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

            // State 22
            //     AllInputs = [Expr, "+"]
            //     OptionalInputs = []
            //     FixedInputs = [Expr, "+"]
            //     WillPushLen = 1
            //     WillPush = [Factor]
            //     WillProduce = Some(Expr)
            //
            //     Expr = Expr "+" (*) Factor [")", "+", "-"]
            //     Factor = (*) Factor "*" Term [")", "+", "-"]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term [")", "+", "-"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term [")", "+", "-"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["/"]
            //     Num = (*) r#"[0-9]+"# [")", "+", "-"]
            //     Num = (*) r#"[0-9]+"# ["*"]
            //     Num = (*) r#"[0-9]+"# ["/"]
            //     Term = (*) Num [")", "+", "-"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["/"]
            //     Term = (*) "(" Expr ")" [")", "+", "-"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["/"]
            //
            //   "(" -> S15
            //   r#"[0-9]+"# -> S16
            //
            //     Factor -> S27
            //     Num -> S13
            //     Term -> S14
            pub fn __state22<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, i32, usize),
                __sym1: (usize, &'input str, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                let __sym0 = &mut Some(__sym0);
                let __sym1 = &mut Some(__sym1);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state15(scale, input, __tokens, __sym2));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(scale, input, __tokens, __sym2));
                    }
                    _ => {
                        return Err(__ParseError::UnrecognizedToken {
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
                        __Nonterminal::Factor(__sym2) => {
                            __result = try!(__state27(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                        }
                        __Nonterminal::Num(__sym2) => {
                            __result = try!(__state13(scale, input, __tokens, __lookahead, __sym2));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state14(scale, input, __tokens, __lookahead, __sym2));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 23
            //     AllInputs = [Expr, "-"]
            //     OptionalInputs = []
            //     FixedInputs = [Expr, "-"]
            //     WillPushLen = 1
            //     WillPush = [Factor]
            //     WillProduce = Some(Expr)
            //
            //     Expr = Expr "-" (*) Factor [")", "+", "-"]
            //     Factor = (*) Factor "*" Term [")", "+", "-"]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term [")", "+", "-"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term [")", "+", "-"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["/"]
            //     Num = (*) r#"[0-9]+"# [")", "+", "-"]
            //     Num = (*) r#"[0-9]+"# ["*"]
            //     Num = (*) r#"[0-9]+"# ["/"]
            //     Term = (*) Num [")", "+", "-"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["/"]
            //     Term = (*) "(" Expr ")" [")", "+", "-"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["/"]
            //
            //   "(" -> S15
            //   r#"[0-9]+"# -> S16
            //
            //     Factor -> S28
            //     Num -> S13
            //     Term -> S14
            pub fn __state23<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, i32, usize),
                __sym1: (usize, &'input str, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                let __sym0 = &mut Some(__sym0);
                let __sym1 = &mut Some(__sym1);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state15(scale, input, __tokens, __sym2));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(scale, input, __tokens, __sym2));
                    }
                    _ => {
                        return Err(__ParseError::UnrecognizedToken {
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
                        __Nonterminal::Factor(__sym2) => {
                            __result = try!(__state28(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                        }
                        __Nonterminal::Num(__sym2) => {
                            __result = try!(__state13(scale, input, __tokens, __lookahead, __sym2));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state14(scale, input, __tokens, __lookahead, __sym2));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 24
            //     AllInputs = [Factor, "*"]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "*"]
            //     WillPushLen = 1
            //     WillPush = [Term]
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "*" (*) Term [")", "*", "+", "-", "/"]
            //     Num = (*) r#"[0-9]+"# [")", "*", "+", "-", "/"]
            //     Term = (*) Num [")", "*", "+", "-", "/"]
            //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
            //
            //   "(" -> S15
            //   r#"[0-9]+"# -> S16
            //
            //     Num -> S13
            //     Term -> S29
            pub fn __state24<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, i32, usize),
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
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state15(scale, input, __tokens, __sym2));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(scale, input, __tokens, __sym2));
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
                        __Nonterminal::Num(__sym2) => {
                            __result = try!(__state13(scale, input, __tokens, __lookahead, __sym2));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state29(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 25
            //     AllInputs = [Factor, "/"]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "/"]
            //     WillPushLen = 1
            //     WillPush = [Term]
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "/" (*) Term [")", "*", "+", "-", "/"]
            //     Num = (*) r#"[0-9]+"# [")", "*", "+", "-", "/"]
            //     Term = (*) Num [")", "*", "+", "-", "/"]
            //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
            //
            //   "(" -> S15
            //   r#"[0-9]+"# -> S16
            //
            //     Num -> S13
            //     Term -> S30
            pub fn __state25<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, i32, usize),
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
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state15(scale, input, __tokens, __sym2));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(scale, input, __tokens, __sym2));
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
                        __Nonterminal::Num(__sym2) => {
                            __result = try!(__state13(scale, input, __tokens, __lookahead, __sym2));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state30(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 26
            //     AllInputs = ["(", Expr]
            //     OptionalInputs = ["("]
            //     FixedInputs = [Expr]
            //     WillPushLen = 1
            //     WillPush = [")"]
            //     WillProduce = None
            //
            //     Expr = Expr (*) "+" Factor [")", "+", "-"]
            //     Expr = Expr (*) "-" Factor [")", "+", "-"]
            //     Term = "(" Expr (*) ")" [")", "*", "+", "-", "/"]
            //
            //   ")" -> S31
            //   "+" -> S22
            //   "-" -> S23
            //
            pub fn __state26<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, &'input str, usize)>,
                __sym1: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state31(scale, input, __tokens, __sym0, __sym1, __sym2));
                        return Ok(__result);
                    }
                    Some((__loc1, (3, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state22(scale, input, __tokens, __sym1, __sym2));
                        return Ok(__result);
                    }
                    Some((__loc1, (4, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state23(scale, input, __tokens, __sym1, __sym2));
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

            // State 27
            //     AllInputs = [Expr, "+", Factor]
            //     OptionalInputs = [Expr, "+"]
            //     FixedInputs = [Factor]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Expr = Expr "+" Factor (*) [")", "+", "-"]
            //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
            //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
            //
            //   "*" -> S24
            //   "/" -> S25
            //   [")", "+", "-"] -> Expr = Expr, "+", Factor => ActionFn(2);
            //
            pub fn __state27<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, i32, usize)>,
                __sym1: &mut Option<(usize, &'input str, usize)>,
                __sym2: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state24(scale, input, __tokens, __sym2, __sym3));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state25(scale, input, __tokens, __sym2, __sym3));
                        return Ok(__result);
                    }
                    Some((_, (1, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2(scale, input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Expr((
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

            // State 28
            //     AllInputs = [Expr, "-", Factor]
            //     OptionalInputs = [Expr, "-"]
            //     FixedInputs = [Factor]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Expr = Expr "-" Factor (*) [")", "+", "-"]
            //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
            //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
            //
            //   "*" -> S24
            //   "/" -> S25
            //   [")", "+", "-"] -> Expr = Expr, "-", Factor => ActionFn(1);
            //
            pub fn __state28<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, i32, usize)>,
                __sym1: &mut Option<(usize, &'input str, usize)>,
                __sym2: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state24(scale, input, __tokens, __sym2, __sym3));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state25(scale, input, __tokens, __sym2, __sym3));
                        return Ok(__result);
                    }
                    Some((_, (1, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1(scale, input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Expr((
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

            // State 29
            //     AllInputs = [Factor, "*", Term]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "*", Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "*" Term (*) [")", "*", "+", "-", "/"]
            //
            //   [")", "*", "+", "-", "/"] -> Factor = Factor, "*", Term => ActionFn(4);
            //
            pub fn __state29<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, i32, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action4(scale, input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Factor((
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

            // State 30
            //     AllInputs = [Factor, "/", Term]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "/", Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "/" Term (*) [")", "*", "+", "-", "/"]
            //
            //   [")", "*", "+", "-", "/"] -> Factor = Factor, "/", Term => ActionFn(5);
            //
            pub fn __state30<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, i32, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, i32, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5(scale, input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Factor((
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

            // State 31
            //     AllInputs = ["(", Expr, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["(", Expr, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = "(" Expr ")" (*) [")", "*", "+", "-", "/"]
            //
            //   [")", "*", "+", "-", "/"] -> Term = "(", Expr, ")" => ActionFn(8);
            //
            pub fn __state31<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
            >(
                scale: i32,
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, i32, usize),
                __sym2: (usize, &'input str, usize),
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action8(scale, input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Term((
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
        pub use self::__parse__Expr::parse_Expr;
    }
    mod __parse_table {

        mod __parse__Expr {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use std::str::FromStr;
            use util::tok::Tok;
            extern crate lalrpop_util as __lalrpop_util;
            use self::__lalrpop_util::ParseError as __ParseError;
            #[allow(dead_code)]
            pub enum __Symbol<'input> {
                Term_22_28_22(&'input str),
                Term_22_29_22(&'input str),
                Term_22_2a_22(&'input str),
                Term_22_2b_22(&'input str),
                Term_22_2d_22(&'input str),
                Term_22_2f_22(&'input str),
                Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
                NtExpr(i32),
                NtFactor(i32),
                NtNum(i32),
                NtTerm(i32),
                Nt____Expr(i32),
            }
            const __ACTION: &'static [i32] = &[
                // State 0
                //     Expr = (*) Expr "+" Factor ["+"]
                //     Expr = (*) Expr "+" Factor ["-"]
                //     Expr = (*) Expr "+" Factor [EOF]
                //     Expr = (*) Expr "-" Factor ["+"]
                //     Expr = (*) Expr "-" Factor ["-"]
                //     Expr = (*) Expr "-" Factor [EOF]
                //     Expr = (*) Factor ["+"]
                //     Expr = (*) Factor ["-"]
                //     Expr = (*) Factor [EOF]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["+"]
                //     Factor = (*) Factor "*" Term ["-"]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "*" Term [EOF]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["+"]
                //     Factor = (*) Factor "/" Term ["-"]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Factor "/" Term [EOF]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["+"]
                //     Factor = (*) Term ["-"]
                //     Factor = (*) Term ["/"]
                //     Factor = (*) Term [EOF]
                //     Num = (*) r#"[0-9]+"# ["*"]
                //     Num = (*) r#"[0-9]+"# ["+"]
                //     Num = (*) r#"[0-9]+"# ["-"]
                //     Num = (*) r#"[0-9]+"# ["/"]
                //     Num = (*) r#"[0-9]+"# [EOF]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+"]
                //     Term = (*) Num ["-"]
                //     Term = (*) Num ["/"]
                //     Term = (*) Num [EOF]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) "(" Expr ")" [EOF]
                //     __Expr = (*) Expr [EOF]
                6, // on "(", goto 5
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                7, // on r#"[0-9]+"#, goto 6
                // State 1
                //     Expr = Expr (*) "+" Factor ["+", "-", EOF]
                //     Expr = Expr (*) "-" Factor ["+", "-", EOF]
                //     __Expr = Expr (*) [EOF]
                0, // on "(", error
                0, // on ")", error
                0, // on "*", error
                8, // on "+", goto 7
                9, // on "-", goto 8
                0, // on "/", error
                0, // on r#"[0-9]+"#, error
                // State 2
                //     Expr = Factor (*) ["+", "-", EOF]
                //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
                //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                10, // on "*", goto 9
                -3, // on "+", reduce `Expr = Factor => ActionFn(3);`
                -3, // on "-", reduce `Expr = Factor => ActionFn(3);`
                11, // on "/", goto 10
                0, // on r#"[0-9]+"#, error
                // State 3
                //     Term = Num (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -8, // on "*", reduce `Term = Num => ActionFn(7);`
                -8, // on "+", reduce `Term = Num => ActionFn(7);`
                -8, // on "-", reduce `Term = Num => ActionFn(7);`
                -8, // on "/", reduce `Term = Num => ActionFn(7);`
                0, // on r#"[0-9]+"#, error
                // State 4
                //     Factor = Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -6, // on "*", reduce `Factor = Term => ActionFn(6);`
                -6, // on "+", reduce `Factor = Term => ActionFn(6);`
                -6, // on "-", reduce `Factor = Term => ActionFn(6);`
                -6, // on "/", reduce `Factor = Term => ActionFn(6);`
                0, // on r#"[0-9]+"#, error
                // State 5
                //     Expr = (*) Expr "+" Factor [")"]
                //     Expr = (*) Expr "+" Factor ["+"]
                //     Expr = (*) Expr "+" Factor ["-"]
                //     Expr = (*) Expr "-" Factor [")"]
                //     Expr = (*) Expr "-" Factor ["+"]
                //     Expr = (*) Expr "-" Factor ["-"]
                //     Expr = (*) Factor [")"]
                //     Expr = (*) Factor ["+"]
                //     Expr = (*) Factor ["-"]
                //     Factor = (*) Factor "*" Term [")"]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["+"]
                //     Factor = (*) Factor "*" Term ["-"]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term [")"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["+"]
                //     Factor = (*) Factor "/" Term ["-"]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term [")"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["+"]
                //     Factor = (*) Term ["-"]
                //     Factor = (*) Term ["/"]
                //     Num = (*) r#"[0-9]+"# [")"]
                //     Num = (*) r#"[0-9]+"# ["*"]
                //     Num = (*) r#"[0-9]+"# ["+"]
                //     Num = (*) r#"[0-9]+"# ["-"]
                //     Num = (*) r#"[0-9]+"# ["/"]
                //     Term = (*) Num [")"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+"]
                //     Term = (*) Num ["-"]
                //     Term = (*) Num ["/"]
                //     Term = (*) "(" Expr ")" [")"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = "(" (*) Expr ")" ["*", "+", "-", "/", EOF]
                16, // on "(", goto 15
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                17, // on r#"[0-9]+"#, goto 16
                // State 6
                //     Num = r#"[0-9]+"# (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -7, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(9);`
                -7, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(9);`
                -7, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(9);`
                -7, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(9);`
                0, // on r#"[0-9]+"#, error
                // State 7
                //     Expr = Expr "+" (*) Factor ["+", "-", EOF]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["+", "-", EOF]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["+", "-", EOF]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["+", "-", EOF]
                //     Factor = (*) Term ["/"]
                //     Num = (*) r#"[0-9]+"# ["*"]
                //     Num = (*) r#"[0-9]+"# ["+", "-", EOF]
                //     Num = (*) r#"[0-9]+"# ["/"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+", "-", EOF]
                //     Term = (*) Num ["/"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+", "-", EOF]
                //     Term = (*) "(" Expr ")" ["/"]
                6, // on "(", goto 5
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                7, // on r#"[0-9]+"#, goto 6
                // State 8
                //     Expr = Expr "-" (*) Factor ["+", "-", EOF]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["+", "-", EOF]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["+", "-", EOF]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["+", "-", EOF]
                //     Factor = (*) Term ["/"]
                //     Num = (*) r#"[0-9]+"# ["*"]
                //     Num = (*) r#"[0-9]+"# ["+", "-", EOF]
                //     Num = (*) r#"[0-9]+"# ["/"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+", "-", EOF]
                //     Term = (*) Num ["/"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+", "-", EOF]
                //     Term = (*) "(" Expr ")" ["/"]
                6, // on "(", goto 5
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                7, // on r#"[0-9]+"#, goto 6
                // State 9
                //     Factor = Factor "*" (*) Term ["*", "+", "-", "/", EOF]
                //     Num = (*) r#"[0-9]+"# ["*", "+", "-", "/", EOF]
                //     Term = (*) Num ["*", "+", "-", "/", EOF]
                //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
                6, // on "(", goto 5
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                7, // on r#"[0-9]+"#, goto 6
                // State 10
                //     Factor = Factor "/" (*) Term ["*", "+", "-", "/", EOF]
                //     Num = (*) r#"[0-9]+"# ["*", "+", "-", "/", EOF]
                //     Term = (*) Num ["*", "+", "-", "/", EOF]
                //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
                6, // on "(", goto 5
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                7, // on r#"[0-9]+"#, goto 6
                // State 11
                //     Expr = Expr (*) "+" Factor [")", "+", "-"]
                //     Expr = Expr (*) "-" Factor [")", "+", "-"]
                //     Term = "(" Expr (*) ")" ["*", "+", "-", "/", EOF]
                0, // on "(", error
                22, // on ")", goto 21
                0, // on "*", error
                23, // on "+", goto 22
                24, // on "-", goto 23
                0, // on "/", error
                0, // on r#"[0-9]+"#, error
                // State 12
                //     Expr = Factor (*) [")", "+", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
                0, // on "(", error
                -3, // on ")", reduce `Expr = Factor => ActionFn(3);`
                25, // on "*", goto 24
                -3, // on "+", reduce `Expr = Factor => ActionFn(3);`
                -3, // on "-", reduce `Expr = Factor => ActionFn(3);`
                26, // on "/", goto 25
                0, // on r#"[0-9]+"#, error
                // State 13
                //     Term = Num (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -8, // on ")", reduce `Term = Num => ActionFn(7);`
                -8, // on "*", reduce `Term = Num => ActionFn(7);`
                -8, // on "+", reduce `Term = Num => ActionFn(7);`
                -8, // on "-", reduce `Term = Num => ActionFn(7);`
                -8, // on "/", reduce `Term = Num => ActionFn(7);`
                0, // on r#"[0-9]+"#, error
                // State 14
                //     Factor = Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -6, // on ")", reduce `Factor = Term => ActionFn(6);`
                -6, // on "*", reduce `Factor = Term => ActionFn(6);`
                -6, // on "+", reduce `Factor = Term => ActionFn(6);`
                -6, // on "-", reduce `Factor = Term => ActionFn(6);`
                -6, // on "/", reduce `Factor = Term => ActionFn(6);`
                0, // on r#"[0-9]+"#, error
                // State 15
                //     Expr = (*) Expr "+" Factor [")"]
                //     Expr = (*) Expr "+" Factor ["+"]
                //     Expr = (*) Expr "+" Factor ["-"]
                //     Expr = (*) Expr "-" Factor [")"]
                //     Expr = (*) Expr "-" Factor ["+"]
                //     Expr = (*) Expr "-" Factor ["-"]
                //     Expr = (*) Factor [")"]
                //     Expr = (*) Factor ["+"]
                //     Expr = (*) Factor ["-"]
                //     Factor = (*) Factor "*" Term [")"]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["+"]
                //     Factor = (*) Factor "*" Term ["-"]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term [")"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["+"]
                //     Factor = (*) Factor "/" Term ["-"]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term [")"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["+"]
                //     Factor = (*) Term ["-"]
                //     Factor = (*) Term ["/"]
                //     Num = (*) r#"[0-9]+"# [")"]
                //     Num = (*) r#"[0-9]+"# ["*"]
                //     Num = (*) r#"[0-9]+"# ["+"]
                //     Num = (*) r#"[0-9]+"# ["-"]
                //     Num = (*) r#"[0-9]+"# ["/"]
                //     Term = (*) Num [")"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+"]
                //     Term = (*) Num ["-"]
                //     Term = (*) Num ["/"]
                //     Term = (*) "(" Expr ")" [")"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = "(" (*) Expr ")" [")", "*", "+", "-", "/"]
                16, // on "(", goto 15
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                17, // on r#"[0-9]+"#, goto 16
                // State 16
                //     Num = r#"[0-9]+"# (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -7, // on ")", reduce `Num = r#"[0-9]+"# => ActionFn(9);`
                -7, // on "*", reduce `Num = r#"[0-9]+"# => ActionFn(9);`
                -7, // on "+", reduce `Num = r#"[0-9]+"# => ActionFn(9);`
                -7, // on "-", reduce `Num = r#"[0-9]+"# => ActionFn(9);`
                -7, // on "/", reduce `Num = r#"[0-9]+"# => ActionFn(9);`
                0, // on r#"[0-9]+"#, error
                // State 17
                //     Expr = Expr "+" Factor (*) ["+", "-", EOF]
                //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
                //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                10, // on "*", goto 9
                -2, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                -2, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                11, // on "/", goto 10
                0, // on r#"[0-9]+"#, error
                // State 18
                //     Expr = Expr "-" Factor (*) ["+", "-", EOF]
                //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
                //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                10, // on "*", goto 9
                -1, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                -1, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                11, // on "/", goto 10
                0, // on r#"[0-9]+"#, error
                // State 19
                //     Factor = Factor "*" Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -4, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                0, // on r#"[0-9]+"#, error
                // State 20
                //     Factor = Factor "/" Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -5, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                0, // on r#"[0-9]+"#, error
                // State 21
                //     Term = "(" Expr ")" (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -9, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -9, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -9, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -9, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                0, // on r#"[0-9]+"#, error
                // State 22
                //     Expr = Expr "+" (*) Factor [")", "+", "-"]
                //     Factor = (*) Factor "*" Term [")", "+", "-"]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term [")", "+", "-"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term [")", "+", "-"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["/"]
                //     Num = (*) r#"[0-9]+"# [")", "+", "-"]
                //     Num = (*) r#"[0-9]+"# ["*"]
                //     Num = (*) r#"[0-9]+"# ["/"]
                //     Term = (*) Num [")", "+", "-"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["/"]
                //     Term = (*) "(" Expr ")" [")", "+", "-"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["/"]
                16, // on "(", goto 15
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                17, // on r#"[0-9]+"#, goto 16
                // State 23
                //     Expr = Expr "-" (*) Factor [")", "+", "-"]
                //     Factor = (*) Factor "*" Term [")", "+", "-"]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term [")", "+", "-"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term [")", "+", "-"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["/"]
                //     Num = (*) r#"[0-9]+"# [")", "+", "-"]
                //     Num = (*) r#"[0-9]+"# ["*"]
                //     Num = (*) r#"[0-9]+"# ["/"]
                //     Term = (*) Num [")", "+", "-"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["/"]
                //     Term = (*) "(" Expr ")" [")", "+", "-"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["/"]
                16, // on "(", goto 15
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                17, // on r#"[0-9]+"#, goto 16
                // State 24
                //     Factor = Factor "*" (*) Term [")", "*", "+", "-", "/"]
                //     Num = (*) r#"[0-9]+"# [")", "*", "+", "-", "/"]
                //     Term = (*) Num [")", "*", "+", "-", "/"]
                //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
                16, // on "(", goto 15
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                17, // on r#"[0-9]+"#, goto 16
                // State 25
                //     Factor = Factor "/" (*) Term [")", "*", "+", "-", "/"]
                //     Num = (*) r#"[0-9]+"# [")", "*", "+", "-", "/"]
                //     Term = (*) Num [")", "*", "+", "-", "/"]
                //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
                16, // on "(", goto 15
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                17, // on r#"[0-9]+"#, goto 16
                // State 26
                //     Expr = Expr (*) "+" Factor [")", "+", "-"]
                //     Expr = Expr (*) "-" Factor [")", "+", "-"]
                //     Term = "(" Expr (*) ")" [")", "*", "+", "-", "/"]
                0, // on "(", error
                32, // on ")", goto 31
                0, // on "*", error
                23, // on "+", goto 22
                24, // on "-", goto 23
                0, // on "/", error
                0, // on r#"[0-9]+"#, error
                // State 27
                //     Expr = Expr "+" Factor (*) [")", "+", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
                0, // on "(", error
                -2, // on ")", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                25, // on "*", goto 24
                -2, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                -2, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                26, // on "/", goto 25
                0, // on r#"[0-9]+"#, error
                // State 28
                //     Expr = Expr "-" Factor (*) [")", "+", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
                0, // on "(", error
                -1, // on ")", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                25, // on "*", goto 24
                -1, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                -1, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                26, // on "/", goto 25
                0, // on r#"[0-9]+"#, error
                // State 29
                //     Factor = Factor "*" Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -4, // on ")", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                0, // on r#"[0-9]+"#, error
                // State 30
                //     Factor = Factor "/" Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -5, // on ")", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                0, // on r#"[0-9]+"#, error
                // State 31
                //     Term = "(" Expr ")" (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -9, // on ")", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -9, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -9, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -9, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -9, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                0, // on r#"[0-9]+"#, error
            ];
            const __EOF_ACTION: &'static [i32] = &[
                0, // on EOF, error
                -10, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
                -3, // on EOF, reduce `Expr = Factor => ActionFn(3);`
                -8, // on EOF, reduce `Term = Num => ActionFn(7);`
                -6, // on EOF, reduce `Factor = Term => ActionFn(6);`
                0, // on EOF, error
                -7, // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(9);`
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
                -2, // on EOF, reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                -1, // on EOF, reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                -4, // on EOF, reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -5, // on EOF, reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -9, // on EOF, reduce `Term = "(", Expr, ")" => ActionFn(8);`
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
            ];
            const __GOTO: &'static [i32] = &[
                // State 0
                2, // on Expr, goto 1
                3, // on Factor, goto 2
                4, // on Num, goto 3
                5, // on Term, goto 4
                0, // on __Expr, error
                // State 1
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 2
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 3
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 4
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 5
                12, // on Expr, goto 11
                13, // on Factor, goto 12
                14, // on Num, goto 13
                15, // on Term, goto 14
                0, // on __Expr, error
                // State 6
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 7
                0, // on Expr, error
                18, // on Factor, goto 17
                4, // on Num, goto 3
                5, // on Term, goto 4
                0, // on __Expr, error
                // State 8
                0, // on Expr, error
                19, // on Factor, goto 18
                4, // on Num, goto 3
                5, // on Term, goto 4
                0, // on __Expr, error
                // State 9
                0, // on Expr, error
                0, // on Factor, error
                4, // on Num, goto 3
                20, // on Term, goto 19
                0, // on __Expr, error
                // State 10
                0, // on Expr, error
                0, // on Factor, error
                4, // on Num, goto 3
                21, // on Term, goto 20
                0, // on __Expr, error
                // State 11
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 12
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 13
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 14
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 15
                27, // on Expr, goto 26
                13, // on Factor, goto 12
                14, // on Num, goto 13
                15, // on Term, goto 14
                0, // on __Expr, error
                // State 16
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 17
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 18
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 19
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 20
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 21
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 22
                0, // on Expr, error
                28, // on Factor, goto 27
                14, // on Num, goto 13
                15, // on Term, goto 14
                0, // on __Expr, error
                // State 23
                0, // on Expr, error
                29, // on Factor, goto 28
                14, // on Num, goto 13
                15, // on Term, goto 14
                0, // on __Expr, error
                // State 24
                0, // on Expr, error
                0, // on Factor, error
                14, // on Num, goto 13
                30, // on Term, goto 29
                0, // on __Expr, error
                // State 25
                0, // on Expr, error
                0, // on Factor, error
                14, // on Num, goto 13
                31, // on Term, goto 30
                0, // on __Expr, error
                // State 26
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 27
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 28
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 29
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 30
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
                // State 31
                0, // on Expr, error
                0, // on Factor, error
                0, // on Num, error
                0, // on Term, error
                0, // on __Expr, error
            ];
            pub fn parse_Expr<
                'input,
            >(
                scale: i32,
                input: &'input str,
            ) -> Result<i32, __ParseError<usize,(usize, &'input str),()>>
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
                        (_, (6, _), _) if true => 6,
                        _ => {
                            return Err(__ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: vec![],
                            });
                        }
                    };
                    loop {
                        let __state = *__states.last().unwrap() as usize;
                        let __action = __ACTION[__state * 7 + __integer];
                        if __action > 0 {
                            let __symbol = match __integer {
                                0 => match __lookahead.1 {
                                    (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                                    _ => unreachable!(),
                                },
                                1 => match __lookahead.1 {
                                    (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                                    _ => unreachable!(),
                                },
                                2 => match __lookahead.1 {
                                    (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                                    _ => unreachable!(),
                                },
                                3 => match __lookahead.1 {
                                    (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                                    _ => unreachable!(),
                                },
                                4 => match __lookahead.1 {
                                    (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                                    _ => unreachable!(),
                                },
                                5 => match __lookahead.1 {
                                    (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                                    _ => unreachable!(),
                                },
                                6 => match __lookahead.1 {
                                    (6, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            __states.push(__action - 1);
                            __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                            continue '__shift;
                        } else if __action < 0 {
                            if let Some(r) = __reduce(scale, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
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
                        if let Some(r) = __reduce(scale, input, __action, None, &mut __states, &mut __symbols) {
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
                scale: i32,
                input: &'input str,
                __action: i32,
                __lookahead_start: Option<&usize>,
                __states: &mut ::std::vec::Vec<i32>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
            ) -> Option<Result<i32,__ParseError<usize,(usize, &'input str),()>>>
            {
                let __nonterminal = match -__action {
                    1 => {
                        // Expr = Expr, "-", Factor => ActionFn(1);
                        let __sym2 = __pop_NtFactor(__symbols);
                        let __sym1 = __pop_Term_22_2d_22(__symbols);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1(scale, input, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                        0
                    }
                    2 => {
                        // Expr = Expr, "+", Factor => ActionFn(2);
                        let __sym2 = __pop_NtFactor(__symbols);
                        let __sym1 = __pop_Term_22_2b_22(__symbols);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2(scale, input, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                        0
                    }
                    3 => {
                        // Expr = Factor => ActionFn(3);
                        let __sym0 = __pop_NtFactor(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3(scale, input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                        0
                    }
                    4 => {
                        // Factor = Factor, "*", Term => ActionFn(4);
                        let __sym2 = __pop_NtTerm(__symbols);
                        let __sym1 = __pop_Term_22_2a_22(__symbols);
                        let __sym0 = __pop_NtFactor(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action4(scale, input, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                        1
                    }
                    5 => {
                        // Factor = Factor, "/", Term => ActionFn(5);
                        let __sym2 = __pop_NtTerm(__symbols);
                        let __sym1 = __pop_Term_22_2f_22(__symbols);
                        let __sym0 = __pop_NtFactor(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5(scale, input, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                        1
                    }
                    6 => {
                        // Factor = Term => ActionFn(6);
                        let __sym0 = __pop_NtTerm(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action6(scale, input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                        1
                    }
                    7 => {
                        // Num = r#"[0-9]+"# => ActionFn(9);
                        let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action9(scale, input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                        2
                    }
                    8 => {
                        // Term = Num => ActionFn(7);
                        let __sym0 = __pop_NtNum(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action7(scale, input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                        3
                    }
                    9 => {
                        // Term = "(", Expr, ")" => ActionFn(8);
                        let __sym2 = __pop_Term_22_29_22(__symbols);
                        let __sym1 = __pop_NtExpr(__symbols);
                        let __sym0 = __pop_Term_22_28_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action8(scale, input, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                        3
                    }
                    10 => {
                        // __Expr = Expr => ActionFn(0);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0(scale, input, __sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 5 + __nonterminal] - 1;
                __states.push(__next_state);
                None
            }
            fn __pop_Term_22_28_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_29_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2a_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
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
            fn __pop_Term_22_2f_22<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtExpr<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, i32, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtFactor<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, i32, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtNum<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, i32, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtTerm<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, i32, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____Expr<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, i32, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
        }
        pub use self::__parse__Expr::parse_Expr;
    }
}
pub use self::__parse__Expr::parse_Expr;
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
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
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
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    (__0)
}

pub fn __action1<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l - r
}

pub fn __action2<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l + r
}

pub fn __action3<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    (__0)
}

pub fn __action4<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l * r
}

pub fn __action5<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l / r
}

pub fn __action6<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    (__0)
}

pub fn __action7<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, n, _): (usize, i32, usize),
) -> i32
{
    n * scale
}

pub fn __action8<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
) -> i32
{
    (__0)
}

pub fn __action9<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
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
