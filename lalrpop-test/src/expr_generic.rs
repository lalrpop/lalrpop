use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::fmt::Debug;
    use std::ops::{Add, Div, Mul, Sub};
    use std::str::FromStr;
    extern crate lalrpop_util as __lalrpop_util;
    pub fn parse_Expr<
        'input,
        F,
    >(
        input: &'input str,
    ) -> Result<F, __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
    {
        let __ascent = __ascent::parse_Expr::<F>(
            input,
        );
        let __parse_table = __parse_table::parse_Expr::<F>(
            input,
        );
        assert_eq!(__ascent, __parse_table);
        return __ascent;
    }
    mod __ascent {

        mod __parse__Expr {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use std::fmt::Debug;
            use std::ops::{Add, Div, Mul, Sub};
            use std::str::FromStr;
            extern crate lalrpop_util as __lalrpop_util;
            pub fn parse_Expr<
                'input,
                F,
            >(
                input: &'input str,
            ) -> Result<F, __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __tokens = super::super::super::__intern_token::__Matcher::new(input);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match try!(__state0(input, &mut __tokens, __lookahead, ::std::marker::PhantomData::<(F)>)) {
                    (Some(__lookahead), _) => {
                        Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead })
                    }
                    (None, __Nonterminal::____Expr((_, __nt, _))) => {
                        Ok(__nt)
                    }
                    _ => unreachable!(),
                }
            }

            #[allow(dead_code)]
            pub enum __Nonterminal<F> {
                Expr((usize, F, usize)),
                Factor((usize, F, usize)),
                Term((usize, F, usize)),
                ____Expr((usize, F, usize)),
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
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+"]
            //     Term = (*) "(" Expr ")" ["-"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) "(" Expr ")" [EOF]
            //     Term = (*) r#"[0-9]+"# ["*"]
            //     Term = (*) r#"[0-9]+"# ["+"]
            //     Term = (*) r#"[0-9]+"# ["-"]
            //     Term = (*) r#"[0-9]+"# ["/"]
            //     Term = (*) r#"[0-9]+"# [EOF]
            //     __Expr = (*) Expr [EOF]
            //
            //   "(" -> S4
            //   r#"[0-9]+"# -> S5
            //
            //     Expr -> S1
            //     Factor -> S2
            //     Term -> S3
            pub fn __state0<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state4(input, __tokens, __sym0, ::std::marker::PhantomData::<(F)>));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(input, __tokens, __sym0, ::std::marker::PhantomData::<(F)>));
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
                        __Nonterminal::Expr(__sym0) => {
                            __result = try!(__state1(input, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<(F)>));
                        }
                        __Nonterminal::Factor(__sym0) => {
                            __result = try!(__state2(input, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<(F)>));
                        }
                        __Nonterminal::Term(__sym0) => {
                            __result = try!(__state3(input, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<(F)>));
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
            //   "+" -> S6
            //   "-" -> S7
            //   [EOF] -> __Expr = Expr => ActionFn(0);
            //
            pub fn __state1<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((__loc1, (3, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(input, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((__loc1, (4, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state7(input, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<F>(input, __sym0);
                        let __nt = __Nonterminal::____Expr((
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
            //   "*" -> S8
            //   "/" -> S9
            //   ["+", "-", EOF] -> Expr = Factor => ActionFn(3);
            //
            pub fn __state2<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state8(input, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state9(input, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<F>(input, __sym0);
                        let __nt = __Nonterminal::Expr((
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
            pub fn __state3<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action6::<F>(input, __sym0);
                        let __nt = __Nonterminal::Factor((
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
            //     Term = (*) "(" Expr ")" [")"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+"]
            //     Term = (*) "(" Expr ")" ["-"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = "(" (*) Expr ")" ["*", "+", "-", "/", EOF]
            //     Term = (*) r#"[0-9]+"# [")"]
            //     Term = (*) r#"[0-9]+"# ["*"]
            //     Term = (*) r#"[0-9]+"# ["+"]
            //     Term = (*) r#"[0-9]+"# ["-"]
            //     Term = (*) r#"[0-9]+"# ["/"]
            //
            //   "(" -> S13
            //   r#"[0-9]+"# -> S14
            //
            //     Expr -> S10
            //     Factor -> S11
            //     Term -> S12
            pub fn __state4<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state13(input, __tokens, __sym1, ::std::marker::PhantomData::<(F)>));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(input, __tokens, __sym1, ::std::marker::PhantomData::<(F)>));
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
                        __Nonterminal::Expr(__sym1) => {
                            __result = try!(__state10(input, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<(F)>));
                        }
                        __Nonterminal::Factor(__sym1) => {
                            __result = try!(__state11(input, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<(F)>));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state12(input, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<(F)>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 5
            //     AllInputs = [r#"[0-9]+"#]
            //     OptionalInputs = []
            //     FixedInputs = [r#"[0-9]+"#]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = r#"[0-9]+"# (*) ["*", "+", "-", "/", EOF]
            //
            //   ["*", "+", "-", "/", EOF] -> Term = r#"[0-9]+"# => ActionFn(7);
            //
            pub fn __state5<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
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
                        let __nt = super::super::super::__action7::<F>(input, __sym0);
                        let __nt = __Nonterminal::Term((
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
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+", "-", EOF]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) r#"[0-9]+"# ["*"]
            //     Term = (*) r#"[0-9]+"# ["+", "-", EOF]
            //     Term = (*) r#"[0-9]+"# ["/"]
            //
            //   "(" -> S4
            //   r#"[0-9]+"# -> S5
            //
            //     Factor -> S15
            //     Term -> S3
            pub fn __state6<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
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
                        __result = try!(__state4(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
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
                        __Nonterminal::Factor(__sym2) => {
                            __result = try!(__state15(input, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state3(input, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<(F)>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 7
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
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+", "-", EOF]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) r#"[0-9]+"# ["*"]
            //     Term = (*) r#"[0-9]+"# ["+", "-", EOF]
            //     Term = (*) r#"[0-9]+"# ["/"]
            //
            //   "(" -> S4
            //   r#"[0-9]+"# -> S5
            //
            //     Factor -> S16
            //     Term -> S3
            pub fn __state7<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
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
                        __result = try!(__state4(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
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
                        __Nonterminal::Factor(__sym2) => {
                            __result = try!(__state16(input, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state3(input, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<(F)>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 8
            //     AllInputs = [Factor, "*"]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "*"]
            //     WillPushLen = 1
            //     WillPush = [Term]
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "*" (*) Term ["*", "+", "-", "/", EOF]
            //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
            //     Term = (*) r#"[0-9]+"# ["*", "+", "-", "/", EOF]
            //
            //   "(" -> S4
            //   r#"[0-9]+"# -> S5
            //
            //     Term -> S17
            pub fn __state8<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state4(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
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
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state17(input, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 9
            //     AllInputs = [Factor, "/"]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "/"]
            //     WillPushLen = 1
            //     WillPush = [Term]
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "/" (*) Term ["*", "+", "-", "/", EOF]
            //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
            //     Term = (*) r#"[0-9]+"# ["*", "+", "-", "/", EOF]
            //
            //   "(" -> S4
            //   r#"[0-9]+"# -> S5
            //
            //     Term -> S18
            pub fn __state9<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state4(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
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
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state18(input, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 10
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
            //   ")" -> S19
            //   "+" -> S20
            //   "-" -> S21
            //
            pub fn __state10<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, &'input str, usize)>,
                __sym1: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state19(input, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((__loc1, (3, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state20(input, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((__loc1, (4, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state21(input, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
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
            //   "*" -> S22
            //   "/" -> S23
            //   [")", "+", "-"] -> Expr = Factor => ActionFn(3);
            //
            pub fn __state11<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state22(input, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state23(input, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((_, (1, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<F>(input, __sym0);
                        let __nt = __Nonterminal::Expr((
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
            pub fn __state12<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action6::<F>(input, __sym0);
                        let __nt = __Nonterminal::Factor((
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
            //     Term = (*) "(" Expr ")" [")"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+"]
            //     Term = (*) "(" Expr ")" ["-"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = "(" (*) Expr ")" [")", "*", "+", "-", "/"]
            //     Term = (*) r#"[0-9]+"# [")"]
            //     Term = (*) r#"[0-9]+"# ["*"]
            //     Term = (*) r#"[0-9]+"# ["+"]
            //     Term = (*) r#"[0-9]+"# ["-"]
            //     Term = (*) r#"[0-9]+"# ["/"]
            //
            //   "(" -> S13
            //   r#"[0-9]+"# -> S14
            //
            //     Expr -> S24
            //     Factor -> S11
            //     Term -> S12
            pub fn __state13<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state13(input, __tokens, __sym1, ::std::marker::PhantomData::<(F)>));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(input, __tokens, __sym1, ::std::marker::PhantomData::<(F)>));
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
                        __Nonterminal::Expr(__sym1) => {
                            __result = try!(__state24(input, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<(F)>));
                        }
                        __Nonterminal::Factor(__sym1) => {
                            __result = try!(__state11(input, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<(F)>));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state12(input, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<(F)>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 14
            //     AllInputs = [r#"[0-9]+"#]
            //     OptionalInputs = []
            //     FixedInputs = [r#"[0-9]+"#]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = r#"[0-9]+"# (*) [")", "*", "+", "-", "/"]
            //
            //   [")", "*", "+", "-", "/"] -> Term = r#"[0-9]+"# => ActionFn(7);
            //
            pub fn __state14<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
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
                        let __nt = super::super::super::__action7::<F>(input, __sym0);
                        let __nt = __Nonterminal::Term((
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

            // State 15
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
            //   "*" -> S8
            //   "/" -> S9
            //   ["+", "-", EOF] -> Expr = Expr, "+", Factor => ActionFn(2);
            //
            pub fn __state15<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, F, usize)>,
                __sym1: &mut Option<(usize, &'input str, usize)>,
                __sym2: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state8(input, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state9(input, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    None => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<F>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Expr((
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
            //   "*" -> S8
            //   "/" -> S9
            //   ["+", "-", EOF] -> Expr = Expr, "-", Factor => ActionFn(1);
            //
            pub fn __state16<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, F, usize)>,
                __sym1: &mut Option<(usize, &'input str, usize)>,
                __sym2: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state8(input, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state9(input, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    None => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1::<F>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Expr((
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

            // State 17
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
            pub fn __state17<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action4::<F>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Factor((
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
            pub fn __state18<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5::<F>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Factor((
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
            pub fn __state19<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, F, usize),
                __sym2: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
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
                        let __nt = super::super::super::__action8::<F>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Term((
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
            //     Term = (*) "(" Expr ")" [")", "+", "-"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) r#"[0-9]+"# [")", "+", "-"]
            //     Term = (*) r#"[0-9]+"# ["*"]
            //     Term = (*) r#"[0-9]+"# ["/"]
            //
            //   "(" -> S13
            //   r#"[0-9]+"# -> S14
            //
            //     Factor -> S25
            //     Term -> S12
            pub fn __state20<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
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
                        __result = try!(__state13(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
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
                        __Nonterminal::Factor(__sym2) => {
                            __result = try!(__state25(input, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state12(input, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<(F)>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 21
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
            //     Term = (*) "(" Expr ")" [")", "+", "-"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) r#"[0-9]+"# [")", "+", "-"]
            //     Term = (*) r#"[0-9]+"# ["*"]
            //     Term = (*) r#"[0-9]+"# ["/"]
            //
            //   "(" -> S13
            //   r#"[0-9]+"# -> S14
            //
            //     Factor -> S26
            //     Term -> S12
            pub fn __state21<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
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
                        __result = try!(__state13(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
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
                        __Nonterminal::Factor(__sym2) => {
                            __result = try!(__state26(input, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state12(input, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<(F)>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 22
            //     AllInputs = [Factor, "*"]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "*"]
            //     WillPushLen = 1
            //     WillPush = [Term]
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "*" (*) Term [")", "*", "+", "-", "/"]
            //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
            //     Term = (*) r#"[0-9]+"# [")", "*", "+", "-", "/"]
            //
            //   "(" -> S13
            //   r#"[0-9]+"# -> S14
            //
            //     Term -> S27
            pub fn __state22<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state13(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
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
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state27(input, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 23
            //     AllInputs = [Factor, "/"]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "/"]
            //     WillPushLen = 1
            //     WillPush = [Term]
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "/" (*) Term [")", "*", "+", "-", "/"]
            //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
            //     Term = (*) r#"[0-9]+"# [")", "*", "+", "-", "/"]
            //
            //   "(" -> S13
            //   r#"[0-9]+"# -> S14
            //
            //     Term -> S28
            pub fn __state23<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, (0, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state13(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
                    }
                    Some((__loc1, (6, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(input, __tokens, __sym2, ::std::marker::PhantomData::<(F)>));
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
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state28(input, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 24
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
            //   ")" -> S29
            //   "+" -> S20
            //   "-" -> S21
            //
            pub fn __state24<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, &'input str, usize)>,
                __sym1: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((__loc1, (1, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state29(input, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((__loc1, (3, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state20(input, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((__loc1, (4, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state21(input, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<(F)>));
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
            //   "*" -> S22
            //   "/" -> S23
            //   [")", "+", "-"] -> Expr = Expr, "+", Factor => ActionFn(2);
            //
            pub fn __state25<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, F, usize)>,
                __sym1: &mut Option<(usize, &'input str, usize)>,
                __sym2: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state22(input, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state23(input, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((_, (1, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<F>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Expr((
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

            // State 26
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
            //   "*" -> S22
            //   "/" -> S23
            //   [")", "+", "-"] -> Expr = Expr, "-", Factor => ActionFn(1);
            //
            pub fn __state26<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: &mut Option<(usize, F, usize)>,
                __sym1: &mut Option<(usize, &'input str, usize)>,
                __sym2: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((__loc1, (2, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state22(input, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((__loc1, (5, __tok0), __loc2)) => {
                        let __sym3 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state23(input, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<(F)>));
                        return Ok(__result);
                    }
                    Some((_, (1, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1::<F>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Expr((
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

            // State 27
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
            pub fn __state27<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action4::<F>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Factor((
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

            // State 28
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
            pub fn __state28<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, (usize, &'input str), usize)>,
                __sym0: (usize, F, usize),
                __sym1: (usize, &'input str, usize),
                __sym2: (usize, F, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
                match __lookahead {
                    Some((_, (1, _), _)) |
                    Some((_, (2, _), _)) |
                    Some((_, (3, _), _)) |
                    Some((_, (4, _), _)) |
                    Some((_, (5, _), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5::<F>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Factor((
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

            // State 29
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
            pub fn __state29<
                'input,
                F,
                __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, F, usize),
                __sym2: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<(F)>,
            ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
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
                        let __nt = super::super::super::__action8::<F>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Term((
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
        pub use self::__parse__Expr::parse_Expr;
    }
    mod __parse_table {

        mod __parse__Expr {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use std::fmt::Debug;
            use std::ops::{Add, Div, Mul, Sub};
            use std::str::FromStr;
            extern crate lalrpop_util as __lalrpop_util;
            #[allow(dead_code)]
            pub enum __Symbol<'input, F> {
                Term_22_28_22(&'input str),
                Term_22_29_22(&'input str),
                Term_22_2a_22(&'input str),
                Term_22_2b_22(&'input str),
                Term_22_2d_22(&'input str),
                Term_22_2f_22(&'input str),
                Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
                NtExpr(F),
                NtFactor(F),
                NtTerm(F),
                Nt____Expr(F),
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
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) "(" Expr ")" [EOF]
                //     Term = (*) r#"[0-9]+"# ["*"]
                //     Term = (*) r#"[0-9]+"# ["+"]
                //     Term = (*) r#"[0-9]+"# ["-"]
                //     Term = (*) r#"[0-9]+"# ["/"]
                //     Term = (*) r#"[0-9]+"# [EOF]
                //     __Expr = (*) Expr [EOF]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                6, // on r#"[0-9]+"#, goto 5
                // State 1
                //     Expr = Expr (*) "+" Factor ["+", "-", EOF]
                //     Expr = Expr (*) "-" Factor ["+", "-", EOF]
                //     __Expr = Expr (*) [EOF]
                0, // on "(", error
                0, // on ")", error
                0, // on "*", error
                7, // on "+", goto 6
                8, // on "-", goto 7
                0, // on "/", error
                0, // on r#"[0-9]+"#, error
                // State 2
                //     Expr = Factor (*) ["+", "-", EOF]
                //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
                //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                9, // on "*", goto 8
                -3, // on "+", reduce `Expr = Factor => ActionFn(3);`
                -3, // on "-", reduce `Expr = Factor => ActionFn(3);`
                10, // on "/", goto 9
                0, // on r#"[0-9]+"#, error
                // State 3
                //     Factor = Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -6, // on "*", reduce `Factor = Term => ActionFn(6);`
                -6, // on "+", reduce `Factor = Term => ActionFn(6);`
                -6, // on "-", reduce `Factor = Term => ActionFn(6);`
                -6, // on "/", reduce `Factor = Term => ActionFn(6);`
                0, // on r#"[0-9]+"#, error
                // State 4
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
                //     Term = (*) "(" Expr ")" [")"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = "(" (*) Expr ")" ["*", "+", "-", "/", EOF]
                //     Term = (*) r#"[0-9]+"# [")"]
                //     Term = (*) r#"[0-9]+"# ["*"]
                //     Term = (*) r#"[0-9]+"# ["+"]
                //     Term = (*) r#"[0-9]+"# ["-"]
                //     Term = (*) r#"[0-9]+"# ["/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on r#"[0-9]+"#, goto 14
                // State 5
                //     Term = r#"[0-9]+"# (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -7, // on "*", reduce `Term = r#"[0-9]+"# => ActionFn(7);`
                -7, // on "+", reduce `Term = r#"[0-9]+"# => ActionFn(7);`
                -7, // on "-", reduce `Term = r#"[0-9]+"# => ActionFn(7);`
                -7, // on "/", reduce `Term = r#"[0-9]+"# => ActionFn(7);`
                0, // on r#"[0-9]+"#, error
                // State 6
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
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+", "-", EOF]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) r#"[0-9]+"# ["*"]
                //     Term = (*) r#"[0-9]+"# ["+", "-", EOF]
                //     Term = (*) r#"[0-9]+"# ["/"]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                6, // on r#"[0-9]+"#, goto 5
                // State 7
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
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+", "-", EOF]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) r#"[0-9]+"# ["*"]
                //     Term = (*) r#"[0-9]+"# ["+", "-", EOF]
                //     Term = (*) r#"[0-9]+"# ["/"]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                6, // on r#"[0-9]+"#, goto 5
                // State 8
                //     Factor = Factor "*" (*) Term ["*", "+", "-", "/", EOF]
                //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
                //     Term = (*) r#"[0-9]+"# ["*", "+", "-", "/", EOF]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                6, // on r#"[0-9]+"#, goto 5
                // State 9
                //     Factor = Factor "/" (*) Term ["*", "+", "-", "/", EOF]
                //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
                //     Term = (*) r#"[0-9]+"# ["*", "+", "-", "/", EOF]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                6, // on r#"[0-9]+"#, goto 5
                // State 10
                //     Expr = Expr (*) "+" Factor [")", "+", "-"]
                //     Expr = Expr (*) "-" Factor [")", "+", "-"]
                //     Term = "(" Expr (*) ")" ["*", "+", "-", "/", EOF]
                0, // on "(", error
                20, // on ")", goto 19
                0, // on "*", error
                21, // on "+", goto 20
                22, // on "-", goto 21
                0, // on "/", error
                0, // on r#"[0-9]+"#, error
                // State 11
                //     Expr = Factor (*) [")", "+", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
                0, // on "(", error
                -3, // on ")", reduce `Expr = Factor => ActionFn(3);`
                23, // on "*", goto 22
                -3, // on "+", reduce `Expr = Factor => ActionFn(3);`
                -3, // on "-", reduce `Expr = Factor => ActionFn(3);`
                24, // on "/", goto 23
                0, // on r#"[0-9]+"#, error
                // State 12
                //     Factor = Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -6, // on ")", reduce `Factor = Term => ActionFn(6);`
                -6, // on "*", reduce `Factor = Term => ActionFn(6);`
                -6, // on "+", reduce `Factor = Term => ActionFn(6);`
                -6, // on "-", reduce `Factor = Term => ActionFn(6);`
                -6, // on "/", reduce `Factor = Term => ActionFn(6);`
                0, // on r#"[0-9]+"#, error
                // State 13
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
                //     Term = (*) "(" Expr ")" [")"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = "(" (*) Expr ")" [")", "*", "+", "-", "/"]
                //     Term = (*) r#"[0-9]+"# [")"]
                //     Term = (*) r#"[0-9]+"# ["*"]
                //     Term = (*) r#"[0-9]+"# ["+"]
                //     Term = (*) r#"[0-9]+"# ["-"]
                //     Term = (*) r#"[0-9]+"# ["/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on r#"[0-9]+"#, goto 14
                // State 14
                //     Term = r#"[0-9]+"# (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -7, // on ")", reduce `Term = r#"[0-9]+"# => ActionFn(7);`
                -7, // on "*", reduce `Term = r#"[0-9]+"# => ActionFn(7);`
                -7, // on "+", reduce `Term = r#"[0-9]+"# => ActionFn(7);`
                -7, // on "-", reduce `Term = r#"[0-9]+"# => ActionFn(7);`
                -7, // on "/", reduce `Term = r#"[0-9]+"# => ActionFn(7);`
                0, // on r#"[0-9]+"#, error
                // State 15
                //     Expr = Expr "+" Factor (*) ["+", "-", EOF]
                //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
                //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                9, // on "*", goto 8
                -2, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                -2, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                10, // on "/", goto 9
                0, // on r#"[0-9]+"#, error
                // State 16
                //     Expr = Expr "-" Factor (*) ["+", "-", EOF]
                //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
                //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                9, // on "*", goto 8
                -1, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                -1, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                10, // on "/", goto 9
                0, // on r#"[0-9]+"#, error
                // State 17
                //     Factor = Factor "*" Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -4, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                0, // on r#"[0-9]+"#, error
                // State 18
                //     Factor = Factor "/" Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -5, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                0, // on r#"[0-9]+"#, error
                // State 19
                //     Term = "(" Expr ")" (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -8, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                0, // on r#"[0-9]+"#, error
                // State 20
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
                //     Term = (*) "(" Expr ")" [")", "+", "-"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) r#"[0-9]+"# [")", "+", "-"]
                //     Term = (*) r#"[0-9]+"# ["*"]
                //     Term = (*) r#"[0-9]+"# ["/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on r#"[0-9]+"#, goto 14
                // State 21
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
                //     Term = (*) "(" Expr ")" [")", "+", "-"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) r#"[0-9]+"# [")", "+", "-"]
                //     Term = (*) r#"[0-9]+"# ["*"]
                //     Term = (*) r#"[0-9]+"# ["/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on r#"[0-9]+"#, goto 14
                // State 22
                //     Factor = Factor "*" (*) Term [")", "*", "+", "-", "/"]
                //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
                //     Term = (*) r#"[0-9]+"# [")", "*", "+", "-", "/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on r#"[0-9]+"#, goto 14
                // State 23
                //     Factor = Factor "/" (*) Term [")", "*", "+", "-", "/"]
                //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
                //     Term = (*) r#"[0-9]+"# [")", "*", "+", "-", "/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on r#"[0-9]+"#, goto 14
                // State 24
                //     Expr = Expr (*) "+" Factor [")", "+", "-"]
                //     Expr = Expr (*) "-" Factor [")", "+", "-"]
                //     Term = "(" Expr (*) ")" [")", "*", "+", "-", "/"]
                0, // on "(", error
                30, // on ")", goto 29
                0, // on "*", error
                21, // on "+", goto 20
                22, // on "-", goto 21
                0, // on "/", error
                0, // on r#"[0-9]+"#, error
                // State 25
                //     Expr = Expr "+" Factor (*) [")", "+", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
                0, // on "(", error
                -2, // on ")", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                23, // on "*", goto 22
                -2, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                -2, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                24, // on "/", goto 23
                0, // on r#"[0-9]+"#, error
                // State 26
                //     Expr = Expr "-" Factor (*) [")", "+", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
                0, // on "(", error
                -1, // on ")", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                23, // on "*", goto 22
                -1, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                -1, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                24, // on "/", goto 23
                0, // on r#"[0-9]+"#, error
                // State 27
                //     Factor = Factor "*" Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -4, // on ")", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                0, // on r#"[0-9]+"#, error
                // State 28
                //     Factor = Factor "/" Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -5, // on ")", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                0, // on r#"[0-9]+"#, error
                // State 29
                //     Term = "(" Expr ")" (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -8, // on ")", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                0, // on r#"[0-9]+"#, error
            ];
            const __EOF_ACTION: &'static [i32] = &[
                0, // on EOF, error
                -9, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
                -3, // on EOF, reduce `Expr = Factor => ActionFn(3);`
                -6, // on EOF, reduce `Factor = Term => ActionFn(6);`
                0, // on EOF, error
                -7, // on EOF, reduce `Term = r#"[0-9]+"# => ActionFn(7);`
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
                -8, // on EOF, reduce `Term = "(", Expr, ")" => ActionFn(8);`
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
                4, // on Term, goto 3
                0, // on __Expr, error
                // State 1
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 2
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 3
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 4
                11, // on Expr, goto 10
                12, // on Factor, goto 11
                13, // on Term, goto 12
                0, // on __Expr, error
                // State 5
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 6
                0, // on Expr, error
                16, // on Factor, goto 15
                4, // on Term, goto 3
                0, // on __Expr, error
                // State 7
                0, // on Expr, error
                17, // on Factor, goto 16
                4, // on Term, goto 3
                0, // on __Expr, error
                // State 8
                0, // on Expr, error
                0, // on Factor, error
                18, // on Term, goto 17
                0, // on __Expr, error
                // State 9
                0, // on Expr, error
                0, // on Factor, error
                19, // on Term, goto 18
                0, // on __Expr, error
                // State 10
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 11
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 12
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 13
                25, // on Expr, goto 24
                12, // on Factor, goto 11
                13, // on Term, goto 12
                0, // on __Expr, error
                // State 14
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 15
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 16
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 17
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 18
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 19
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 20
                0, // on Expr, error
                26, // on Factor, goto 25
                13, // on Term, goto 12
                0, // on __Expr, error
                // State 21
                0, // on Expr, error
                27, // on Factor, goto 26
                13, // on Term, goto 12
                0, // on __Expr, error
                // State 22
                0, // on Expr, error
                0, // on Factor, error
                28, // on Term, goto 27
                0, // on __Expr, error
                // State 23
                0, // on Expr, error
                0, // on Factor, error
                29, // on Term, goto 28
                0, // on __Expr, error
                // State 24
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 25
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 26
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 27
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 28
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 29
                0, // on Expr, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
            ];
            pub fn parse_Expr<
                'input,
                F,
            >(
                input: &'input str,
            ) -> Result<F, __lalrpop_util::ParseError<usize,(usize, &'input str),()>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
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
                            return Err(__lalrpop_util::ParseError::UnrecognizedToken {
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
                            if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<(F)>) {
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
                        if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<(F)>) {
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
                F,
            >(
                input: &'input str,
                __action: i32,
                __lookahead_start: Option<&usize>,
                __states: &mut ::std::vec::Vec<i32>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>,
                _: ::std::marker::PhantomData<(F)>,
            ) -> Option<Result<F,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>> where
              F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
            {
                let __nonterminal = match -__action {
                    1 => {
                        // Expr = Expr, "-", Factor => ActionFn(1);
                        let __sym2 = __pop_NtFactor(__symbols);
                        let __sym1 = __pop_Term_22_2d_22(__symbols);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1::<F>(input, __sym0, __sym1, __sym2);
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
                        let __nt = super::super::super::__action2::<F>(input, __sym0, __sym1, __sym2);
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
                        let __nt = super::super::super::__action3::<F>(input, __sym0);
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
                        let __nt = super::super::super::__action4::<F>(input, __sym0, __sym1, __sym2);
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
                        let __nt = super::super::super::__action5::<F>(input, __sym0, __sym1, __sym2);
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
                        let __nt = super::super::super::__action6::<F>(input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                        1
                    }
                    7 => {
                        // Term = r#"[0-9]+"# => ActionFn(7);
                        let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action7::<F>(input, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                        2
                    }
                    8 => {
                        // Term = "(", Expr, ")" => ActionFn(8);
                        let __sym2 = __pop_Term_22_29_22(__symbols);
                        let __sym1 = __pop_NtExpr(__symbols);
                        let __sym0 = __pop_Term_22_28_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action8::<F>(input, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                        2
                    }
                    9 => {
                        // __Expr = Expr => ActionFn(0);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<F>(input, __sym0);
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
              'input,
              F,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_29_22<
              'input,
              F,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2a_22<
              'input,
              F,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2b_22<
              'input,
              F,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2d_22<
              'input,
              F,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2f_22<
              'input,
              F,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
              'input,
              F,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>
            ) -> (usize, &'input str, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtExpr<
              'input,
              F,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>
            ) -> (usize, F, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtFactor<
              'input,
              F,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>
            ) -> (usize, F, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtTerm<
              'input,
              F,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>
            ) -> (usize, F, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____Expr<
              'input,
              F,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input, F>,usize)>
            ) -> (usize, F, usize) {
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
    F,
>(
    input: &'input str,
    (_, __0, _): (usize, F, usize),
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
    F,
>(
    input: &'input str,
    (_, l, _): (usize, F, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, F, usize),
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
{
    l - r
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
    F,
>(
    input: &'input str,
    (_, l, _): (usize, F, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, F, usize),
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
{
    l + r
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
    F,
>(
    input: &'input str,
    (_, __0, _): (usize, F, usize),
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
    F,
>(
    input: &'input str,
    (_, l, _): (usize, F, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, F, usize),
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
{
    l * r
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
    F,
>(
    input: &'input str,
    (_, l, _): (usize, F, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, F, usize),
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
{
    l / r
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
    F,
>(
    input: &'input str,
    (_, __0, _): (usize, F, usize),
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
    F,
>(
    input: &'input str,
    (_, n, _): (usize, &'input str, usize),
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
{
    match n.parse() { Ok(v) => v, Err(_) => panic!("can't parse") }
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
    F,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, F, usize),
    (_, _, _): (usize, &'input str, usize),
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F> + PartialEq + Eq,
{
    (__0)
}

pub trait __ToTriple<'input, F, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, F, > __ToTriple<'input, F, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, F, > __ToTriple<'input, F, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
