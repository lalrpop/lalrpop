use expr_arena_ast::{Arena, Node, Op};
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use expr_arena_ast::{Arena, Node, Op};
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    pub fn parse_Expr<
        'ast,
        __TOKEN: __ToTriple<'ast, Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        arena: &'ast Arena<'ast>,
        __tokens0: __TOKENS,
    ) -> Result<&'ast Node<'ast>, __lalrpop_util::ParseError<usize,Tok,()>> where
      __TOKENS: Clone,
    {
        let __ascent = __ascent::parse_Expr(
            arena,
            __tokens0.clone(),
        );
        let __parse_table = __parse_table::parse_Expr(
            arena,
            __tokens0.clone(),
        );
        assert_eq!(__ascent, __parse_table);
        return __ascent;
    }
    mod __ascent {

        mod __parse__Expr {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use expr_arena_ast::{Arena, Node, Op};
            use util::tok::Tok;
            extern crate lalrpop_util as __lalrpop_util;
            use super::super::super::__ToTriple;
            pub fn parse_Expr<
                'ast,
                __TOKEN: __ToTriple<'ast, Error=()>,
                __TOKENS: IntoIterator<Item=__TOKEN>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens0: __TOKENS,
            ) -> Result<&'ast Node<'ast>, __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let __tokens = __tokens0.into_iter();
                let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match try!(__state0(arena, &mut __tokens, __lookahead, ::std::marker::PhantomData::<()>)) {
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
            pub enum __Nonterminal<'ast> {
                _28_3cExpr_3e_20_22_2c_22_29((usize, &'ast Node<'ast>, usize)),
                _28_3cExpr_3e_20_22_2c_22_29_2a((usize, ::std::vec::Vec<&'ast Node<'ast>>, usize)),
                _28_3cExpr_3e_20_22_2c_22_29_2b((usize, ::std::vec::Vec<&'ast Node<'ast>>, usize)),
                Comma_3cExpr_3e((usize, Vec<&'ast Node<'ast>>, usize)),
                Expr((usize, &'ast Node<'ast>, usize)),
                Expr_3f((usize, ::std::option::Option<&'ast Node<'ast>>, usize)),
                Factor((usize, &'ast Node<'ast>, usize)),
                Term((usize, &'ast Node<'ast>, usize)),
                ____Expr((usize, &'ast Node<'ast>, usize)),
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
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" [EOF]
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
            //   "*" -> S5
            //   Num -> S6
            //
            //     Expr -> S1
            //     Factor -> S2
            //     Term -> S3
            pub fn __state0<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym0 = (__loc1, (__tok), __loc2);
                        __result = try!(__state4(arena, __tokens, __sym0, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym0 = (__loc1, (__tok), __loc2);
                        __result = try!(__state5(arena, __tokens, __sym0, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(arena, __tokens, __sym0, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state1(arena, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Factor(__sym0) => {
                            __result = try!(__state2(arena, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym0) => {
                            __result = try!(__state3(arena, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
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
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state7(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state8(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<>(arena, __sym0);
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
            //   "*" -> S9
            //   "/" -> S10
            //   ["+", "-", EOF] -> Expr = Factor => ActionFn(3);
            //
            pub fn __state2<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state9(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state10(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<>(arena, __sym0);
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
            //   ["*", "+", "-", "/", EOF] -> Factor = Term => ActionFn(7);
            //
            pub fn __state3<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action7::<>(arena, __sym0);
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
            //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
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
            //   "(" -> S14
            //   "*" -> S15
            //   Num -> S16
            //
            //     Expr -> S11
            //     Factor -> S12
            //     Term -> S13
            pub fn __state4<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state14(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state15(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state11(arena, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Factor(__sym1) => {
                            __result = try!(__state12(arena, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state13(arena, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 5
            //     AllInputs = ["*"]
            //     OptionalInputs = []
            //     FixedInputs = ["*"]
            //     WillPushLen = 3
            //     WillPush = ["(", Comma<Expr>, ")"]
            //     WillProduce = Some(Factor)
            //
            //     Factor = "*" (*) "(" Comma<Expr> ")" ["*", "+", "-", "/", EOF]
            //
            //   "(" -> S17
            //
            pub fn __state5<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state17(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
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
            //     AllInputs = [Num]
            //     OptionalInputs = []
            //     FixedInputs = [Num]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = Num (*) ["*", "+", "-", "/", EOF]
            //
            //   ["*", "+", "-", "/", EOF] -> Term = Num => ActionFn(8);
            //
            pub fn __state6<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, i32, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        let __nt = super::super::super::__action8::<>(arena, __sym0);
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
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["+", "-", EOF]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+", "-", EOF]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+", "-", EOF]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S4
            //   "*" -> S5
            //   Num -> S6
            //
            //     Factor -> S18
            //     Term -> S3
            pub fn __state7<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        __result = try!(__state4(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state5(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state18(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state3(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
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
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["+", "-", EOF]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+", "-", EOF]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+", "-", EOF]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S4
            //   "*" -> S5
            //   Num -> S6
            //
            //     Factor -> S19
            //     Term -> S3
            pub fn __state8<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        __result = try!(__state4(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state5(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state19(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state3(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
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
            //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
            //     Term = (*) Num ["*", "+", "-", "/", EOF]
            //
            //   "(" -> S4
            //   Num -> S6
            //
            //     Term -> S20
            pub fn __state9<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state4(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state20(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
            //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
            //     Term = (*) Num ["*", "+", "-", "/", EOF]
            //
            //   "(" -> S4
            //   Num -> S6
            //
            //     Term -> S21
            pub fn __state10<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state4(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state21(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
            //   ")" -> S22
            //   "+" -> S23
            //   "-" -> S24
            //
            pub fn __state11<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: &mut Option<(usize, Tok, usize)>,
                __sym1: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state22(arena, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state23(arena, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state24(arena, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
            //   "*" -> S25
            //   "/" -> S26
            //   [")", "+", "-"] -> Expr = Factor => ActionFn(3);
            //
            pub fn __state12<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state25(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state26(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<>(arena, __sym0);
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

            // State 13
            //     AllInputs = [Term]
            //     OptionalInputs = []
            //     FixedInputs = [Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = Term (*) [")", "*", "+", "-", "/"]
            //
            //   [")", "*", "+", "-", "/"] -> Factor = Term => ActionFn(7);
            //
            pub fn __state13<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action7::<>(arena, __sym0);
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

            // State 14
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
            //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
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
            //   "(" -> S14
            //   "*" -> S15
            //   Num -> S16
            //
            //     Expr -> S27
            //     Factor -> S12
            //     Term -> S13
            pub fn __state14<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state14(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state15(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state27(arena, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Factor(__sym1) => {
                            __result = try!(__state12(arena, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state13(arena, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 15
            //     AllInputs = ["*"]
            //     OptionalInputs = []
            //     FixedInputs = ["*"]
            //     WillPushLen = 3
            //     WillPush = ["(", Comma<Expr>, ")"]
            //     WillProduce = Some(Factor)
            //
            //     Factor = "*" (*) "(" Comma<Expr> ")" [")", "*", "+", "-", "/"]
            //
            //   "(" -> S28
            //
            pub fn __state15<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state28(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
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
            //     AllInputs = [Num]
            //     OptionalInputs = []
            //     FixedInputs = [Num]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = Num (*) [")", "*", "+", "-", "/"]
            //
            //   [")", "*", "+", "-", "/"] -> Term = Num => ActionFn(8);
            //
            pub fn __state16<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, i32, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        let __nt = super::super::super::__action8::<>(arena, __sym0);
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

            // State 17
            //     AllInputs = ["*", "("]
            //     OptionalInputs = []
            //     FixedInputs = ["*", "("]
            //     WillPushLen = 2
            //     WillPush = [Comma<Expr>, ")"]
            //     WillProduce = Some(Factor)
            //
            //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," ["(", "*", Num]
            //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," [")"]
            //     (<Expr> ",")+ = (*) Expr "," ["(", "*", Num]
            //     (<Expr> ",")+ = (*) Expr "," [")"]
            //     Comma<Expr> = (*) [")"]
            //     Comma<Expr> = (*) (<Expr> ",")+ [")"]
            //     Comma<Expr> = (*) (<Expr> ",")+ Expr [")"]
            //     Comma<Expr> = (*) Expr [")"]
            //     Expr = (*) Expr "+" Factor [")"]
            //     Expr = (*) Expr "+" Factor ["+"]
            //     Expr = (*) Expr "+" Factor [","]
            //     Expr = (*) Expr "+" Factor ["-"]
            //     Expr = (*) Expr "-" Factor [")"]
            //     Expr = (*) Expr "-" Factor ["+"]
            //     Expr = (*) Expr "-" Factor [","]
            //     Expr = (*) Expr "-" Factor ["-"]
            //     Expr = (*) Factor [")"]
            //     Expr = (*) Factor ["+"]
            //     Expr = (*) Factor [","]
            //     Expr = (*) Factor ["-"]
            //     Factor = (*) Factor "*" Term [")"]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["+"]
            //     Factor = (*) Factor "*" Term [","]
            //     Factor = (*) Factor "*" Term ["-"]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term [")"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["+"]
            //     Factor = (*) Factor "/" Term [","]
            //     Factor = (*) Factor "/" Term ["-"]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term [")"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["+"]
            //     Factor = (*) Term [","]
            //     Factor = (*) Term ["-"]
            //     Factor = (*) Term ["/"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Factor = "*" "(" (*) Comma<Expr> ")" ["*", "+", "-", "/", EOF]
            //     Term = (*) "(" Expr ")" [")"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+"]
            //     Term = (*) "(" Expr ")" [","]
            //     Term = (*) "(" Expr ")" ["-"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) Num [")"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+"]
            //     Term = (*) Num [","]
            //     Term = (*) Num ["-"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S34
            //   "*" -> S35
            //   Num -> S36
            //   [")"] -> Comma<Expr> =  => ActionFn(23);
            //
            //     (<Expr> ",")+ -> S29
            //     Comma<Expr> -> S30
            //     Expr -> S31
            //     Factor -> S32
            //     Term -> S33
            pub fn __state17<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state34(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state35(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state36(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((_, Tok::RParen, _)) => {
                        let __start = __sym1.2.clone();
                        let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action23::<>(arena, &__start, &__end);
                        let __nt = __Nonterminal::Comma_3cExpr_3e((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
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
                        __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2b(__sym2) => {
                            __result = try!(__state29(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Comma_3cExpr_3e(__sym2) => {
                            __result = try!(__state30(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        __Nonterminal::Expr(__sym2) => {
                            __result = try!(__state31(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Factor(__sym2) => {
                            __result = try!(__state32(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state33(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 18
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
            pub fn __state18<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: &mut Option<(usize, &'ast Node<'ast>, usize)>,
                __sym1: &mut Option<(usize, Tok, usize)>,
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state9(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state10(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<>(arena, __sym0, __sym1, __sym2);
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

            // State 19
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
            pub fn __state19<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: &mut Option<(usize, &'ast Node<'ast>, usize)>,
                __sym1: &mut Option<(usize, Tok, usize)>,
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state9(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state10(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1::<>(arena, __sym0, __sym1, __sym2);
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

            // State 20
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
            pub fn __state20<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action4::<>(arena, __sym0, __sym1, __sym2);
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

            // State 21
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
            pub fn __state21<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5::<>(arena, __sym0, __sym1, __sym2);
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

            // State 22
            //     AllInputs = ["(", Expr, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["(", Expr, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = "(" Expr ")" (*) ["*", "+", "-", "/", EOF]
            //
            //   ["*", "+", "-", "/", EOF] -> Term = "(", Expr, ")" => ActionFn(9);
            //
            pub fn __state22<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, &'ast Node<'ast>, usize),
                __sym2: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        let __nt = super::super::super::__action9::<>(arena, __sym0, __sym1, __sym2);
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

            // State 23
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
            //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", "-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Term = (*) "(" Expr ")" [")", "+", "-"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) Num [")", "+", "-"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S14
            //   "*" -> S15
            //   Num -> S16
            //
            //     Factor -> S37
            //     Term -> S13
            pub fn __state23<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        __result = try!(__state14(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state15(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state37(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state13(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 24
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
            //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", "-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Term = (*) "(" Expr ")" [")", "+", "-"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) Num [")", "+", "-"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S14
            //   "*" -> S15
            //   Num -> S16
            //
            //     Factor -> S38
            //     Term -> S13
            pub fn __state24<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        __result = try!(__state14(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state15(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state38(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state13(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 25
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
            //   "(" -> S14
            //   Num -> S16
            //
            //     Term -> S39
            pub fn __state25<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state14(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state39(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 26
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
            //   "(" -> S14
            //   Num -> S16
            //
            //     Term -> S40
            pub fn __state26<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state14(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state40(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 27
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
            //   ")" -> S41
            //   "+" -> S23
            //   "-" -> S24
            //
            pub fn __state27<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: &mut Option<(usize, Tok, usize)>,
                __sym1: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state41(arena, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state23(arena, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state24(arena, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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
            //     AllInputs = ["*", "("]
            //     OptionalInputs = []
            //     FixedInputs = ["*", "("]
            //     WillPushLen = 2
            //     WillPush = [Comma<Expr>, ")"]
            //     WillProduce = Some(Factor)
            //
            //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," ["(", "*", Num]
            //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," [")"]
            //     (<Expr> ",")+ = (*) Expr "," ["(", "*", Num]
            //     (<Expr> ",")+ = (*) Expr "," [")"]
            //     Comma<Expr> = (*) [")"]
            //     Comma<Expr> = (*) (<Expr> ",")+ [")"]
            //     Comma<Expr> = (*) (<Expr> ",")+ Expr [")"]
            //     Comma<Expr> = (*) Expr [")"]
            //     Expr = (*) Expr "+" Factor [")"]
            //     Expr = (*) Expr "+" Factor ["+"]
            //     Expr = (*) Expr "+" Factor [","]
            //     Expr = (*) Expr "+" Factor ["-"]
            //     Expr = (*) Expr "-" Factor [")"]
            //     Expr = (*) Expr "-" Factor ["+"]
            //     Expr = (*) Expr "-" Factor [","]
            //     Expr = (*) Expr "-" Factor ["-"]
            //     Expr = (*) Factor [")"]
            //     Expr = (*) Factor ["+"]
            //     Expr = (*) Factor [","]
            //     Expr = (*) Factor ["-"]
            //     Factor = (*) Factor "*" Term [")"]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["+"]
            //     Factor = (*) Factor "*" Term [","]
            //     Factor = (*) Factor "*" Term ["-"]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term [")"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["+"]
            //     Factor = (*) Factor "/" Term [","]
            //     Factor = (*) Factor "/" Term ["-"]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term [")"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["+"]
            //     Factor = (*) Term [","]
            //     Factor = (*) Term ["-"]
            //     Factor = (*) Term ["/"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Factor = "*" "(" (*) Comma<Expr> ")" [")", "*", "+", "-", "/"]
            //     Term = (*) "(" Expr ")" [")"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+"]
            //     Term = (*) "(" Expr ")" [","]
            //     Term = (*) "(" Expr ")" ["-"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) Num [")"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+"]
            //     Term = (*) Num [","]
            //     Term = (*) Num ["-"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S34
            //   "*" -> S35
            //   Num -> S36
            //   [")"] -> Comma<Expr> =  => ActionFn(23);
            //
            //     (<Expr> ",")+ -> S29
            //     Comma<Expr> -> S42
            //     Expr -> S31
            //     Factor -> S32
            //     Term -> S33
            pub fn __state28<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state34(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state35(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state36(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((_, Tok::RParen, _)) => {
                        let __start = __sym1.2.clone();
                        let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action23::<>(arena, &__start, &__end);
                        let __nt = __Nonterminal::Comma_3cExpr_3e((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
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
                        __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2b(__sym2) => {
                            __result = try!(__state29(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Comma_3cExpr_3e(__sym2) => {
                            __result = try!(__state42(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        __Nonterminal::Expr(__sym2) => {
                            __result = try!(__state31(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Factor(__sym2) => {
                            __result = try!(__state32(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state33(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 29
            //     AllInputs = [(<Expr> ",")+]
            //     OptionalInputs = []
            //     FixedInputs = [(<Expr> ",")+]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     (<Expr> ",")+ = (<Expr> ",")+ (*) Expr "," ["(", ")", "*", Num]
            //     Comma<Expr> = (<Expr> ",")+ (*) [")"]
            //     Comma<Expr> = (<Expr> ",")+ (*) Expr [")"]
            //     Expr = (*) Expr "+" Factor [")"]
            //     Expr = (*) Expr "+" Factor ["+"]
            //     Expr = (*) Expr "+" Factor [","]
            //     Expr = (*) Expr "+" Factor ["-"]
            //     Expr = (*) Expr "-" Factor [")"]
            //     Expr = (*) Expr "-" Factor ["+"]
            //     Expr = (*) Expr "-" Factor [","]
            //     Expr = (*) Expr "-" Factor ["-"]
            //     Expr = (*) Factor [")"]
            //     Expr = (*) Factor ["+"]
            //     Expr = (*) Factor [","]
            //     Expr = (*) Factor ["-"]
            //     Factor = (*) Factor "*" Term [")"]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["+"]
            //     Factor = (*) Factor "*" Term [","]
            //     Factor = (*) Factor "*" Term ["-"]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term [")"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["+"]
            //     Factor = (*) Factor "/" Term [","]
            //     Factor = (*) Factor "/" Term ["-"]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term [")"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["+"]
            //     Factor = (*) Term [","]
            //     Factor = (*) Term ["-"]
            //     Factor = (*) Term ["/"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Term = (*) "(" Expr ")" [")"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+"]
            //     Term = (*) "(" Expr ")" [","]
            //     Term = (*) "(" Expr ")" ["-"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) Num [")"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+"]
            //     Term = (*) Num [","]
            //     Term = (*) Num ["-"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S34
            //   "*" -> S35
            //   Num -> S36
            //   [")"] -> Comma<Expr> = (<Expr> ",")+ => ActionFn(25);
            //
            //     Expr -> S43
            //     Factor -> S32
            //     Term -> S33
            pub fn __state29<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state34(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state35(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state36(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((_, Tok::RParen, _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action25::<>(arena, __sym0);
                        let __nt = __Nonterminal::Comma_3cExpr_3e((
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
                    if __sym0.is_none() {
                        return Ok(__result);
                    }
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::Expr(__sym1) => {
                            __result = try!(__state43(arena, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Factor(__sym1) => {
                            __result = try!(__state32(arena, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state33(arena, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 30
            //     AllInputs = ["*", "(", Comma<Expr>]
            //     OptionalInputs = []
            //     FixedInputs = ["*", "(", Comma<Expr>]
            //     WillPushLen = 1
            //     WillPush = [")"]
            //     WillProduce = Some(Factor)
            //
            //     Factor = "*" "(" Comma<Expr> (*) ")" ["*", "+", "-", "/", EOF]
            //
            //   ")" -> S44
            //
            pub fn __state30<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, Vec<&'ast Node<'ast>>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state44(arena, __tokens, __sym0, __sym1, __sym2, __sym3, ::std::marker::PhantomData::<()>));
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

            // State 31
            //     AllInputs = [Expr]
            //     OptionalInputs = []
            //     FixedInputs = [Expr]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     (<Expr> ",")+ = Expr (*) "," ["(", ")", "*", Num]
            //     Comma<Expr> = Expr (*) [")"]
            //     Expr = Expr (*) "+" Factor [")", "+", ",", "-"]
            //     Expr = Expr (*) "-" Factor [")", "+", ",", "-"]
            //
            //   "+" -> S45
            //   "," -> S46
            //   "-" -> S47
            //   [")"] -> Comma<Expr> = Expr => ActionFn(22);
            //
            pub fn __state31<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state45(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Comma, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state46(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state47(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::RParen, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action22::<>(arena, __sym0);
                        let __nt = __Nonterminal::Comma_3cExpr_3e((
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

            // State 32
            //     AllInputs = [Factor]
            //     OptionalInputs = []
            //     FixedInputs = [Factor]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Expr = Factor (*) [")", "+", ",", "-"]
            //     Factor = Factor (*) "*" Term [")", "*", "+", ",", "-", "/"]
            //     Factor = Factor (*) "/" Term [")", "*", "+", ",", "-", "/"]
            //
            //   "*" -> S48
            //   "/" -> S49
            //   [")", "+", ",", "-"] -> Expr = Factor => ActionFn(3);
            //
            pub fn __state32<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state48(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state49(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Comma, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<>(arena, __sym0);
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

            // State 33
            //     AllInputs = [Term]
            //     OptionalInputs = []
            //     FixedInputs = [Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = Term (*) [")", "*", "+", ",", "-", "/"]
            //
            //   [")", "*", "+", ",", "-", "/"] -> Factor = Term => ActionFn(7);
            //
            pub fn __state33<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Comma, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action7::<>(arena, __sym0);
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

            // State 34
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
            //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Term = (*) "(" Expr ")" [")"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+"]
            //     Term = (*) "(" Expr ")" ["-"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = "(" (*) Expr ")" [")", "*", "+", ",", "-", "/"]
            //     Term = (*) Num [")"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+"]
            //     Term = (*) Num ["-"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S14
            //   "*" -> S15
            //   Num -> S16
            //
            //     Expr -> S50
            //     Factor -> S12
            //     Term -> S13
            pub fn __state34<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                let __sym0 = &mut Some(__sym0);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state14(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state15(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state16(arena, __tokens, __sym1, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state50(arena, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Factor(__sym1) => {
                            __result = try!(__state12(arena, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state13(arena, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 35
            //     AllInputs = ["*"]
            //     OptionalInputs = []
            //     FixedInputs = ["*"]
            //     WillPushLen = 3
            //     WillPush = ["(", Comma<Expr>, ")"]
            //     WillProduce = Some(Factor)
            //
            //     Factor = "*" (*) "(" Comma<Expr> ")" [")", "*", "+", ",", "-", "/"]
            //
            //   "(" -> S51
            //
            pub fn __state35<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state51(arena, __tokens, __sym0, __sym1, ::std::marker::PhantomData::<()>));
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

            // State 36
            //     AllInputs = [Num]
            //     OptionalInputs = []
            //     FixedInputs = [Num]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = Num (*) [")", "*", "+", ",", "-", "/"]
            //
            //   [")", "*", "+", ",", "-", "/"] -> Term = Num => ActionFn(8);
            //
            pub fn __state36<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, i32, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Comma, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action8::<>(arena, __sym0);
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

            // State 37
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
            //   "*" -> S25
            //   "/" -> S26
            //   [")", "+", "-"] -> Expr = Expr, "+", Factor => ActionFn(2);
            //
            pub fn __state37<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: &mut Option<(usize, &'ast Node<'ast>, usize)>,
                __sym1: &mut Option<(usize, Tok, usize)>,
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state25(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state26(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<>(arena, __sym0, __sym1, __sym2);
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

            // State 38
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
            //   "*" -> S25
            //   "/" -> S26
            //   [")", "+", "-"] -> Expr = Expr, "-", Factor => ActionFn(1);
            //
            pub fn __state38<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: &mut Option<(usize, &'ast Node<'ast>, usize)>,
                __sym1: &mut Option<(usize, Tok, usize)>,
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state25(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state26(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1::<>(arena, __sym0, __sym1, __sym2);
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

            // State 39
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
            pub fn __state39<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action4::<>(arena, __sym0, __sym1, __sym2);
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

            // State 40
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
            pub fn __state40<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5::<>(arena, __sym0, __sym1, __sym2);
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

            // State 41
            //     AllInputs = ["(", Expr, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["(", Expr, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = "(" Expr ")" (*) [")", "*", "+", "-", "/"]
            //
            //   [")", "*", "+", "-", "/"] -> Term = "(", Expr, ")" => ActionFn(9);
            //
            pub fn __state41<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, &'ast Node<'ast>, usize),
                __sym2: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        let __nt = super::super::super::__action9::<>(arena, __sym0, __sym1, __sym2);
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

            // State 42
            //     AllInputs = ["*", "(", Comma<Expr>]
            //     OptionalInputs = []
            //     FixedInputs = ["*", "(", Comma<Expr>]
            //     WillPushLen = 1
            //     WillPush = [")"]
            //     WillProduce = Some(Factor)
            //
            //     Factor = "*" "(" Comma<Expr> (*) ")" [")", "*", "+", "-", "/"]
            //
            //   ")" -> S52
            //
            pub fn __state42<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, Vec<&'ast Node<'ast>>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state52(arena, __tokens, __sym0, __sym1, __sym2, __sym3, ::std::marker::PhantomData::<()>));
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

            // State 43
            //     AllInputs = [(<Expr> ",")+, Expr]
            //     OptionalInputs = [(<Expr> ",")+]
            //     FixedInputs = [Expr]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     (<Expr> ",")+ = (<Expr> ",")+ Expr (*) "," ["(", ")", "*", Num]
            //     Comma<Expr> = (<Expr> ",")+ Expr (*) [")"]
            //     Expr = Expr (*) "+" Factor [")", "+", ",", "-"]
            //     Expr = Expr (*) "-" Factor [")", "+", ",", "-"]
            //
            //   "+" -> S45
            //   "," -> S53
            //   "-" -> S47
            //   [")"] -> Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(24);
            //
            pub fn __state43<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: &mut Option<(usize, ::std::vec::Vec<&'ast Node<'ast>>, usize)>,
                __sym1: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state45(arena, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Comma, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state53(arena, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state47(arena, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::RParen, _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action24::<>(arena, __sym0, __sym1);
                        let __nt = __Nonterminal::Comma_3cExpr_3e((
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

            // State 44
            //     AllInputs = ["*", "(", Comma<Expr>, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["*", "(", Comma<Expr>, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = "*" "(" Comma<Expr> ")" (*) ["*", "+", "-", "/", EOF]
            //
            //   ["*", "+", "-", "/", EOF] -> Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);
            //
            pub fn __state44<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, Vec<&'ast Node<'ast>>, usize),
                __sym3: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action6::<>(arena, __sym0, __sym1, __sym2, __sym3);
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

            // State 45
            //     AllInputs = [Expr, "+"]
            //     OptionalInputs = []
            //     FixedInputs = [Expr, "+"]
            //     WillPushLen = 1
            //     WillPush = [Factor]
            //     WillProduce = Some(Expr)
            //
            //     Expr = Expr "+" (*) Factor [")", "+", ",", "-"]
            //     Factor = (*) Factor "*" Term [")", "+", ",", "-"]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term [")", "+", ",", "-"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term [")", "+", ",", "-"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["/"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", ",", "-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Term = (*) "(" Expr ")" [")", "+", ",", "-"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) Num [")", "+", ",", "-"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S34
            //   "*" -> S35
            //   Num -> S36
            //
            //     Factor -> S54
            //     Term -> S33
            pub fn __state45<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        __result = try!(__state34(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state35(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state36(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state54(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state33(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 46
            //     AllInputs = [Expr, ","]
            //     OptionalInputs = []
            //     FixedInputs = [Expr, ","]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some((<Expr> ",")+)
            //
            //     (<Expr> ",")+ = Expr "," (*) ["(", ")", "*", Num]
            //
            //   ["(", ")", "*", Num] -> (<Expr> ",")+ = Expr, "," => ActionFn(18);
            //
            pub fn __state46<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((_, Tok::LParen, _)) |
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Num(_), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action18::<>(arena, __sym0, __sym1);
                        let __nt = __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2b((
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

            // State 47
            //     AllInputs = [Expr, "-"]
            //     OptionalInputs = []
            //     FixedInputs = [Expr, "-"]
            //     WillPushLen = 1
            //     WillPush = [Factor]
            //     WillProduce = Some(Expr)
            //
            //     Expr = Expr "-" (*) Factor [")", "+", ",", "-"]
            //     Factor = (*) Factor "*" Term [")", "+", ",", "-"]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term [")", "+", ",", "-"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term [")", "+", ",", "-"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["/"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", ",", "-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Term = (*) "(" Expr ")" [")", "+", ",", "-"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) Num [")", "+", ",", "-"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S34
            //   "*" -> S35
            //   Num -> S36
            //
            //     Factor -> S55
            //     Term -> S33
            pub fn __state47<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        __result = try!(__state34(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state35(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state36(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state55(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state33(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 48
            //     AllInputs = [Factor, "*"]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "*"]
            //     WillPushLen = 1
            //     WillPush = [Term]
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "*" (*) Term [")", "*", "+", ",", "-", "/"]
            //     Term = (*) "(" Expr ")" [")", "*", "+", ",", "-", "/"]
            //     Term = (*) Num [")", "*", "+", ",", "-", "/"]
            //
            //   "(" -> S34
            //   Num -> S36
            //
            //     Term -> S56
            pub fn __state48<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state34(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state36(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state56(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 49
            //     AllInputs = [Factor, "/"]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "/"]
            //     WillPushLen = 1
            //     WillPush = [Term]
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "/" (*) Term [")", "*", "+", ",", "-", "/"]
            //     Term = (*) "(" Expr ")" [")", "*", "+", ",", "-", "/"]
            //     Term = (*) Num [")", "*", "+", ",", "-", "/"]
            //
            //   "(" -> S34
            //   Num -> S36
            //
            //     Term -> S57
            pub fn __state49<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state34(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state36(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
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
                            __result = try!(__state57(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 50
            //     AllInputs = ["(", Expr]
            //     OptionalInputs = ["("]
            //     FixedInputs = [Expr]
            //     WillPushLen = 1
            //     WillPush = [")"]
            //     WillProduce = None
            //
            //     Expr = Expr (*) "+" Factor [")", "+", "-"]
            //     Expr = Expr (*) "-" Factor [")", "+", "-"]
            //     Term = "(" Expr (*) ")" [")", "*", "+", ",", "-", "/"]
            //
            //   ")" -> S58
            //   "+" -> S23
            //   "-" -> S24
            //
            pub fn __state50<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: &mut Option<(usize, Tok, usize)>,
                __sym1: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        let __sym0 = __sym0.take().unwrap();
                        __result = try!(__state58(arena, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state23(arena, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state24(arena, __tokens, __sym1, __sym2, ::std::marker::PhantomData::<()>));
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

            // State 51
            //     AllInputs = ["*", "("]
            //     OptionalInputs = []
            //     FixedInputs = ["*", "("]
            //     WillPushLen = 2
            //     WillPush = [Comma<Expr>, ")"]
            //     WillProduce = Some(Factor)
            //
            //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," ["(", "*", Num]
            //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," [")"]
            //     (<Expr> ",")+ = (*) Expr "," ["(", "*", Num]
            //     (<Expr> ",")+ = (*) Expr "," [")"]
            //     Comma<Expr> = (*) [")"]
            //     Comma<Expr> = (*) (<Expr> ",")+ [")"]
            //     Comma<Expr> = (*) (<Expr> ",")+ Expr [")"]
            //     Comma<Expr> = (*) Expr [")"]
            //     Expr = (*) Expr "+" Factor [")"]
            //     Expr = (*) Expr "+" Factor ["+"]
            //     Expr = (*) Expr "+" Factor [","]
            //     Expr = (*) Expr "+" Factor ["-"]
            //     Expr = (*) Expr "-" Factor [")"]
            //     Expr = (*) Expr "-" Factor ["+"]
            //     Expr = (*) Expr "-" Factor [","]
            //     Expr = (*) Expr "-" Factor ["-"]
            //     Expr = (*) Factor [")"]
            //     Expr = (*) Factor ["+"]
            //     Expr = (*) Factor [","]
            //     Expr = (*) Factor ["-"]
            //     Factor = (*) Factor "*" Term [")"]
            //     Factor = (*) Factor "*" Term ["*"]
            //     Factor = (*) Factor "*" Term ["+"]
            //     Factor = (*) Factor "*" Term [","]
            //     Factor = (*) Factor "*" Term ["-"]
            //     Factor = (*) Factor "*" Term ["/"]
            //     Factor = (*) Factor "/" Term [")"]
            //     Factor = (*) Factor "/" Term ["*"]
            //     Factor = (*) Factor "/" Term ["+"]
            //     Factor = (*) Factor "/" Term [","]
            //     Factor = (*) Factor "/" Term ["-"]
            //     Factor = (*) Factor "/" Term ["/"]
            //     Factor = (*) Term [")"]
            //     Factor = (*) Term ["*"]
            //     Factor = (*) Term ["+"]
            //     Factor = (*) Term [","]
            //     Factor = (*) Term ["-"]
            //     Factor = (*) Term ["/"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
            //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
            //     Factor = "*" "(" (*) Comma<Expr> ")" [")", "*", "+", ",", "-", "/"]
            //     Term = (*) "(" Expr ")" [")"]
            //     Term = (*) "(" Expr ")" ["*"]
            //     Term = (*) "(" Expr ")" ["+"]
            //     Term = (*) "(" Expr ")" [","]
            //     Term = (*) "(" Expr ")" ["-"]
            //     Term = (*) "(" Expr ")" ["/"]
            //     Term = (*) Num [")"]
            //     Term = (*) Num ["*"]
            //     Term = (*) Num ["+"]
            //     Term = (*) Num [","]
            //     Term = (*) Num ["-"]
            //     Term = (*) Num ["/"]
            //
            //   "(" -> S34
            //   "*" -> S35
            //   Num -> S36
            //   [")"] -> Comma<Expr> =  => ActionFn(23);
            //
            //     (<Expr> ",")+ -> S29
            //     Comma<Expr> -> S59
            //     Expr -> S31
            //     Factor -> S32
            //     Term -> S33
            pub fn __state51<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state34(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym2 = (__loc1, (__tok), __loc2);
                        __result = try!(__state35(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state36(arena, __tokens, __sym2, ::std::marker::PhantomData::<()>));
                    }
                    Some((_, Tok::RParen, _)) => {
                        let __start = __sym1.2.clone();
                        let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action23::<>(arena, &__start, &__end);
                        let __nt = __Nonterminal::Comma_3cExpr_3e((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
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
                        __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2b(__sym2) => {
                            __result = try!(__state29(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Comma_3cExpr_3e(__sym2) => {
                            __result = try!(__state59(arena, __tokens, __lookahead, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        __Nonterminal::Expr(__sym2) => {
                            __result = try!(__state31(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Factor(__sym2) => {
                            __result = try!(__state32(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym2) => {
                            __result = try!(__state33(arena, __tokens, __lookahead, __sym2, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 52
            //     AllInputs = ["*", "(", Comma<Expr>, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["*", "(", Comma<Expr>, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = "*" "(" Comma<Expr> ")" (*) [")", "*", "+", "-", "/"]
            //
            //   [")", "*", "+", "-", "/"] -> Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);
            //
            pub fn __state52<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, Vec<&'ast Node<'ast>>, usize),
                __sym3: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action6::<>(arena, __sym0, __sym1, __sym2, __sym3);
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

            // State 53
            //     AllInputs = [(<Expr> ",")+, Expr, ","]
            //     OptionalInputs = []
            //     FixedInputs = [(<Expr> ",")+, Expr, ","]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some((<Expr> ",")+)
            //
            //     (<Expr> ",")+ = (<Expr> ",")+ Expr "," (*) ["(", ")", "*", Num]
            //
            //   ["(", ")", "*", Num] -> (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(19);
            //
            pub fn __state53<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
                __sym1: (usize, &'ast Node<'ast>, usize),
                __sym2: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((_, Tok::LParen, _)) |
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Num(_), _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action19::<>(arena, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2b((
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

            // State 54
            //     AllInputs = [Expr, "+", Factor]
            //     OptionalInputs = [Expr, "+"]
            //     FixedInputs = [Factor]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Expr = Expr "+" Factor (*) [")", "+", ",", "-"]
            //     Factor = Factor (*) "*" Term [")", "*", "+", ",", "-", "/"]
            //     Factor = Factor (*) "/" Term [")", "*", "+", ",", "-", "/"]
            //
            //   "*" -> S48
            //   "/" -> S49
            //   [")", "+", ",", "-"] -> Expr = Expr, "+", Factor => ActionFn(2);
            //
            pub fn __state54<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: &mut Option<(usize, &'ast Node<'ast>, usize)>,
                __sym1: &mut Option<(usize, Tok, usize)>,
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state48(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state49(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Comma, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<>(arena, __sym0, __sym1, __sym2);
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

            // State 55
            //     AllInputs = [Expr, "-", Factor]
            //     OptionalInputs = [Expr, "-"]
            //     FixedInputs = [Factor]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Expr = Expr "-" Factor (*) [")", "+", ",", "-"]
            //     Factor = Factor (*) "*" Term [")", "*", "+", ",", "-", "/"]
            //     Factor = Factor (*) "/" Term [")", "*", "+", ",", "-", "/"]
            //
            //   "*" -> S48
            //   "/" -> S49
            //   [")", "+", ",", "-"] -> Expr = Expr, "-", Factor => ActionFn(1);
            //
            pub fn __state55<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: &mut Option<(usize, &'ast Node<'ast>, usize)>,
                __sym1: &mut Option<(usize, Tok, usize)>,
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state48(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state49(arena, __tokens, __sym2, __sym3, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Comma, _)) |
                    Some((_, Tok::Minus, _)) => {
                        let __sym0 = __sym0.take().unwrap();
                        let __sym1 = __sym1.take().unwrap();
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1::<>(arena, __sym0, __sym1, __sym2);
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

            // State 56
            //     AllInputs = [Factor, "*", Term]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "*", Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "*" Term (*) [")", "*", "+", ",", "-", "/"]
            //
            //   [")", "*", "+", ",", "-", "/"] -> Factor = Factor, "*", Term => ActionFn(4);
            //
            pub fn __state56<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Comma, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action4::<>(arena, __sym0, __sym1, __sym2);
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

            // State 57
            //     AllInputs = [Factor, "/", Term]
            //     OptionalInputs = []
            //     FixedInputs = [Factor, "/", Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = Factor "/" Term (*) [")", "*", "+", ",", "-", "/"]
            //
            //   [")", "*", "+", ",", "-", "/"] -> Factor = Factor, "/", Term => ActionFn(5);
            //
            pub fn __state57<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, &'ast Node<'ast>, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, &'ast Node<'ast>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Comma, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5::<>(arena, __sym0, __sym1, __sym2);
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

            // State 58
            //     AllInputs = ["(", Expr, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["(", Expr, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = "(" Expr ")" (*) [")", "*", "+", ",", "-", "/"]
            //
            //   [")", "*", "+", ",", "-", "/"] -> Term = "(", Expr, ")" => ActionFn(9);
            //
            pub fn __state58<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, &'ast Node<'ast>, usize),
                __sym2: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Comma, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action9::<>(arena, __sym0, __sym1, __sym2);
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

            // State 59
            //     AllInputs = ["*", "(", Comma<Expr>]
            //     OptionalInputs = []
            //     FixedInputs = ["*", "(", Comma<Expr>]
            //     WillPushLen = 1
            //     WillPush = [")"]
            //     WillProduce = Some(Factor)
            //
            //     Factor = "*" "(" Comma<Expr> (*) ")" [")", "*", "+", ",", "-", "/"]
            //
            //   ")" -> S60
            //
            pub fn __state59<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, Vec<&'ast Node<'ast>>, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                        let __sym3 = (__loc1, (__tok), __loc2);
                        __result = try!(__state60(arena, __tokens, __sym0, __sym1, __sym2, __sym3, ::std::marker::PhantomData::<()>));
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

            // State 60
            //     AllInputs = ["*", "(", Comma<Expr>, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["*", "(", Comma<Expr>, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Factor)
            //
            //     Factor = "*" "(" Comma<Expr> ")" (*) [")", "*", "+", ",", "-", "/"]
            //
            //   [")", "*", "+", ",", "-", "/"] -> Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);
            //
            pub fn __state60<
                'ast,
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens: &mut __TOKENS,
                __sym0: (usize, Tok, usize),
                __sym1: (usize, Tok, usize),
                __sym2: (usize, Vec<&'ast Node<'ast>>, usize),
                __sym3: (usize, Tok, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __lalrpop_util::ParseError<usize,Tok,()>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((_, Tok::RParen, _)) |
                    Some((_, Tok::Times, _)) |
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Comma, _)) |
                    Some((_, Tok::Minus, _)) |
                    Some((_, Tok::Div, _)) => {
                        let __start = __sym0.0.clone();
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action6::<>(arena, __sym0, __sym1, __sym2, __sym3);
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
        }
        pub use self::__parse__Expr::parse_Expr;
    }
    mod __parse_table {

        mod __parse__Expr {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use expr_arena_ast::{Arena, Node, Op};
            use util::tok::Tok;
            extern crate lalrpop_util as __lalrpop_util;
            use super::super::super::__ToTriple;
            #[allow(dead_code)]
            pub enum __Symbol<'ast> {
                Term_22_28_22(Tok),
                Term_22_29_22(Tok),
                Term_22_2a_22(Tok),
                Term_22_2b_22(Tok),
                Term_22_2c_22(Tok),
                Term_22_2d_22(Tok),
                Term_22_2f_22(Tok),
                TermNum(i32),
                Nt_28_3cExpr_3e_20_22_2c_22_29(&'ast Node<'ast>),
                Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'ast Node<'ast>>),
                Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'ast Node<'ast>>),
                NtComma_3cExpr_3e(Vec<&'ast Node<'ast>>),
                NtExpr(&'ast Node<'ast>),
                NtExpr_3f(::std::option::Option<&'ast Node<'ast>>),
                NtFactor(&'ast Node<'ast>),
                NtTerm(&'ast Node<'ast>),
                Nt____Expr(&'ast Node<'ast>),
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
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" [EOF]
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
                6, // on "*", goto 5
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                7, // on Num, goto 6
                // State 1
                //     Expr = Expr (*) "+" Factor ["+", "-", EOF]
                //     Expr = Expr (*) "-" Factor ["+", "-", EOF]
                //     __Expr = Expr (*) [EOF]
                0, // on "(", error
                0, // on ")", error
                0, // on "*", error
                8, // on "+", goto 7
                0, // on ",", error
                9, // on "-", goto 8
                0, // on "/", error
                0, // on Num, error
                // State 2
                //     Expr = Factor (*) ["+", "-", EOF]
                //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
                //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                10, // on "*", goto 9
                -12, // on "+", reduce `Expr = Factor => ActionFn(3);`
                0, // on ",", error
                -12, // on "-", reduce `Expr = Factor => ActionFn(3);`
                11, // on "/", goto 10
                0, // on Num, error
                // State 3
                //     Factor = Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -18, // on "*", reduce `Factor = Term => ActionFn(7);`
                -18, // on "+", reduce `Factor = Term => ActionFn(7);`
                0, // on ",", error
                -18, // on "-", reduce `Factor = Term => ActionFn(7);`
                -18, // on "/", reduce `Factor = Term => ActionFn(7);`
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
                //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
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
                15, // on "(", goto 14
                0, // on ")", error
                16, // on "*", goto 15
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                17, // on Num, goto 16
                // State 5
                //     Factor = "*" (*) "(" Comma<Expr> ")" ["*", "+", "-", "/", EOF]
                18, // on "(", goto 17
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                0, // on Num, error
                // State 6
                //     Term = Num (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -19, // on "*", reduce `Term = Num => ActionFn(8);`
                -19, // on "+", reduce `Term = Num => ActionFn(8);`
                0, // on ",", error
                -19, // on "-", reduce `Term = Num => ActionFn(8);`
                -19, // on "/", reduce `Term = Num => ActionFn(8);`
                0, // on Num, error
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
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["+", "-", EOF]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+", "-", EOF]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+", "-", EOF]
                //     Term = (*) Num ["/"]
                5, // on "(", goto 4
                0, // on ")", error
                6, // on "*", goto 5
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                7, // on Num, goto 6
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
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["+", "-", EOF]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+", "-", EOF]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+", "-", EOF]
                //     Term = (*) Num ["/"]
                5, // on "(", goto 4
                0, // on ")", error
                6, // on "*", goto 5
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                7, // on Num, goto 6
                // State 9
                //     Factor = Factor "*" (*) Term ["*", "+", "-", "/", EOF]
                //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
                //     Term = (*) Num ["*", "+", "-", "/", EOF]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                7, // on Num, goto 6
                // State 10
                //     Factor = Factor "/" (*) Term ["*", "+", "-", "/", EOF]
                //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
                //     Term = (*) Num ["*", "+", "-", "/", EOF]
                5, // on "(", goto 4
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                7, // on Num, goto 6
                // State 11
                //     Expr = Expr (*) "+" Factor [")", "+", "-"]
                //     Expr = Expr (*) "-" Factor [")", "+", "-"]
                //     Term = "(" Expr (*) ")" ["*", "+", "-", "/", EOF]
                0, // on "(", error
                23, // on ")", goto 22
                0, // on "*", error
                24, // on "+", goto 23
                0, // on ",", error
                25, // on "-", goto 24
                0, // on "/", error
                0, // on Num, error
                // State 12
                //     Expr = Factor (*) [")", "+", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
                0, // on "(", error
                -12, // on ")", reduce `Expr = Factor => ActionFn(3);`
                26, // on "*", goto 25
                -12, // on "+", reduce `Expr = Factor => ActionFn(3);`
                0, // on ",", error
                -12, // on "-", reduce `Expr = Factor => ActionFn(3);`
                27, // on "/", goto 26
                0, // on Num, error
                // State 13
                //     Factor = Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -18, // on ")", reduce `Factor = Term => ActionFn(7);`
                -18, // on "*", reduce `Factor = Term => ActionFn(7);`
                -18, // on "+", reduce `Factor = Term => ActionFn(7);`
                0, // on ",", error
                -18, // on "-", reduce `Factor = Term => ActionFn(7);`
                -18, // on "/", reduce `Factor = Term => ActionFn(7);`
                0, // on Num, error
                // State 14
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
                //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
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
                15, // on "(", goto 14
                0, // on ")", error
                16, // on "*", goto 15
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                17, // on Num, goto 16
                // State 15
                //     Factor = "*" (*) "(" Comma<Expr> ")" [")", "*", "+", "-", "/"]
                29, // on "(", goto 28
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                0, // on Num, error
                // State 16
                //     Term = Num (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -19, // on ")", reduce `Term = Num => ActionFn(8);`
                -19, // on "*", reduce `Term = Num => ActionFn(8);`
                -19, // on "+", reduce `Term = Num => ActionFn(8);`
                0, // on ",", error
                -19, // on "-", reduce `Term = Num => ActionFn(8);`
                -19, // on "/", reduce `Term = Num => ActionFn(8);`
                0, // on Num, error
                // State 17
                //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," ["(", "*", Num]
                //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," [")"]
                //     (<Expr> ",")+ = (*) Expr "," ["(", "*", Num]
                //     (<Expr> ",")+ = (*) Expr "," [")"]
                //     Comma<Expr> = (*) [")"]
                //     Comma<Expr> = (*) (<Expr> ",")+ [")"]
                //     Comma<Expr> = (*) (<Expr> ",")+ Expr [")"]
                //     Comma<Expr> = (*) Expr [")"]
                //     Expr = (*) Expr "+" Factor [")"]
                //     Expr = (*) Expr "+" Factor ["+"]
                //     Expr = (*) Expr "+" Factor [","]
                //     Expr = (*) Expr "+" Factor ["-"]
                //     Expr = (*) Expr "-" Factor [")"]
                //     Expr = (*) Expr "-" Factor ["+"]
                //     Expr = (*) Expr "-" Factor [","]
                //     Expr = (*) Expr "-" Factor ["-"]
                //     Expr = (*) Factor [")"]
                //     Expr = (*) Factor ["+"]
                //     Expr = (*) Factor [","]
                //     Expr = (*) Factor ["-"]
                //     Factor = (*) Factor "*" Term [")"]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["+"]
                //     Factor = (*) Factor "*" Term [","]
                //     Factor = (*) Factor "*" Term ["-"]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term [")"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["+"]
                //     Factor = (*) Factor "/" Term [","]
                //     Factor = (*) Factor "/" Term ["-"]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term [")"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["+"]
                //     Factor = (*) Term [","]
                //     Factor = (*) Term ["-"]
                //     Factor = (*) Term ["/"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Factor = "*" "(" (*) Comma<Expr> ")" ["*", "+", "-", "/", EOF]
                //     Term = (*) "(" Expr ")" [")"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" [","]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) Num [")"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+"]
                //     Term = (*) Num [","]
                //     Term = (*) Num ["-"]
                //     Term = (*) Num ["/"]
                35, // on "(", goto 34
                -7, // on ")", reduce `Comma<Expr> =  => ActionFn(23);`
                36, // on "*", goto 35
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                37, // on Num, goto 36
                // State 18
                //     Expr = Expr "+" Factor (*) ["+", "-", EOF]
                //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
                //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                10, // on "*", goto 9
                -11, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                0, // on ",", error
                -11, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                11, // on "/", goto 10
                0, // on Num, error
                // State 19
                //     Expr = Expr "-" Factor (*) ["+", "-", EOF]
                //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
                //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                10, // on "*", goto 9
                -10, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                0, // on ",", error
                -10, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                11, // on "/", goto 10
                0, // on Num, error
                // State 20
                //     Factor = Factor "*" Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -15, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -15, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                0, // on ",", error
                -15, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -15, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                0, // on Num, error
                // State 21
                //     Factor = Factor "/" Term (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -16, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -16, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                0, // on ",", error
                -16, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -16, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                0, // on Num, error
                // State 22
                //     Term = "(" Expr ")" (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -20, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                -20, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                0, // on ",", error
                -20, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                -20, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                0, // on Num, error
                // State 23
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
                //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", "-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Term = (*) "(" Expr ")" [")", "+", "-"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) Num [")", "+", "-"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["/"]
                15, // on "(", goto 14
                0, // on ")", error
                16, // on "*", goto 15
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                17, // on Num, goto 16
                // State 24
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
                //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", "-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Term = (*) "(" Expr ")" [")", "+", "-"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) Num [")", "+", "-"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["/"]
                15, // on "(", goto 14
                0, // on ")", error
                16, // on "*", goto 15
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                17, // on Num, goto 16
                // State 25
                //     Factor = Factor "*" (*) Term [")", "*", "+", "-", "/"]
                //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
                //     Term = (*) Num [")", "*", "+", "-", "/"]
                15, // on "(", goto 14
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                17, // on Num, goto 16
                // State 26
                //     Factor = Factor "/" (*) Term [")", "*", "+", "-", "/"]
                //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
                //     Term = (*) Num [")", "*", "+", "-", "/"]
                15, // on "(", goto 14
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                17, // on Num, goto 16
                // State 27
                //     Expr = Expr (*) "+" Factor [")", "+", "-"]
                //     Expr = Expr (*) "-" Factor [")", "+", "-"]
                //     Term = "(" Expr (*) ")" [")", "*", "+", "-", "/"]
                0, // on "(", error
                42, // on ")", goto 41
                0, // on "*", error
                24, // on "+", goto 23
                0, // on ",", error
                25, // on "-", goto 24
                0, // on "/", error
                0, // on Num, error
                // State 28
                //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," ["(", "*", Num]
                //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," [")"]
                //     (<Expr> ",")+ = (*) Expr "," ["(", "*", Num]
                //     (<Expr> ",")+ = (*) Expr "," [")"]
                //     Comma<Expr> = (*) [")"]
                //     Comma<Expr> = (*) (<Expr> ",")+ [")"]
                //     Comma<Expr> = (*) (<Expr> ",")+ Expr [")"]
                //     Comma<Expr> = (*) Expr [")"]
                //     Expr = (*) Expr "+" Factor [")"]
                //     Expr = (*) Expr "+" Factor ["+"]
                //     Expr = (*) Expr "+" Factor [","]
                //     Expr = (*) Expr "+" Factor ["-"]
                //     Expr = (*) Expr "-" Factor [")"]
                //     Expr = (*) Expr "-" Factor ["+"]
                //     Expr = (*) Expr "-" Factor [","]
                //     Expr = (*) Expr "-" Factor ["-"]
                //     Expr = (*) Factor [")"]
                //     Expr = (*) Factor ["+"]
                //     Expr = (*) Factor [","]
                //     Expr = (*) Factor ["-"]
                //     Factor = (*) Factor "*" Term [")"]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["+"]
                //     Factor = (*) Factor "*" Term [","]
                //     Factor = (*) Factor "*" Term ["-"]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term [")"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["+"]
                //     Factor = (*) Factor "/" Term [","]
                //     Factor = (*) Factor "/" Term ["-"]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term [")"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["+"]
                //     Factor = (*) Term [","]
                //     Factor = (*) Term ["-"]
                //     Factor = (*) Term ["/"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Factor = "*" "(" (*) Comma<Expr> ")" [")", "*", "+", "-", "/"]
                //     Term = (*) "(" Expr ")" [")"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" [","]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) Num [")"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+"]
                //     Term = (*) Num [","]
                //     Term = (*) Num ["-"]
                //     Term = (*) Num ["/"]
                35, // on "(", goto 34
                -7, // on ")", reduce `Comma<Expr> =  => ActionFn(23);`
                36, // on "*", goto 35
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                37, // on Num, goto 36
                // State 29
                //     (<Expr> ",")+ = (<Expr> ",")+ (*) Expr "," ["(", ")", "*", Num]
                //     Comma<Expr> = (<Expr> ",")+ (*) [")"]
                //     Comma<Expr> = (<Expr> ",")+ (*) Expr [")"]
                //     Expr = (*) Expr "+" Factor [")"]
                //     Expr = (*) Expr "+" Factor ["+"]
                //     Expr = (*) Expr "+" Factor [","]
                //     Expr = (*) Expr "+" Factor ["-"]
                //     Expr = (*) Expr "-" Factor [")"]
                //     Expr = (*) Expr "-" Factor ["+"]
                //     Expr = (*) Expr "-" Factor [","]
                //     Expr = (*) Expr "-" Factor ["-"]
                //     Expr = (*) Factor [")"]
                //     Expr = (*) Factor ["+"]
                //     Expr = (*) Factor [","]
                //     Expr = (*) Factor ["-"]
                //     Factor = (*) Factor "*" Term [")"]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["+"]
                //     Factor = (*) Factor "*" Term [","]
                //     Factor = (*) Factor "*" Term ["-"]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term [")"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["+"]
                //     Factor = (*) Factor "/" Term [","]
                //     Factor = (*) Factor "/" Term ["-"]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term [")"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["+"]
                //     Factor = (*) Term [","]
                //     Factor = (*) Term ["-"]
                //     Factor = (*) Term ["/"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Term = (*) "(" Expr ")" [")"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" [","]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) Num [")"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+"]
                //     Term = (*) Num [","]
                //     Term = (*) Num ["-"]
                //     Term = (*) Num ["/"]
                35, // on "(", goto 34
                -9, // on ")", reduce `Comma<Expr> = (<Expr> ",")+ => ActionFn(25);`
                36, // on "*", goto 35
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                37, // on Num, goto 36
                // State 30
                //     Factor = "*" "(" Comma<Expr> (*) ")" ["*", "+", "-", "/", EOF]
                0, // on "(", error
                45, // on ")", goto 44
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                0, // on Num, error
                // State 31
                //     (<Expr> ",")+ = Expr (*) "," ["(", ")", "*", Num]
                //     Comma<Expr> = Expr (*) [")"]
                //     Expr = Expr (*) "+" Factor [")", "+", ",", "-"]
                //     Expr = Expr (*) "-" Factor [")", "+", ",", "-"]
                0, // on "(", error
                -6, // on ")", reduce `Comma<Expr> = Expr => ActionFn(22);`
                0, // on "*", error
                46, // on "+", goto 45
                47, // on ",", goto 46
                48, // on "-", goto 47
                0, // on "/", error
                0, // on Num, error
                // State 32
                //     Expr = Factor (*) [")", "+", ",", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", ",", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", ",", "-", "/"]
                0, // on "(", error
                -12, // on ")", reduce `Expr = Factor => ActionFn(3);`
                49, // on "*", goto 48
                -12, // on "+", reduce `Expr = Factor => ActionFn(3);`
                -12, // on ",", reduce `Expr = Factor => ActionFn(3);`
                -12, // on "-", reduce `Expr = Factor => ActionFn(3);`
                50, // on "/", goto 49
                0, // on Num, error
                // State 33
                //     Factor = Term (*) [")", "*", "+", ",", "-", "/"]
                0, // on "(", error
                -18, // on ")", reduce `Factor = Term => ActionFn(7);`
                -18, // on "*", reduce `Factor = Term => ActionFn(7);`
                -18, // on "+", reduce `Factor = Term => ActionFn(7);`
                -18, // on ",", reduce `Factor = Term => ActionFn(7);`
                -18, // on "-", reduce `Factor = Term => ActionFn(7);`
                -18, // on "/", reduce `Factor = Term => ActionFn(7);`
                0, // on Num, error
                // State 34
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
                //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Term = (*) "(" Expr ")" [")"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = "(" (*) Expr ")" [")", "*", "+", ",", "-", "/"]
                //     Term = (*) Num [")"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+"]
                //     Term = (*) Num ["-"]
                //     Term = (*) Num ["/"]
                15, // on "(", goto 14
                0, // on ")", error
                16, // on "*", goto 15
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                17, // on Num, goto 16
                // State 35
                //     Factor = "*" (*) "(" Comma<Expr> ")" [")", "*", "+", ",", "-", "/"]
                52, // on "(", goto 51
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                0, // on Num, error
                // State 36
                //     Term = Num (*) [")", "*", "+", ",", "-", "/"]
                0, // on "(", error
                -19, // on ")", reduce `Term = Num => ActionFn(8);`
                -19, // on "*", reduce `Term = Num => ActionFn(8);`
                -19, // on "+", reduce `Term = Num => ActionFn(8);`
                -19, // on ",", reduce `Term = Num => ActionFn(8);`
                -19, // on "-", reduce `Term = Num => ActionFn(8);`
                -19, // on "/", reduce `Term = Num => ActionFn(8);`
                0, // on Num, error
                // State 37
                //     Expr = Expr "+" Factor (*) [")", "+", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
                0, // on "(", error
                -11, // on ")", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                26, // on "*", goto 25
                -11, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                0, // on ",", error
                -11, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                27, // on "/", goto 26
                0, // on Num, error
                // State 38
                //     Expr = Expr "-" Factor (*) [")", "+", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
                0, // on "(", error
                -10, // on ")", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                26, // on "*", goto 25
                -10, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                0, // on ",", error
                -10, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                27, // on "/", goto 26
                0, // on Num, error
                // State 39
                //     Factor = Factor "*" Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -15, // on ")", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -15, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -15, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                0, // on ",", error
                -15, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -15, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                0, // on Num, error
                // State 40
                //     Factor = Factor "/" Term (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -16, // on ")", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -16, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -16, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                0, // on ",", error
                -16, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -16, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                0, // on Num, error
                // State 41
                //     Term = "(" Expr ")" (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -20, // on ")", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                -20, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                -20, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                0, // on ",", error
                -20, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                -20, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                0, // on Num, error
                // State 42
                //     Factor = "*" "(" Comma<Expr> (*) ")" [")", "*", "+", "-", "/"]
                0, // on "(", error
                53, // on ")", goto 52
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                0, // on Num, error
                // State 43
                //     (<Expr> ",")+ = (<Expr> ",")+ Expr (*) "," ["(", ")", "*", Num]
                //     Comma<Expr> = (<Expr> ",")+ Expr (*) [")"]
                //     Expr = Expr (*) "+" Factor [")", "+", ",", "-"]
                //     Expr = Expr (*) "-" Factor [")", "+", ",", "-"]
                0, // on "(", error
                -8, // on ")", reduce `Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(24);`
                0, // on "*", error
                46, // on "+", goto 45
                54, // on ",", goto 53
                48, // on "-", goto 47
                0, // on "/", error
                0, // on Num, error
                // State 44
                //     Factor = "*" "(" Comma<Expr> ")" (*) ["*", "+", "-", "/", EOF]
                0, // on "(", error
                0, // on ")", error
                -17, // on "*", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                -17, // on "+", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                0, // on ",", error
                -17, // on "-", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                -17, // on "/", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                0, // on Num, error
                // State 45
                //     Expr = Expr "+" (*) Factor [")", "+", ",", "-"]
                //     Factor = (*) Factor "*" Term [")", "+", ",", "-"]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term [")", "+", ",", "-"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term [")", "+", ",", "-"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["/"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", ",", "-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Term = (*) "(" Expr ")" [")", "+", ",", "-"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) Num [")", "+", ",", "-"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["/"]
                35, // on "(", goto 34
                0, // on ")", error
                36, // on "*", goto 35
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                37, // on Num, goto 36
                // State 46
                //     (<Expr> ",")+ = Expr "," (*) ["(", ")", "*", Num]
                -4, // on "(", reduce `(<Expr> ",")+ = Expr, "," => ActionFn(18);`
                -4, // on ")", reduce `(<Expr> ",")+ = Expr, "," => ActionFn(18);`
                -4, // on "*", reduce `(<Expr> ",")+ = Expr, "," => ActionFn(18);`
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                -4, // on Num, reduce `(<Expr> ",")+ = Expr, "," => ActionFn(18);`
                // State 47
                //     Expr = Expr "-" (*) Factor [")", "+", ",", "-"]
                //     Factor = (*) Factor "*" Term [")", "+", ",", "-"]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term [")", "+", ",", "-"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term [")", "+", ",", "-"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["/"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", ",", "-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Term = (*) "(" Expr ")" [")", "+", ",", "-"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) Num [")", "+", ",", "-"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["/"]
                35, // on "(", goto 34
                0, // on ")", error
                36, // on "*", goto 35
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                37, // on Num, goto 36
                // State 48
                //     Factor = Factor "*" (*) Term [")", "*", "+", ",", "-", "/"]
                //     Term = (*) "(" Expr ")" [")", "*", "+", ",", "-", "/"]
                //     Term = (*) Num [")", "*", "+", ",", "-", "/"]
                35, // on "(", goto 34
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                37, // on Num, goto 36
                // State 49
                //     Factor = Factor "/" (*) Term [")", "*", "+", ",", "-", "/"]
                //     Term = (*) "(" Expr ")" [")", "*", "+", ",", "-", "/"]
                //     Term = (*) Num [")", "*", "+", ",", "-", "/"]
                35, // on "(", goto 34
                0, // on ")", error
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                37, // on Num, goto 36
                // State 50
                //     Expr = Expr (*) "+" Factor [")", "+", "-"]
                //     Expr = Expr (*) "-" Factor [")", "+", "-"]
                //     Term = "(" Expr (*) ")" [")", "*", "+", ",", "-", "/"]
                0, // on "(", error
                59, // on ")", goto 58
                0, // on "*", error
                24, // on "+", goto 23
                0, // on ",", error
                25, // on "-", goto 24
                0, // on "/", error
                0, // on Num, error
                // State 51
                //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," ["(", "*", Num]
                //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," [")"]
                //     (<Expr> ",")+ = (*) Expr "," ["(", "*", Num]
                //     (<Expr> ",")+ = (*) Expr "," [")"]
                //     Comma<Expr> = (*) [")"]
                //     Comma<Expr> = (*) (<Expr> ",")+ [")"]
                //     Comma<Expr> = (*) (<Expr> ",")+ Expr [")"]
                //     Comma<Expr> = (*) Expr [")"]
                //     Expr = (*) Expr "+" Factor [")"]
                //     Expr = (*) Expr "+" Factor ["+"]
                //     Expr = (*) Expr "+" Factor [","]
                //     Expr = (*) Expr "+" Factor ["-"]
                //     Expr = (*) Expr "-" Factor [")"]
                //     Expr = (*) Expr "-" Factor ["+"]
                //     Expr = (*) Expr "-" Factor [","]
                //     Expr = (*) Expr "-" Factor ["-"]
                //     Expr = (*) Factor [")"]
                //     Expr = (*) Factor ["+"]
                //     Expr = (*) Factor [","]
                //     Expr = (*) Factor ["-"]
                //     Factor = (*) Factor "*" Term [")"]
                //     Factor = (*) Factor "*" Term ["*"]
                //     Factor = (*) Factor "*" Term ["+"]
                //     Factor = (*) Factor "*" Term [","]
                //     Factor = (*) Factor "*" Term ["-"]
                //     Factor = (*) Factor "*" Term ["/"]
                //     Factor = (*) Factor "/" Term [")"]
                //     Factor = (*) Factor "/" Term ["*"]
                //     Factor = (*) Factor "/" Term ["+"]
                //     Factor = (*) Factor "/" Term [","]
                //     Factor = (*) Factor "/" Term ["-"]
                //     Factor = (*) Factor "/" Term ["/"]
                //     Factor = (*) Term [")"]
                //     Factor = (*) Term ["*"]
                //     Factor = (*) Term ["+"]
                //     Factor = (*) Term [","]
                //     Factor = (*) Term ["-"]
                //     Factor = (*) Term ["/"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
                //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
                //     Factor = "*" "(" (*) Comma<Expr> ")" [")", "*", "+", ",", "-", "/"]
                //     Term = (*) "(" Expr ")" [")"]
                //     Term = (*) "(" Expr ")" ["*"]
                //     Term = (*) "(" Expr ")" ["+"]
                //     Term = (*) "(" Expr ")" [","]
                //     Term = (*) "(" Expr ")" ["-"]
                //     Term = (*) "(" Expr ")" ["/"]
                //     Term = (*) Num [")"]
                //     Term = (*) Num ["*"]
                //     Term = (*) Num ["+"]
                //     Term = (*) Num [","]
                //     Term = (*) Num ["-"]
                //     Term = (*) Num ["/"]
                35, // on "(", goto 34
                -7, // on ")", reduce `Comma<Expr> =  => ActionFn(23);`
                36, // on "*", goto 35
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                37, // on Num, goto 36
                // State 52
                //     Factor = "*" "(" Comma<Expr> ")" (*) [")", "*", "+", "-", "/"]
                0, // on "(", error
                -17, // on ")", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                -17, // on "*", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                -17, // on "+", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                0, // on ",", error
                -17, // on "-", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                -17, // on "/", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                0, // on Num, error
                // State 53
                //     (<Expr> ",")+ = (<Expr> ",")+ Expr "," (*) ["(", ")", "*", Num]
                -5, // on "(", reduce `(<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(19);`
                -5, // on ")", reduce `(<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(19);`
                -5, // on "*", reduce `(<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(19);`
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                -5, // on Num, reduce `(<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(19);`
                // State 54
                //     Expr = Expr "+" Factor (*) [")", "+", ",", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", ",", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", ",", "-", "/"]
                0, // on "(", error
                -11, // on ")", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                49, // on "*", goto 48
                -11, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                -11, // on ",", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                -11, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                50, // on "/", goto 49
                0, // on Num, error
                // State 55
                //     Expr = Expr "-" Factor (*) [")", "+", ",", "-"]
                //     Factor = Factor (*) "*" Term [")", "*", "+", ",", "-", "/"]
                //     Factor = Factor (*) "/" Term [")", "*", "+", ",", "-", "/"]
                0, // on "(", error
                -10, // on ")", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                49, // on "*", goto 48
                -10, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                -10, // on ",", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                -10, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                50, // on "/", goto 49
                0, // on Num, error
                // State 56
                //     Factor = Factor "*" Term (*) [")", "*", "+", ",", "-", "/"]
                0, // on "(", error
                -15, // on ")", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -15, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -15, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -15, // on ",", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -15, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -15, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
                0, // on Num, error
                // State 57
                //     Factor = Factor "/" Term (*) [")", "*", "+", ",", "-", "/"]
                0, // on "(", error
                -16, // on ")", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -16, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -16, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -16, // on ",", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -16, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -16, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
                0, // on Num, error
                // State 58
                //     Term = "(" Expr ")" (*) [")", "*", "+", ",", "-", "/"]
                0, // on "(", error
                -20, // on ")", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                -20, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                -20, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                -20, // on ",", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                -20, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                -20, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(9);`
                0, // on Num, error
                // State 59
                //     Factor = "*" "(" Comma<Expr> (*) ")" [")", "*", "+", ",", "-", "/"]
                0, // on "(", error
                61, // on ")", goto 60
                0, // on "*", error
                0, // on "+", error
                0, // on ",", error
                0, // on "-", error
                0, // on "/", error
                0, // on Num, error
                // State 60
                //     Factor = "*" "(" Comma<Expr> ")" (*) [")", "*", "+", ",", "-", "/"]
                0, // on "(", error
                -17, // on ")", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                -17, // on "*", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                -17, // on "+", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                -17, // on ",", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                -17, // on "-", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                -17, // on "/", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
                0, // on Num, error
            ];
            const __EOF_ACTION: &'static [i32] = &[
                0, // on EOF, error
                -21, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
                -12, // on EOF, reduce `Expr = Factor => ActionFn(3);`
                -18, // on EOF, reduce `Factor = Term => ActionFn(7);`
                0, // on EOF, error
                0, // on EOF, error
                -19, // on EOF, reduce `Term = Num => ActionFn(8);`
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
                -11, // on EOF, reduce `Expr = Expr, "+", Factor => ActionFn(2);`
                -10, // on EOF, reduce `Expr = Expr, "-", Factor => ActionFn(1);`
                -15, // on EOF, reduce `Factor = Factor, "*", Term => ActionFn(4);`
                -16, // on EOF, reduce `Factor = Factor, "/", Term => ActionFn(5);`
                -20, // on EOF, reduce `Term = "(", Expr, ")" => ActionFn(9);`
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
                -17, // on EOF, reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
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
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
                0, // on EOF, error
            ];
            const __GOTO: &'static [i32] = &[
                // State 0
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                2, // on Expr, goto 1
                0, // on Expr?, error
                3, // on Factor, goto 2
                4, // on Term, goto 3
                0, // on __Expr, error
                // State 1
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 2
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 3
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 4
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                12, // on Expr, goto 11
                0, // on Expr?, error
                13, // on Factor, goto 12
                14, // on Term, goto 13
                0, // on __Expr, error
                // State 5
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 6
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 7
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                19, // on Factor, goto 18
                4, // on Term, goto 3
                0, // on __Expr, error
                // State 8
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                20, // on Factor, goto 19
                4, // on Term, goto 3
                0, // on __Expr, error
                // State 9
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                21, // on Term, goto 20
                0, // on __Expr, error
                // State 10
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                22, // on Term, goto 21
                0, // on __Expr, error
                // State 11
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 12
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 13
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 14
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                28, // on Expr, goto 27
                0, // on Expr?, error
                13, // on Factor, goto 12
                14, // on Term, goto 13
                0, // on __Expr, error
                // State 15
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 16
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 17
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                30, // on (<Expr> ",")+, goto 29
                31, // on Comma<Expr>, goto 30
                32, // on Expr, goto 31
                0, // on Expr?, error
                33, // on Factor, goto 32
                34, // on Term, goto 33
                0, // on __Expr, error
                // State 18
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 19
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 20
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 21
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 22
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 23
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                38, // on Factor, goto 37
                14, // on Term, goto 13
                0, // on __Expr, error
                // State 24
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                39, // on Factor, goto 38
                14, // on Term, goto 13
                0, // on __Expr, error
                // State 25
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                40, // on Term, goto 39
                0, // on __Expr, error
                // State 26
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                41, // on Term, goto 40
                0, // on __Expr, error
                // State 27
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 28
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                30, // on (<Expr> ",")+, goto 29
                43, // on Comma<Expr>, goto 42
                32, // on Expr, goto 31
                0, // on Expr?, error
                33, // on Factor, goto 32
                34, // on Term, goto 33
                0, // on __Expr, error
                // State 29
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                44, // on Expr, goto 43
                0, // on Expr?, error
                33, // on Factor, goto 32
                34, // on Term, goto 33
                0, // on __Expr, error
                // State 30
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 31
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 32
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 33
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 34
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                51, // on Expr, goto 50
                0, // on Expr?, error
                13, // on Factor, goto 12
                14, // on Term, goto 13
                0, // on __Expr, error
                // State 35
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 36
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 37
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 38
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 39
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 40
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 41
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 42
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 43
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 44
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 45
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                55, // on Factor, goto 54
                34, // on Term, goto 33
                0, // on __Expr, error
                // State 46
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 47
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                56, // on Factor, goto 55
                34, // on Term, goto 33
                0, // on __Expr, error
                // State 48
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                57, // on Term, goto 56
                0, // on __Expr, error
                // State 49
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                58, // on Term, goto 57
                0, // on __Expr, error
                // State 50
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 51
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                30, // on (<Expr> ",")+, goto 29
                60, // on Comma<Expr>, goto 59
                32, // on Expr, goto 31
                0, // on Expr?, error
                33, // on Factor, goto 32
                34, // on Term, goto 33
                0, // on __Expr, error
                // State 52
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 53
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 54
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 55
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 56
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 57
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 58
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 59
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
                // State 60
                0, // on (<Expr> ","), error
                0, // on (<Expr> ",")*, error
                0, // on (<Expr> ",")+, error
                0, // on Comma<Expr>, error
                0, // on Expr, error
                0, // on Expr?, error
                0, // on Factor, error
                0, // on Term, error
                0, // on __Expr, error
            ];
            pub fn parse_Expr<
                'ast,
                __TOKEN: __ToTriple<'ast, Error=()>,
                __TOKENS: IntoIterator<Item=__TOKEN>,
            >(
                arena: &'ast Arena<'ast>,
                __tokens0: __TOKENS,
            ) -> Result<&'ast Node<'ast>, __lalrpop_util::ParseError<usize,Tok,()>>
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
                        (_, Tok::Comma, _) if true => 4,
                        (_, Tok::Minus, _) if true => 5,
                        (_, Tok::Div, _) if true => 6,
                        (_, Tok::Num(_), _) if true => 7,
                        _ => {
                            return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: vec![],
                            });
                        }
                    };
                    loop {
                        let __state = *__states.last().unwrap() as usize;
                        let __action = __ACTION[__state * 8 + __integer];
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
                                    __tok @ Tok::Comma => __Symbol::Term_22_2c_22(__tok),
                                    _ => unreachable!(),
                                },
                                5 => match __lookahead.1 {
                                    __tok @ Tok::Minus => __Symbol::Term_22_2d_22(__tok),
                                    _ => unreachable!(),
                                },
                                6 => match __lookahead.1 {
                                    __tok @ Tok::Div => __Symbol::Term_22_2f_22(__tok),
                                    _ => unreachable!(),
                                },
                                7 => match __lookahead.1 {
                                    Tok::Num(__tok0) => __Symbol::TermNum(__tok0),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            __states.push(__action - 1);
                            __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                            continue '__shift;
                        } else if __action < 0 {
                            if let Some(r) = __reduce(arena, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                        if let Some(r) = __reduce(arena, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __action: i32,
                __lookahead_start: Option<&usize>,
                __states: &mut ::std::vec::Vec<i32>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
                _: ::std::marker::PhantomData<()>,
            ) -> Option<Result<&'ast Node<'ast>,__lalrpop_util::ParseError<usize,Tok,()>>>
            {
                let __nonterminal = match -__action {
                    1 => {
                        // (<Expr> ",") = Expr, "," => ActionFn(15);
                        let __sym1 = __pop_Term_22_2c_22(__symbols);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action15::<>(arena, __sym0, __sym1);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 2);
                        __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                        0
                    }
                    2 => {
                        // (<Expr> ",")* =  => ActionFn(13);
                        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action13::<>(arena, &__start, &__end);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 0);
                        __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                        1
                    }
                    3 => {
                        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(14);
                        let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action14::<>(arena, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                        1
                    }
                    4 => {
                        // (<Expr> ",")+ = Expr, "," => ActionFn(18);
                        let __sym1 = __pop_Term_22_2c_22(__symbols);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action18::<>(arena, __sym0, __sym1);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 2);
                        __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                        2
                    }
                    5 => {
                        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(19);
                        let __sym2 = __pop_Term_22_2c_22(__symbols);
                        let __sym1 = __pop_NtExpr(__symbols);
                        let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action19::<>(arena, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                        2
                    }
                    6 => {
                        // Comma<Expr> = Expr => ActionFn(22);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action22::<>(arena, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                        3
                    }
                    7 => {
                        // Comma<Expr> =  => ActionFn(23);
                        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action23::<>(arena, &__start, &__end);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 0);
                        __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                        3
                    }
                    8 => {
                        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(24);
                        let __sym1 = __pop_NtExpr(__symbols);
                        let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = super::super::super::__action24::<>(arena, __sym0, __sym1);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 2);
                        __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                        3
                    }
                    9 => {
                        // Comma<Expr> = (<Expr> ",")+ => ActionFn(25);
                        let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action25::<>(arena, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                        3
                    }
                    10 => {
                        // Expr = Expr, "-", Factor => ActionFn(1);
                        let __sym2 = __pop_NtFactor(__symbols);
                        let __sym1 = __pop_Term_22_2d_22(__symbols);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action1::<>(arena, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                        4
                    }
                    11 => {
                        // Expr = Expr, "+", Factor => ActionFn(2);
                        let __sym2 = __pop_NtFactor(__symbols);
                        let __sym1 = __pop_Term_22_2b_22(__symbols);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<>(arena, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                        4
                    }
                    12 => {
                        // Expr = Factor => ActionFn(3);
                        let __sym0 = __pop_NtFactor(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<>(arena, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                        4
                    }
                    13 => {
                        // Expr? = Expr => ActionFn(11);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action11::<>(arena, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                        5
                    }
                    14 => {
                        // Expr? =  => ActionFn(12);
                        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action12::<>(arena, &__start, &__end);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 0);
                        __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                        5
                    }
                    15 => {
                        // Factor = Factor, "*", Term => ActionFn(4);
                        let __sym2 = __pop_NtTerm(__symbols);
                        let __sym1 = __pop_Term_22_2a_22(__symbols);
                        let __sym0 = __pop_NtFactor(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action4::<>(arena, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                        6
                    }
                    16 => {
                        // Factor = Factor, "/", Term => ActionFn(5);
                        let __sym2 = __pop_NtTerm(__symbols);
                        let __sym1 = __pop_Term_22_2f_22(__symbols);
                        let __sym0 = __pop_NtFactor(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action5::<>(arena, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                        6
                    }
                    17 => {
                        // Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);
                        let __sym3 = __pop_Term_22_29_22(__symbols);
                        let __sym2 = __pop_NtComma_3cExpr_3e(__symbols);
                        let __sym1 = __pop_Term_22_28_22(__symbols);
                        let __sym0 = __pop_Term_22_2a_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym3.2.clone();
                        let __nt = super::super::super::__action6::<>(arena, __sym0, __sym1, __sym2, __sym3);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 4);
                        __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                        6
                    }
                    18 => {
                        // Factor = Term => ActionFn(7);
                        let __sym0 = __pop_NtTerm(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action7::<>(arena, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                        6
                    }
                    19 => {
                        // Term = Num => ActionFn(8);
                        let __sym0 = __pop_TermNum(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action8::<>(arena, __sym0);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 1);
                        __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                        7
                    }
                    20 => {
                        // Term = "(", Expr, ")" => ActionFn(9);
                        let __sym2 = __pop_Term_22_29_22(__symbols);
                        let __sym1 = __pop_NtExpr(__symbols);
                        let __sym0 = __pop_Term_22_28_22(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action9::<>(arena, __sym0, __sym1, __sym2);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 3);
                        __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                        7
                    }
                    21 => {
                        // __Expr = Expr => ActionFn(0);
                        let __sym0 = __pop_NtExpr(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<>(arena, __sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 9 + __nonterminal] - 1;
                __states.push(__next_state);
                None
            }
            fn __pop_Term_22_28_22<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, Tok, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_29_22<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, Tok, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2a_22<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, Tok, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2b_22<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, Tok, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2c_22<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, Tok, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2d_22<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, Tok, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2f_22<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, Tok, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_TermNum<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, i32, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::TermNum(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, &'ast Node<'ast>, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtComma_3cExpr_3e<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, Vec<&'ast Node<'ast>>, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtExpr<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, &'ast Node<'ast>, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtExpr_3f<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, ::std::option::Option<&'ast Node<'ast>>, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtExpr_3f(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtFactor<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, &'ast Node<'ast>, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtTerm<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, &'ast Node<'ast>, usize)
            {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____Expr<
                'ast,
            >(
                arena: &'ast Arena<'ast>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
            ) -> (usize, &'ast Node<'ast>, usize)
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
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, l, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
    (_, r, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Sub, l, r))
}

#[allow(unused_variables)]
pub fn __action2<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, l, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
    (_, r, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Add, l, r))
}

#[allow(unused_variables)]
pub fn __action3<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, l, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
    (_, r, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Mul, l, r))
}

#[allow(unused_variables)]
pub fn __action5<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, l, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
    (_, r, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Div, l, r))
}

#[allow(unused_variables)]
pub fn __action6<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, _, _): (usize, Tok, usize),
    (_, _, _): (usize, Tok, usize),
    (_, __0, _): (usize, Vec<&'ast Node<'ast>>, usize),
    (_, _, _): (usize, Tok, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Reduce(Op::Mul, __0))
}

#[allow(unused_variables)]
pub fn __action7<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action8<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, n, _): (usize, i32, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Value(n))
}

#[allow(unused_variables)]
pub fn __action9<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, _, _): (usize, Tok, usize),
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Paren(__0))
}

#[allow(unused_variables)]
pub fn __action10<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, h, _): (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
    (_, t, _): (usize, ::std::option::Option<&'ast Node<'ast>>, usize),
) -> Vec<&'ast Node<'ast>>
{
    h.into_iter().chain(t).collect()
}

#[allow(unused_variables)]
pub fn __action11<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
) -> ::std::option::Option<&'ast Node<'ast>>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action12<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<&'ast Node<'ast>>
{
    None
}

#[allow(unused_variables)]
pub fn __action13<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action14<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, v, _): (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    v
}

#[allow(unused_variables)]
pub fn __action15<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
) -> &'ast Node<'ast>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action16<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action17<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, v, _): (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
    (_, e, _): (usize, &'ast Node<'ast>, usize),
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action18<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, &'ast Node<'ast>, usize),
    __1: (usize, Tok, usize),
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action15(
        arena,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        arena,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action19<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
    __1: (usize, &'ast Node<'ast>, usize),
    __2: (usize, Tok, usize),
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action15(
        arena,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        arena,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action20<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, ::std::option::Option<&'ast Node<'ast>>, usize),
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action13(
        arena,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        arena,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
pub fn __action21<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
    __1: (usize, ::std::option::Option<&'ast Node<'ast>>, usize),
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action14(
        arena,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        arena,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action22<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, &'ast Node<'ast>, usize),
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action11(
        arena,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        arena,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action23<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action12(
        arena,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        arena,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action24<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
    __1: (usize, &'ast Node<'ast>, usize),
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action11(
        arena,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        arena,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action25<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action12(
        arena,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        arena,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'ast, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),Self::Error>;
}

impl<'ast, > __ToTriple<'ast, > for (usize, Tok, usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),()> {
        Ok(value)
    }
}
impl<'ast, > __ToTriple<'ast, > for Result<(usize, Tok, usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),()> {
        value
    }
}
