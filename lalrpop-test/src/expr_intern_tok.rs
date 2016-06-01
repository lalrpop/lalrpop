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
        let mut __tokens = super::__intern_token::__Matcher::new(input);
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
                let __nt = super::__action0(scale, input, __sym0);
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
                let __nt = super::__action3(scale, input, __sym0);
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
                let __nt = super::__action7(scale, input, __sym0);
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
                let __nt = super::__action6(scale, input, __sym0);
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
                let __nt = super::__action9(scale, input, __sym0);
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
                let __nt = super::__action3(scale, input, __sym0);
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
                let __nt = super::__action7(scale, input, __sym0);
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
                let __nt = super::__action6(scale, input, __sym0);
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
                let __nt = super::__action9(scale, input, __sym0);
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
                let __nt = super::__action2(scale, input, __sym0, __sym1, __sym2);
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
                let __nt = super::__action1(scale, input, __sym0, __sym1, __sym2);
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
                let __nt = super::__action4(scale, input, __sym0, __sym1, __sym2);
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
                let __nt = super::__action5(scale, input, __sym0, __sym1, __sym2);
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
                let __nt = super::__action8(scale, input, __sym0, __sym1, __sym2);
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
                let __nt = super::__action2(scale, input, __sym0, __sym1, __sym2);
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
                let __nt = super::__action1(scale, input, __sym0, __sym1, __sym2);
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
                let __nt = super::__action4(scale, input, __sym0, __sym1, __sym2);
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
                let __nt = super::__action5(scale, input, __sym0, __sym1, __sym2);
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
                let __nt = super::__action8(scale, input, __sym0, __sym1, __sym2);
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
