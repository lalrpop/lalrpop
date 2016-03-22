#![allow(unused_variables)]
use std::str::FromStr;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Expr<
        'input,
    >(
        input: &'input str,
    ) -> Result<i32, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Expr(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        Factor(i32),
        ____Expr(i32),
        Expr(i32),
        Num(i32),
        Term(i32),
    }

    // State 0
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    //   Expr = (*) Expr "+" Factor [EOF]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [EOF]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [EOF]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   __Expr = (*) Expr [EOF]
    //
    //   r#"[0-9]+"# -> Shift(S2)
    //   "(" -> Shift(S6)
    //
    //   Factor -> S1
    //   Expr -> S5
    //   Num -> S3
    //   Term -> S4
=======
    //     Kind = None
    //     AllInputs = []
    //     OptionalInputs = []
    //     FixedInputs = []
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = (*) Expr "+" Factor [EOF]
    //     Expr = (*) Expr "+" Factor ["+"]
    //     Expr = (*) Expr "+" Factor ["-"]
    //     Expr = (*) Expr "-" Factor [EOF]
    //     Expr = (*) Expr "-" Factor ["+"]
    //     Expr = (*) Expr "-" Factor ["-"]
    //     Expr = (*) Factor [EOF]
    //     Expr = (*) Factor ["+"]
    //     Expr = (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [EOF]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [EOF]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [EOF]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) r#"[0-9]+"# [EOF]
    //     Term = (*) r#"[0-9]+"# ["*"]
    //     Term = (*) r#"[0-9]+"# ["+"]
    //     Term = (*) r#"[0-9]+"# ["-"]
    //     Term = (*) r#"[0-9]+"# ["/"]
    //     __Expr = (*) Expr [EOF]
    //
    //     "(" -> Shift(S4)
    //     r#"[0-9]+"# -> Shift(S5)
    //
    //     Expr -> S1
    //     Factor -> S2
    //     Term -> S3
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state2(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0));
=======
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym0 = (__loc1, (__tok0), __loc2);
                __result = try!(__state4(input, __tokens, __sym0));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym0 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym0));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0));
=======
                __Nonterminal::Expr(__sym0) => {
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Factor(__sym0) => {
                    __result = try!(__state2(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__sym0) => {
                    __result = try!(__custom0(input, __tokens, __lookahead, __sym0));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    //   Expr = Factor (*) [EOF]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "*" -> Shift(S7)
    //   EOF -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "/" -> Shift(S8)
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
=======
    //     Kind = None
    //     AllInputs = [Expr]
    //     OptionalInputs = []
    //     FixedInputs = [Expr]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr (*) "+" Factor [EOF]
    //     Expr = Expr (*) "+" Factor ["+"]
    //     Expr = Expr (*) "+" Factor ["-"]
    //     Expr = Expr (*) "-" Factor [EOF]
    //     Expr = Expr (*) "-" Factor ["+"]
    //     Expr = Expr (*) "-" Factor ["-"]
    //     __Expr = Expr (*) [EOF]
    //
    //     EOF -> Reduce(__Expr = Expr => ActionFn(0);)
    //     "+" -> Shift(S6)
    //     "-" -> Shift(S7)
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None |
            Some((_, (0, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
=======
            Some((__loc1, (3, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state6(input, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state7(input, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __nt = __Nonterminal::____Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
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
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    //   Num = r#"[0-9]+"# (*) [EOF]
    //   Num = r#"[0-9]+"# (*) ["*"]
    //   Num = r#"[0-9]+"# (*) ["+"]
    //   Num = r#"[0-9]+"# (*) ["-"]
    //   Num = r#"[0-9]+"# (*) ["/"]
    //
    //   "/" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(9));)
    //   EOF -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(9));)
    //   "-" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(9));)
    //   "+" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(9));)
    //   "*" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(9));)
=======
    //     Kind = None
    //     AllInputs = [Factor]
    //     OptionalInputs = []
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Factor (*) [EOF]
    //     Expr = Factor (*) ["+"]
    //     Expr = Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [EOF]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [EOF]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     EOF -> Reduce(Expr = Factor => ActionFn(3);)
    //     "*" -> Shift(S8)
    //     "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "/" -> Shift(S9)
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (4, _), _)) |
            None |
            Some((_, (3, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Num(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
=======
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state8(input, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state9(input, __tokens, __sym0, __sym1));
                return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
        }
    }

    // State 3
    //   Term = Num (*) [EOF]
    //   Term = Num (*) ["*"]
    //   Term = Num (*) ["+"]
    //   Term = Num (*) ["-"]
    //   Term = Num (*) ["/"]
    //
    //   "+" -> Reduce(Term = Num => Call(ActionFn(7));)
    //   EOF -> Reduce(Term = Num => Call(ActionFn(7));)
    //   "/" -> Reduce(Term = Num => Call(ActionFn(7));)
    //   "-" -> Reduce(Term = Num => Call(ActionFn(7));)
    //   "*" -> Reduce(Term = Num => Call(ActionFn(7));)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            None |
            Some((_, (4, _), _)) |
            Some((_, (3, _), _)) |
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
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
    //   Factor = Term (*) [EOF]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   EOF -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, _), _)) |
            Some((_, (0, _), _)) |
            None |
            Some((_, (5, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
=======
            Some((_, (4, _), _)) => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    // State 5
    //   Expr = Expr (*) "+" Factor [EOF]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [EOF]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   __Expr = Expr (*) [EOF]
    //
    //   "-" -> Shift(S9)
    //   EOF -> Reduce(__Expr = Expr => Call(ActionFn(0));)
    //   "+" -> Shift(S10)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 6
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Num = (*) r#"[0-9]+"# [")"]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [EOF]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //
    //   r#"[0-9]+"# -> Shift(S16)
    //   "(" -> Shift(S14)
    //
    //   Term -> S12
    //   Num -> S11
    //   Expr -> S15
    //   Factor -> S13
    pub fn __state6<
=======
    // State 4
    //     Kind = None
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
    //     Term = "(" (*) Expr ")" [EOF]
    //     Term = "(" (*) Expr ")" ["*"]
    //     Term = "(" (*) Expr ")" ["+"]
    //     Term = "(" (*) Expr ")" ["-"]
    //     Term = "(" (*) Expr ")" ["/"]
    //     Term = (*) r#"[0-9]+"# [")"]
    //     Term = (*) r#"[0-9]+"# ["*"]
    //     Term = (*) r#"[0-9]+"# ["+"]
    //     Term = (*) r#"[0-9]+"# ["-"]
    //     Term = (*) r#"[0-9]+"# ["/"]
    //
    //     "(" -> Shift(S13)
    //     r#"[0-9]+"# -> Shift(S14)
    //
    //     Expr -> S10
    //     Factor -> S11
    //     Term -> S12
    pub fn __state4<
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
=======
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state13(input, __tokens, __sym1));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym1));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym1));
=======
        loop {
            if __sym0.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__sym1) => {
                    __result = try!(__state10(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__sym1) => {
                    __result = try!(__state11(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__sym1) => {
                    __result = try!(__custom0(input, __tokens, __lookahead, __sym1));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        return Ok(__result);
    }

    // State 7
    //   Factor = Factor "*" (*) Term [EOF]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   r#"[0-9]+"# -> Shift(S2)
    //   "(" -> Shift(S6)
    //
    //   Term -> S17
    //   Num -> S3
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state2(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 8
    //   Factor = Factor "/" (*) Term [EOF]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S6)
    //   r#"[0-9]+"# -> Shift(S2)
    //
    //   Term -> S18
    //   Num -> S3
    pub fn __state8<
=======
    }

    // State 6
    //     Kind = None
    //     AllInputs = [Expr, "+"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "+"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "+" (*) Factor [EOF]
    //     Expr = Expr "+" (*) Factor ["+"]
    //     Expr = Expr "+" (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [EOF]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [EOF]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [EOF]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) r#"[0-9]+"# [EOF]
    //     Term = (*) r#"[0-9]+"# ["*"]
    //     Term = (*) r#"[0-9]+"# ["+"]
    //     Term = (*) r#"[0-9]+"# ["-"]
    //     Term = (*) r#"[0-9]+"# ["/"]
    //
    //     "(" -> Shift(S4)
    //     r#"[0-9]+"# -> Shift(S5)
    //
    //     Factor -> S15
    //     Term -> S3
    pub fn __state6<
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: (usize, F, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state2(input, __lookbehind, __tokens, __sym2));
=======
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state4(input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym2));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
=======
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state15(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom0(input, __tokens, __lookahead, __sym2));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    // State 9
    //   Expr = Expr "-" (*) Factor [EOF]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S6)
    //   r#"[0-9]+"# -> Shift(S2)
    //
    //   Factor -> S19
    //   Term -> S4
    //   Num -> S3
    pub fn __state9<
=======
    // State 7
    //     Kind = None
    //     AllInputs = [Expr, "-"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "-"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "-" (*) Factor [EOF]
    //     Expr = Expr "-" (*) Factor ["+"]
    //     Expr = Expr "-" (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [EOF]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [EOF]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [EOF]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) r#"[0-9]+"# [EOF]
    //     Term = (*) r#"[0-9]+"# ["*"]
    //     Term = (*) r#"[0-9]+"# ["+"]
    //     Term = (*) r#"[0-9]+"# ["-"]
    //     Term = (*) r#"[0-9]+"# ["/"]
    //
    //     "(" -> Shift(S4)
    //     r#"[0-9]+"# -> Shift(S5)
    //
    //     Factor -> S16
    //     Term -> S3
    pub fn __state7<
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: (usize, F, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state2(input, __lookbehind, __tokens, __sym2));
=======
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state4(input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym2));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
=======
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state16(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom0(input, __tokens, __lookahead, __sym2));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    // State 10
    //   Expr = Expr "+" (*) Factor [EOF]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S6)
    //   r#"[0-9]+"# -> Shift(S2)
    //
    //   Term -> S4
    //   Num -> S3
    //   Factor -> S20
    pub fn __state10<
=======
    // State 8
    //     Kind = None
    //     AllInputs = [Factor, "*"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "*"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "*" (*) Term [EOF]
    //     Factor = Factor "*" (*) Term ["*"]
    //     Factor = Factor "*" (*) Term ["+"]
    //     Factor = Factor "*" (*) Term ["-"]
    //     Factor = Factor "*" (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) r#"[0-9]+"# [EOF]
    //     Term = (*) r#"[0-9]+"# ["*"]
    //     Term = (*) r#"[0-9]+"# ["+"]
    //     Term = (*) r#"[0-9]+"# ["-"]
    //     Term = (*) r#"[0-9]+"# ["/"]
    //
    //     "(" -> Shift(S4)
    //     r#"[0-9]+"# -> Shift(S5)
    //
    //     Term -> S17
    pub fn __state8<
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: (usize, F, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state2(input, __lookbehind, __tokens, __sym2));
=======
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state4(input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym2));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
=======
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom2(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    // State 11
    //   Term = Num (*) [")"]
    //   Term = Num (*) ["*"]
    //   Term = Num (*) ["+"]
    //   Term = Num (*) ["-"]
    //   Term = Num (*) ["/"]
    //
    //   "-" -> Reduce(Term = Num => Call(ActionFn(7));)
    //   "+" -> Reduce(Term = Num => Call(ActionFn(7));)
    //   ")" -> Reduce(Term = Num => Call(ActionFn(7));)
    //   "*" -> Reduce(Term = Num => Call(ActionFn(7));)
    //   "/" -> Reduce(Term = Num => Call(ActionFn(7));)
    //
    pub fn __state11<
=======
    // State 9
    //     Kind = None
    //     AllInputs = [Factor, "/"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "/"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "/" (*) Term [EOF]
    //     Factor = Factor "/" (*) Term ["*"]
    //     Factor = Factor "/" (*) Term ["+"]
    //     Factor = Factor "/" (*) Term ["-"]
    //     Factor = Factor "/" (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) r#"[0-9]+"# [EOF]
    //     Term = (*) r#"[0-9]+"# ["*"]
    //     Term = (*) r#"[0-9]+"# ["+"]
    //     Term = (*) r#"[0-9]+"# ["-"]
    //     Term = (*) r#"[0-9]+"# ["/"]
    //
    //     "(" -> Shift(S4)
    //     r#"[0-9]+"# -> Shift(S5)
    //
    //     Term -> S18
    pub fn __state9<
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: (usize, F, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (3, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
=======
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state4(input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym2));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    }

    // State 12
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   ")" -> Reduce(Factor = Term => Call(ActionFn(6));)
=======
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom3(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 10
    //     Kind = None
    //     AllInputs = ["(", Expr]
    //     OptionalInputs = ["("]
    //     FixedInputs = [Expr]
    //     WillPushLen = 1
    //     WillPush = [")"]
    //     WillProduce = None
    //
    //     Expr = Expr (*) "+" Factor [")"]
    //     Expr = Expr (*) "+" Factor ["+"]
    //     Expr = Expr (*) "+" Factor ["-"]
    //     Expr = Expr (*) "-" Factor [")"]
    //     Expr = Expr (*) "-" Factor ["+"]
    //     Expr = Expr (*) "-" Factor ["-"]
    //     Term = "(" Expr (*) ")" [EOF]
    //     Term = "(" Expr (*) ")" ["*"]
    //     Term = "(" Expr (*) ")" ["+"]
    //     Term = "(" Expr (*) ")" ["-"]
    //     Term = "(" Expr (*) ")" ["/"]
    //
    //     ")" -> Shift(S19)
    //     "+" -> Shift(S20)
    //     "-" -> Shift(S21)
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
=======
            Some((__loc1, (1, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                let __sym0 = __sym0.take().unwrap();
                __result = try!(__custom4(input, __tokens, __sym0, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state20(input, __tokens, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state21(input, __tokens, __sym1, __sym2));
                return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    // State 13
    //   Expr = Factor (*) [")"]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "/" -> Shift(S21)
    //   "*" -> Shift(S22)
    //   ")" -> Reduce(Expr = Factor => Call(ActionFn(3));)
=======
    // State 11
    //     Kind = None
    //     AllInputs = [Factor]
    //     OptionalInputs = []
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Factor (*) [")"]
    //     Expr = Factor (*) ["+"]
    //     Expr = Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [")"]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [")"]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     ")" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "*" -> Shift(S22)
    //     "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "/" -> Shift(S23)
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym0, __sym1));
=======
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state22(input, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state23(input, __tokens, __sym0, __sym1));
                return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            Some((_, (0, _), _)) |
            Some((_, (3, _), _)) |
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
=======
            Some((_, (4, _), _)) => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        return Ok(__result);
    }

    // State 14
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Num = (*) r#"[0-9]+"# [")"]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [")"]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //
    //   "(" -> Shift(S14)
    //   r#"[0-9]+"# -> Shift(S16)
    //
    //   Factor -> S13
    //   Expr -> S23
    //   Term -> S12
    //   Num -> S11
    pub fn __state14<
=======
    }

    // State 13
    //     Kind = None
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
    //     Term = "(" (*) Expr ")" [")"]
    //     Term = "(" (*) Expr ")" ["*"]
    //     Term = "(" (*) Expr ")" ["+"]
    //     Term = "(" (*) Expr ")" ["-"]
    //     Term = "(" (*) Expr ")" ["/"]
    //     Term = (*) r#"[0-9]+"# [")"]
    //     Term = (*) r#"[0-9]+"# ["*"]
    //     Term = (*) r#"[0-9]+"# ["+"]
    //     Term = (*) r#"[0-9]+"# ["-"]
    //     Term = (*) r#"[0-9]+"# ["/"]
    //
    //     "(" -> Shift(S13)
    //     r#"[0-9]+"# -> Shift(S14)
    //
    //     Expr -> S24
    //     Factor -> S11
    //     Term -> S12
    pub fn __state13<
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym1));
=======
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state13(input, __tokens, __sym1));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym1));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
=======
        loop {
            if __sym0.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__sym1) => {
                    __result = try!(__state24(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__sym1) => {
                    __result = try!(__state11(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__sym1) => {
                    __result = try!(__custom0(input, __tokens, __lookahead, __sym1));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        return Ok(__result);
    }

    // State 15
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [EOF]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   "+" -> Shift(S26)
    //   "-" -> Shift(S25)
    //   ")" -> Shift(S24)
=======
    }

    // State 15
    //     Kind = None
    //     AllInputs = [Expr, "+", Factor]
    //     OptionalInputs = [Expr, "+"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "+" Factor (*) [EOF]
    //     Expr = Expr "+" Factor (*) ["+"]
    //     Expr = Expr "+" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [EOF]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [EOF]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     EOF -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "*" -> Shift(S8)
    //     "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "/" -> Shift(S9)
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    //
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: &mut Option<(usize, F, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state25(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state24(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
=======
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state8(input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state9(input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 16
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    //   Num = r#"[0-9]+"# (*) [")"]
    //   Num = r#"[0-9]+"# (*) ["*"]
    //   Num = r#"[0-9]+"# (*) ["+"]
    //   Num = r#"[0-9]+"# (*) ["-"]
    //   Num = r#"[0-9]+"# (*) ["/"]
    //
    //   "/" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(9));)
    //   "-" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(9));)
    //   ")" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(9));)
    //   "*" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(9));)
    //   "+" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(9));)
=======
    //     Kind = None
    //     AllInputs = [Expr, "-", Factor]
    //     OptionalInputs = [Expr, "-"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "-" Factor (*) [EOF]
    //     Expr = Expr "-" Factor (*) ["+"]
    //     Expr = Expr "-" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [EOF]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [EOF]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     EOF -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "*" -> Shift(S8)
    //     "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "/" -> Shift(S9)
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, F, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (4, _), _)) |
=======
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state8(input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state9(input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            None |
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            Some((_, (3, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Num(__nt)));
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
    //   Factor = Factor "*" Term (*) [EOF]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   EOF -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (5, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
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
    //   Factor = Factor "/" Term (*) [EOF]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   EOF -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (0, _), _)) |
            None |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
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
    //   Expr = Expr "-" Factor (*) [EOF]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(2));)
    //   EOF -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(2));)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(2));)
    //   "*" -> Shift(S7)
    //   "/" -> Shift(S8)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (0, _), _)) |
            None |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
=======
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 20
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    //   Expr = Expr "+" Factor (*) [EOF]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(1));)
    //   "/" -> Shift(S8)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(1));)
    //   "*" -> Shift(S7)
    //   EOF -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(1));)
    //
=======
    //     Kind = None
    //     AllInputs = [Expr, "+"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "+"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "+" (*) Factor [")"]
    //     Expr = Expr "+" (*) Factor ["+"]
    //     Expr = Expr "+" (*) Factor ["-"]
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
    //     Term = (*) r#"[0-9]+"# [")"]
    //     Term = (*) r#"[0-9]+"# ["*"]
    //     Term = (*) r#"[0-9]+"# ["+"]
    //     Term = (*) r#"[0-9]+"# ["-"]
    //     Term = (*) r#"[0-9]+"# ["/"]
    //
    //     "(" -> Shift(S13)
    //     r#"[0-9]+"# -> Shift(S14)
    //
    //     Factor -> S25
    //     Term -> S12
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (3, _), _)) |
            Some((_, (0, _), _)) |
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
=======
        __sym0: (usize, F, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
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
                __result = try!(__state13(input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym2));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        return Ok(__result);
    }

    // State 21
    //   Factor = Factor "/" (*) Term [")"]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Num = (*) r#"[0-9]+"# [")"]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S14)
    //   r#"[0-9]+"# -> Shift(S16)
    //
    //   Num -> S11
    //   Term -> S27
=======
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state25(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom0(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 21
    //     Kind = None
    //     AllInputs = [Expr, "-"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "-"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "-" (*) Factor [")"]
    //     Expr = Expr "-" (*) Factor ["+"]
    //     Expr = Expr "-" (*) Factor ["-"]
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
    //     Term = (*) r#"[0-9]+"# [")"]
    //     Term = (*) r#"[0-9]+"# ["*"]
    //     Term = (*) r#"[0-9]+"# ["+"]
    //     Term = (*) r#"[0-9]+"# ["-"]
    //     Term = (*) r#"[0-9]+"# ["/"]
    //
    //     "(" -> Shift(S13)
    //     r#"[0-9]+"# -> Shift(S14)
    //
    //     Factor -> S26
    //     Term -> S12
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: (usize, F, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym2));
=======
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state13(input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym2));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
=======
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state26(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom0(input, __tokens, __lookahead, __sym2));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 22
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    //   Factor = Factor "*" (*) Term [")"]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Num = (*) r#"[0-9]+"# [")"]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S14)
    //   r#"[0-9]+"# -> Shift(S16)
    //
    //   Num -> S11
    //   Term -> S28
=======
    //     Kind = None
    //     AllInputs = [Factor, "*"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "*"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "*" (*) Term [")"]
    //     Factor = Factor "*" (*) Term ["*"]
    //     Factor = Factor "*" (*) Term ["+"]
    //     Factor = Factor "*" (*) Term ["-"]
    //     Factor = Factor "*" (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) r#"[0-9]+"# [")"]
    //     Term = (*) r#"[0-9]+"# ["*"]
    //     Term = (*) r#"[0-9]+"# ["+"]
    //     Term = (*) r#"[0-9]+"# ["-"]
    //     Term = (*) r#"[0-9]+"# ["/"]
    //
    //     "(" -> Shift(S13)
    //     r#"[0-9]+"# -> Shift(S14)
    //
    //     Term -> S27
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: (usize, F, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym2));
=======
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state13(input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym2));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
=======
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom2(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 23
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [")"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   "+" -> Shift(S26)
    //   ")" -> Shift(S29)
    //   "-" -> Shift(S25)
    //
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state25(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 24
    //   Term = "(" Expr ")" (*) [EOF]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   EOF -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<i32>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (3, _), _)) |
            None |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 25
    //   Expr = Expr "-" (*) Factor [")"]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Num = (*) r#"[0-9]+"# [")"]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S14)
    //   r#"[0-9]+"# -> Shift(S16)
    //
    //   Factor -> S30
    //   Term -> S12
    //   Num -> S11
    pub fn __state25<
=======
    //     Kind = None
    //     AllInputs = [Factor, "/"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "/"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "/" (*) Term [")"]
    //     Factor = Factor "/" (*) Term ["*"]
    //     Factor = Factor "/" (*) Term ["+"]
    //     Factor = Factor "/" (*) Term ["-"]
    //     Factor = Factor "/" (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) r#"[0-9]+"# [")"]
    //     Term = (*) r#"[0-9]+"# ["*"]
    //     Term = (*) r#"[0-9]+"# ["+"]
    //     Term = (*) r#"[0-9]+"# ["-"]
    //     Term = (*) r#"[0-9]+"# ["/"]
    //
    //     "(" -> Shift(S13)
    //     r#"[0-9]+"# -> Shift(S14)
    //
    //     Term -> S28
    pub fn __state23<
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: (usize, F, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym2));
=======
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state13(input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym2));
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state30(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
=======
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom3(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    // State 26
    //   Expr = Expr "+" (*) Factor [")"]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Num = (*) r#"[0-9]+"# [")"]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   r#"[0-9]+"# -> Shift(S16)
    //   "(" -> Shift(S14)
=======
    // State 24
    //     Kind = None
    //     AllInputs = ["(", Expr]
    //     OptionalInputs = ["("]
    //     FixedInputs = [Expr]
    //     WillPushLen = 1
    //     WillPush = [")"]
    //     WillProduce = None
    //
    //     Expr = Expr (*) "+" Factor [")"]
    //     Expr = Expr (*) "+" Factor ["+"]
    //     Expr = Expr (*) "+" Factor ["-"]
    //     Expr = Expr (*) "-" Factor [")"]
    //     Expr = Expr (*) "-" Factor ["+"]
    //     Expr = Expr (*) "-" Factor ["-"]
    //     Term = "(" Expr (*) ")" [")"]
    //     Term = "(" Expr (*) ")" ["*"]
    //     Term = "(" Expr (*) ")" ["+"]
    //     Term = "(" Expr (*) ")" ["-"]
    //     Term = "(" Expr (*) ")" ["/"]
    //
    //     ")" -> Shift(S29)
    //     "+" -> Shift(S20)
    //     "-" -> Shift(S21)
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    //
    //   Term -> S12
    //   Num -> S11
    //   Factor -> S31
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
=======
            Some((__loc1, (1, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                let __sym0 = __sym0.take().unwrap();
                __result = try!(__custom4(input, __tokens, __sym0, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state20(input, __tokens, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state21(input, __tokens, __sym1, __sym2));
                return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 27
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   ")" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
=======
    }

    // State 25
    //     Kind = None
    //     AllInputs = [Expr, "+", Factor]
    //     OptionalInputs = [Expr, "+"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "+" Factor (*) [")"]
    //     Expr = Expr "+" Factor (*) ["+"]
    //     Expr = Expr "+" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [")"]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [")"]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     ")" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "*" -> Shift(S22)
    //     "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "/" -> Shift(S23)
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    //
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: &mut Option<(usize, F, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
=======
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state22(input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state23(input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
=======
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    // State 28
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
=======
    // State 26
    //     Kind = None
    //     AllInputs = [Expr, "-", Factor]
    //     OptionalInputs = [Expr, "-"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "-" Factor (*) [")"]
    //     Expr = Expr "-" Factor (*) ["+"]
    //     Expr = Expr "-" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [")"]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [")"]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     ")" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "*" -> Shift(S22)
    //     "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "/" -> Shift(S23)
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    //
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
=======
        __sym0: &mut Option<(usize, F, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
=======
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state22(input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state23(input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
=======
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
    }

    // State 29
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   ")" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //
    pub fn __state29<
=======
    }

    // Custom 0
    //    Reduce Factor = Term => ActionFn(6);
    pub fn __custom0<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6(input, __sym0);
        let __nt = __Nonterminal::Factor((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 1
    //    Reduce Term = r#"[0-9]+"# => ActionFn(7);
    pub fn __custom1<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7(input, __sym0);
        let __nt = __Nonterminal::Term((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 2
    //    Reduce Factor = Factor, "*", Term => ActionFn(4);
    pub fn __custom2<
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<i32>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (5, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
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
    //   Expr = Expr "-" Factor (*) [")"]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(2));)
    //   "*" -> Shift(S22)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(2));)
    //   ")" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(2));)
    //   "/" -> Shift(S21)
    //
    pub fn __state30<
=======
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, F, usize),
        __sym1: (usize, &'input str, usize),
        __sym2: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action4(input, __sym0, __sym1, __sym2);
        let __nt = __Nonterminal::Factor((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 3
    //    Reduce Factor = Factor, "/", Term => ActionFn(5);
    pub fn __custom3<
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (3, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 31
    //   Expr = Expr "+" Factor (*) [")"]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "/" -> Shift(S21)
    //   "*" -> Shift(S22)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(1));)
    //   ")" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(1));)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(1));)
    //
    pub fn __state31<
=======
        __sym0: (usize, F, usize),
        __sym1: (usize, &'input str, usize),
        __sym2: (usize, F, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5(input, __sym0, __sym1, __sym2);
        let __nt = __Nonterminal::Factor((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 4
    //    Reduce Term = "(", Expr, ")" => ActionFn(8);
    pub fn __custom4<
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6:doc/calculator/src/calculator3.rs
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (3, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
=======
        __sym0: (usize, &'input str, usize),
        __sym1: (usize, F, usize),
        __sym2: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8(input, __sym0, __sym1, __sym2);
        let __nt = __Nonterminal::Term((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
>>>>>>> update test output:lalrpop-test/src/expr_generic.rs
        return Ok(__result);
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
                    match __ch {
                        '(' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        ')' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '*' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        '+' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        '-' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '/' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((2, __index + 1));
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
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
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
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    l: i32,
    _: &'input str,
    r: i32,
) -> i32
{
    l+r
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    l: i32,
    _: &'input str,
    r: i32,
) -> i32
{
    l-r
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    l: i32,
    _: &'input str,
    r: i32,
) -> i32
{
    l*r
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    l: i32,
    _: &'input str,
    r: i32,
) -> i32
{
    l/r
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: i32,
    _: &'input str,
) -> i32
{
    (__0)
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: &'input str,
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
