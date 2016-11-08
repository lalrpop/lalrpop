use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    pub fn parse_Expr<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        scale: i32,
        __tokens0: __TOKENS,
    ) -> Result<i32, __lalrpop_util::ParseError<(),Tok,()>> where
      __TOKENS: Clone,
    {
        let __ascent = __ascent::parse_Expr(
            scale,
            __tokens0.clone(),
        );
        let __parse_table = __parse_table::parse_Expr(
            scale,
            __tokens0.clone(),
        );
        assert_eq!(__ascent, __parse_table);
        return __ascent;
    }
    mod __ascent {

        mod __parse__Expr {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use util::tok::Tok;
            extern crate lalrpop_util as __lalrpop_util;
            use super::super::super::__ToTriple;
            pub fn parse_Expr<
                __TOKEN: __ToTriple<Error=()>,
                __TOKENS: IntoIterator<Item=__TOKEN>,
            >(
                scale: i32,
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
                match try!(__state0(scale, &mut __tokens, __lookahead, ::std::marker::PhantomData::<()>)) {
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
            pub enum __Nonterminal<> {
                Expr(((), i32, ())),
                Factor(((), i32, ())),
                Term(((), i32, ())),
                ____Expr(((), i32, ())),
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
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+"]
            //     Term = (*) Num ["-"]
            //     Term = (*) Num ["/"]
            //     Term = (*) Num [EOF]
            //     __Expr = (*) Expr [EOF]
            //
            //   "(" -> S4
            //   Num -> S5
            //
            //     Expr -> S1
            //     Factor -> S2
            //     Term -> S3
            pub fn __state0<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym0 = (__loc1, (__tok), __loc2);
                        __result = try!(__state4(scale, __tokens, __sym0, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(scale, __tokens, __sym0, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state1(scale, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Factor(__sym0) => {
                            __result = try!(__state2(scale, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym0) => {
                            __result = try!(__state3(scale, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state6(scale, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state7(scale, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<>(scale, __sym0);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state8(scale, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state9(scale, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<>(scale, __sym0);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action6::<>(scale, __sym0);
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
            //     Term = (*) Num [")"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+"]
            //     Term = (*) Num ["-"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S13
            //   Num -> S14
            //
            //     Expr -> S10
            //     Factor -> S11
            //     Term -> S12
            pub fn __state4<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
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
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state13(scale, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(scale, __tokens, __sym1, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state10(scale, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Factor(__sym1) => {
                            __result = try!(__state11(scale, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state12(scale, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
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
            //     WillProduce = Some(Term)
            //
            //     Term = Num (*) ["*", "+", "-", "/", EOF]
            //
            //   ["*", "+", "-", "/", EOF] -> Term = Num => ActionFn(7);
            //
            pub fn __state5<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
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
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action7::<>(scale, __sym0);
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
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+", "-", EOF]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S4
            //   Num -> S5
            //
            //     Factor -> S15
            //     Term -> S3
            pub fn __state6<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
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
                let __sym0 = &mut Some(__sym0);
                let __sym1 = &mut Some(__sym1);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state4(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state15(scale, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state3(scale, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
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
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+", "-", EOF]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S4
            //   Num -> S5
            //
            //     Factor -> S16
            //     Term -> S3
            pub fn __state7<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
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
                let __sym0 = &mut Some(__sym0);
                let __sym1 = &mut Some(__sym1);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state4(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state16(scale, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state3(scale, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
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
            //     Term = (*) Num ["*", "+", "-", "/", EOF]
            //
            //   "(" -> S4
            //   Num -> S5
            //
            //     Term -> S17
            pub fn __state8<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
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
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state4(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state17(scale, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
            //     Term = (*) Num ["*", "+", "-", "/", EOF]
            //
            //   "(" -> S4
            //   Num -> S5
            //
            //     Term -> S18
            pub fn __state9<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
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
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state4(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state18(scale, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: &mut Option<((), Tok, ())>,
                __sym1: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state19(scale, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state20(scale, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state21(scale, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state22(scale, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state23(scale, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<>(scale, __sym0);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action6::<>(scale, __sym0);
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
            //     Term = (*) Num [")"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+"]
            //     Term = (*) Num ["-"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S13
            //   Num -> S14
            //
            //     Expr -> S24
            //     Factor -> S11
            //     Term -> S12
            pub fn __state13<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
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
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state13(scale, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(scale, __tokens, __sym1, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state24(scale, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Factor(__sym1) => {
                            __result = try!(__state11(scale, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state12(scale, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 14
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
            pub fn __state14<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
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
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action7::<>(scale, __sym0);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: &mut Option<((), i32, ())>,
                __sym1: &mut Option<((), Tok, ())>,
                __sym2: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state8(scale, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state9(scale, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<>(scale, __sym0, __sym1, __sym2);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: &mut Option<((), i32, ())>,
                __sym1: &mut Option<((), Tok, ())>,
                __sym2: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state8(scale, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state9(scale, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1::<>(scale, __sym0, __sym1, __sym2);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                __sym1: ((), Tok, ()),
                __sym2: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action4::<>(scale, __sym0, __sym1, __sym2);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                __sym1: ((), Tok, ()),
                __sym2: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5::<>(scale, __sym0, __sym1, __sym2);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), Tok, ()),
                __sym1: ((), i32, ()),
                __sym2: ((), Tok, ()),
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
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action8::<>(scale, __sym0, __sym1, __sym2);
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
            //     Term = (*) Num [")", "+", "-"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S13
            //   Num -> S14
            //
            //     Factor -> S25
            //     Term -> S12
            pub fn __state20<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
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
                let __sym0 = &mut Some(__sym0);
                let __sym1 = &mut Some(__sym1);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state13(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state25(scale, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state12(scale, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
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
            //     Term = (*) Num [")", "+", "-"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S13
            //   Num -> S14
            //
            //     Factor -> S26
            //     Term -> S12
            pub fn __state21<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
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
                let __sym0 = &mut Some(__sym0);
                let __sym1 = &mut Some(__sym1);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state13(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state26(scale, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state12(scale, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
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
            //     Term = (*) Num [")", "*", "+", "-", "/"]
            //
            //   "(" -> S13
            //   Num -> S14
            //
            //     Term -> S27
            pub fn __state22<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
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
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state13(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state27(scale, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
            //     Term = (*) Num [")", "*", "+", "-", "/"]
            //
            //   "(" -> S13
            //   Num -> S14
            //
            //     Term -> S28
            pub fn __state23<
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), i32, ()),
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
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state13(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state14(scale, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state28(scale, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: &mut Option<((), Tok, ())>,
                __sym1: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state29(scale, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state20(scale, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state21(scale, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: &mut Option<((), i32, ())>,
                __sym1: &mut Option<((), Tok, ())>,
                __sym2: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state22(scale, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state23(scale, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<>(scale, __sym0, __sym1, __sym2);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: &mut Option<((), i32, ())>,
                __sym1: &mut Option<((), Tok, ())>,
                __sym2: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state22(scale, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state23(scale, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1::<>(scale, __sym0, __sym1, __sym2);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                __sym1: ((), Tok, ()),
                __sym2: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action4::<>(scale, __sym0, __sym1, __sym2);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __lookahead: Option<((), Tok, ())>,
                __sym0: ((), i32, ()),
                __sym1: ((), Tok, ()),
                __sym2: ((), i32, ()),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __lalrpop_util::ParseError<(),Tok,()>>
            {
                let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5::<>(scale, __sym0, __sym1, __sym2);
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
                __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
            >(
                scale: i32,
                __tokens: &mut __TOKENS,
                __sym0: ((), Tok, ()),
                __sym1: ((), i32, ()),
                __sym2: ((), Tok, ()),
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
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action8::<>(scale, __sym0, __sym1, __sym2);
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

            use util::tok::Tok;
            extern crate lalrpop_util as __lalrpop_util;
            use super::super::super::__ToTriple;
            #[allow(dead_code)]
            pub enum __Symbol<> {
                Term_22_28_22(Tok),
                Term_22_29_22(Tok),
                Term_22_2a_22(Tok),
                Term_22_2b_22(Tok),
                Term_22_2d_22(Tok),
                Term_22_2f_22(Tok),
                TermNum(i32),
                NtExpr(i32),
                NtFactor(i32),
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
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) "(" Expr ")" [EOF]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+"]
                //     Term = (*) Num ["-"]
                //     Term = (*) Num ["/"]
                //     Term = (*) Num [EOF]
                //     __Expr = (*) Expr [EOF]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                6, // on Num, goto 5
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
                0, // on Num, error
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
                0, // on Num, error
                // State 3
                //     Factor = Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -6, // on "*", reduce `Factor = Term => ActionFn(6);`
                -6, // on "+", reduce `Factor = Term => ActionFn(6);`
                -6, // on "-", reduce `Factor = Term => ActionFn(6);`
                -6, // on "/", reduce `Factor = Term => ActionFn(6);`
                0, // on Num, error
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
                //     Term = (*) Num [")"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+"]
                //     Term = (*) Num ["-"]
                //     Term = (*) Num ["/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on Num, goto 14
                // State 5
                //     Term = Num (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -7, // on "*", reduce `Term = Num => ActionFn(7);`
                -7, // on "+", reduce `Term = Num => ActionFn(7);`
                -7, // on "-", reduce `Term = Num => ActionFn(7);`
                -7, // on "/", reduce `Term = Num => ActionFn(7);`
                0, // on Num, error
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
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+", "-", EOF]
                //     Term = (*) Num ["/"]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                6, // on Num, goto 5
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
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+", "-", EOF]
                //     Term = (*) Num ["/"]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                6, // on Num, goto 5
                // State 8
                //     Factor = Factor "*" (*) Term ["*", "+", "-", "/", EOF]
                //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
                //     Term = (*) Num ["*", "+", "-", "/", EOF]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                6, // on Num, goto 5
                // State 9
                //     Factor = Factor "/" (*) Term ["*", "+", "-", "/", EOF]
                //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
                //     Term = (*) Num ["*", "+", "-", "/", EOF]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                6, // on Num, goto 5
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
                0, // on Num, error
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
                0, // on Num, error
                // State 12
                //     Factor = Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -6, // on ")", reduce `Factor = Term => ActionFn(6);`
                -6, // on "*", reduce `Factor = Term => ActionFn(6);`
                -6, // on "+", reduce `Factor = Term => ActionFn(6);`
                -6, // on "-", reduce `Factor = Term => ActionFn(6);`
                -6, // on "/", reduce `Factor = Term => ActionFn(6);`
                0, // on Num, error
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
                //     Term = (*) Num [")"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+"]
                //     Term = (*) Num ["-"]
                //     Term = (*) Num ["/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on Num, goto 14
                // State 14
                //     Term = Num (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -7, // on ")", reduce `Term = Num => ActionFn(7);`
                -7, // on "*", reduce `Term = Num => ActionFn(7);`
                -7, // on "+", reduce `Term = Num => ActionFn(7);`
                -7, // on "-", reduce `Term = Num => ActionFn(7);`
                -7, // on "/", reduce `Term = Num => ActionFn(7);`
                0, // on Num, error
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
                0, // on Num, error
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
                0, // on Num, error
                // State 17
                //     Factor = Factor "*" Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -4, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                0, // on Num, error
                // State 18
                //     Factor = Factor "/" Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -5, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                0, // on Num, error
                // State 19
                //     Term = "(" Expr ")" (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -8, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                0, // on Num, error
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
                //     Term = (*) Num [")", "+", "-"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on Num, goto 14
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
                //     Term = (*) Num [")", "+", "-"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on Num, goto 14
                // State 22
                //     Factor = Factor "*" (*) Term [")", "*", "+", "-", "/"]
                //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
                //     Term = (*) Num [")", "*", "+", "-", "/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on Num, goto 14
                // State 23
                //     Factor = Factor "/" (*) Term [")", "*", "+", "-", "/"]
                //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
                //     Term = (*) Num [")", "*", "+", "-", "/"]
                14, // on "(", goto 13
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on "-", error
                0, // on "/", error
                15, // on Num, goto 14
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
                0, // on Num, error
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
                0, // on Num, error
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
                0, // on Num, error
                // State 27
                //     Factor = Factor "*" Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -4, // on ")", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -4, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                0, // on Num, error
                // State 28
                //     Factor = Factor "/" Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -5, // on ")", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -5, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                0, // on Num, error
                // State 29
                //     Term = "(" Expr ")" (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -8, // on ")", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                -8, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(8);`
                0, // on Num, error
            ];
            const __EOF_ACTION: &'static [i32] = &[
                0, // on EOF, error
                -9, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
                -3, // on EOF, reduce `Expr = Factor => ActionFn(3);`
                -6, // on EOF, reduce `Factor = Term => ActionFn(6);`
                0, // on EOF, error
                -7, // on EOF, reduce `Term = Num => ActionFn(7);`
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
                __TOKEN: __ToTriple<Error=()>,
                __TOKENS: IntoIterator<Item=__TOKEN>,
            >(
                scale: i32,
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
                        (_, Tok::Times, _) if true => 2,
                        (_, Tok::Plus, _) if true => 3,
                        (_, Tok::Minus, _) if true => 4,
                        (_, Tok::Div, _) if true => 5,
                        (_, Tok::Num(_), _) if true => 6,
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
                                    __tok @ Tok::LParen => __Symbol::Term_22_28_22(__tok),
                                    _ => unreachable!(),
                                },
                                1 => match __lookahead.1 {
                                    __tok @ Tok::RParen => __Symbol::Term_22_29_22(__tok),
                                    _ => unreachable!(),
                                },
                                2 => match __lookahead.1 {
                                    __tok @ Tok::Times => __Symbol::Term_22_2a_22(__tok),
                                    _ => unreachable!(),
                                },
                                3 => match __lookahead.1 {
                                    __tok @ Tok::Plus => __Symbol::Term_22_2b_22(__tok),
                                    _ => unreachable!(),
                                },
                                4 => match __lookahead.1 {
                                    __tok @ Tok::Minus => __Symbol::Term_22_2d_22(__tok),
                                    _ => unreachable!(),
                                },
                                5 => match __lookahead.1 {
                                    __tok @ Tok::Div => __Symbol::Term_22_2f_22(__tok),
                                    _ => unreachable!(),
                                },
                                6 => match __lookahead.1 {
                                    Tok::Num(__tok0) => __Symbol::TermNum(__tok0),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            __states.push(__action - 1);
                            __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                            continue '__shift;
                        } else if __action < 0 {
                            if let Some(r) = __reduce(scale, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                        if let Some(r) = __reduce(scale, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                scale: i32,
                __action: i32,
                __lookahead_start: Option<&()>,
                __states: &mut ::std::vec::Vec<i32>,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
                _: ::std::marker::PhantomData<()>,
            ) -> Option<Result<i32,__lalrpop_util::ParseError<(),Tok,()>>>
            {
                let __nonterminal = match -__action {
                    1 => {
                        // Expr = Expr, "-", Factor => ActionFn(1);
                        let __sym2 = __pop_NtFactor(__symbols);
                        let __sym1 = __pop_Term_22_2d_22(__symbols);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1::<>(scale, __sym0, __sym1, __sym2);
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
                        let __nt = super::super::super::__action2::<>(scale, __sym0, __sym1, __sym2);
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
                        let __nt = super::super::super::__action3::<>(scale, __sym0);
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
                        let __nt = super::super::super::__action4::<>(scale, __sym0, __sym1, __sym2);
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
                        let __nt = super::super::super::__action5::<>(scale, __sym0, __sym1, __sym2);
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
                        let __nt = super::super::super::__action6::<>(scale, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                        1
                    }
                    7 => {
                        // Term = Num => ActionFn(7);
                        let __sym0 = __pop_TermNum(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action7::<>(scale, __sym0);
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
                        let __nt = super::super::super::__action8::<>(scale, __sym0, __sym1, __sym2);
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
                        let __nt = super::super::super::__action0::<>(scale, __sym0);
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
                scale: i32,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> ((), Tok, ())
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_29_22<
            >(
                scale: i32,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> ((), Tok, ())
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2a_22<
            >(
                scale: i32,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> ((), Tok, ())
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2b_22<
            >(
                scale: i32,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> ((), Tok, ())
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2d_22<
            >(
                scale: i32,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> ((), Tok, ())
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2f_22<
            >(
                scale: i32,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> ((), Tok, ())
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_TermNum<
            >(
                scale: i32,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> ((), i32, ())
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::TermNum(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtExpr<
            >(
                scale: i32,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> ((), i32, ())
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtFactor<
            >(
                scale: i32,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> ((), i32, ())
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtTerm<
            >(
                scale: i32,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> ((), i32, ())
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____Expr<
            >(
                scale: i32,
                __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
            ) -> ((), i32, ())
            {
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

#[allow(unused_variables)]
pub fn __action0<
>(
    scale: i32,
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
>(
    scale: i32,
    (_, l, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    (_, r, _): ((), i32, ()),
) -> i32
{
    l - r
}

#[allow(unused_variables)]
pub fn __action2<
>(
    scale: i32,
    (_, l, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    (_, r, _): ((), i32, ()),
) -> i32
{
    l + r
}

#[allow(unused_variables)]
pub fn __action3<
>(
    scale: i32,
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action4<
>(
    scale: i32,
    (_, l, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    (_, r, _): ((), i32, ()),
) -> i32
{
    l * r
}

#[allow(unused_variables)]
pub fn __action5<
>(
    scale: i32,
    (_, l, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    (_, r, _): ((), i32, ()),
) -> i32
{
    l / r
}

#[allow(unused_variables)]
pub fn __action6<
>(
    scale: i32,
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action7<
>(
    scale: i32,
    (_, n, _): ((), i32, ()),
) -> i32
{
    n * scale
}

#[allow(unused_variables)]
pub fn __action8<
>(
    scale: i32,
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
