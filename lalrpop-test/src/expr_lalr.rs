#![allow(unused_imports)]
#![allow(unused_variables)]
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_Expr<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        scale: i32,
        __tokens: __TOKENS,
    ) -> Result<i32, __ParseError<(),Tok,()>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match try!(__state0(scale, &mut __tokens, __lookahead)) {
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
    //     Expr = (*) Expr "+" Factor ["+", "-", EOF]
    //     Expr = (*) Expr "-" Factor ["+", "-", EOF]
    //     Expr = (*) Factor ["+", "-", EOF]
    //     Factor = (*) Factor "*" Term ["*", "+", "-", "/", EOF]
    //     Factor = (*) Factor "/" Term ["*", "+", "-", "/", EOF]
    //     Factor = (*) Term ["*", "+", "-", "/", EOF]
    //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
    //     Term = (*) Num ["*", "+", "-", "/", EOF]
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
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym0 = (__loc1, (__tok), __loc2);
                __result = try!(__state4(scale, __tokens, __sym0));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym0 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(scale, __tokens, __sym0));
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
                    __result = try!(__state1(scale, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Factor(__sym0) => {
                    __result = try!(__state2(scale, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__sym0) => {
                    __result = try!(__state3(scale, __tokens, __lookahead, __sym0));
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
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state6(scale, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state7(scale, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(scale, __sym0);
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
    //     Expr = Factor (*) [")", "+", "-", EOF]
    //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/", EOF]
    //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/", EOF]
    //
    //   "*" -> S8
    //   "/" -> S9
    //   [")", "+", "-", EOF] -> Expr = Factor => ActionFn(3);
    //
    pub fn __state2<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state8(scale, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state9(scale, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(scale, __sym0);
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
    //     AllInputs = [Term]
    //     OptionalInputs = []
    //     FixedInputs = [Term]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = Some(Factor)
    //
    //     Factor = Term (*) [")", "*", "+", "-", "/", EOF]
    //
    //   [")", "*", "+", "-", "/", EOF] -> Factor = Term => ActionFn(6);
    //
    pub fn __state3<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(scale, __sym0);
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

    // State 4
    //     AllInputs = ["("]
    //     OptionalInputs = []
    //     FixedInputs = ["("]
    //     WillPushLen = 2
    //     WillPush = [Expr, ")"]
    //     WillProduce = Some(Term)
    //
    //     Expr = (*) Expr "+" Factor [")", "+", "-"]
    //     Expr = (*) Expr "-" Factor [")", "+", "-"]
    //     Expr = (*) Factor [")", "+", "-"]
    //     Factor = (*) Factor "*" Term [")", "*", "+", "-", "/"]
    //     Factor = (*) Factor "/" Term [")", "*", "+", "-", "/"]
    //     Factor = (*) Term [")", "*", "+", "-", "/"]
    //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
    //     Term = "(" (*) Expr ")" [")", "*", "+", "-", "/", EOF]
    //     Term = (*) Num [")", "*", "+", "-", "/"]
    //
    //   "(" -> S4
    //   Num -> S5
    //
    //     Expr -> S10
    //     Factor -> S2
    //     Term -> S3
    pub fn __state4<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __sym0: ((), Tok, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        let __sym0 = &mut Some(__sym0);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state4(scale, __tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(scale, __tokens, __sym1));
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
                    __result = try!(__state10(scale, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__sym1) => {
                    __result = try!(__state2(scale, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__sym1) => {
                    __result = try!(__state3(scale, __tokens, __lookahead, __sym1));
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
    //     Term = Num (*) [")", "*", "+", "-", "/", EOF]
    //
    //   [")", "*", "+", "-", "/", EOF] -> Term = Num => ActionFn(7);
    //
    pub fn __state5<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __sym0: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(scale, __sym0);
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

    // State 6
    //     AllInputs = [Expr, "+"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "+"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "+" (*) Factor [")", "+", "-", EOF]
    //     Factor = (*) Factor "*" Term [")", "*", "+", "-", "/", EOF]
    //     Factor = (*) Factor "/" Term [")", "*", "+", "-", "/", EOF]
    //     Factor = (*) Term [")", "*", "+", "-", "/", EOF]
    //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/", EOF]
    //     Term = (*) Num [")", "*", "+", "-", "/", EOF]
    //
    //   "(" -> S4
    //   Num -> S5
    //
    //     Factor -> S11
    //     Term -> S3
    pub fn __state6<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __sym0: ((), i32, ()),
        __sym1: ((), Tok, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state4(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(scale, __tokens, __sym2));
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
                    __result = try!(__state11(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__state3(scale, __tokens, __lookahead, __sym2));
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
    //     Expr = Expr "-" (*) Factor [")", "+", "-", EOF]
    //     Factor = (*) Factor "*" Term [")", "*", "+", "-", "/", EOF]
    //     Factor = (*) Factor "/" Term [")", "*", "+", "-", "/", EOF]
    //     Factor = (*) Term [")", "*", "+", "-", "/", EOF]
    //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/", EOF]
    //     Term = (*) Num [")", "*", "+", "-", "/", EOF]
    //
    //   "(" -> S4
    //   Num -> S5
    //
    //     Factor -> S12
    //     Term -> S3
    pub fn __state7<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __sym0: ((), i32, ()),
        __sym1: ((), Tok, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state4(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(scale, __tokens, __sym2));
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
                    __result = try!(__state12(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__state3(scale, __tokens, __lookahead, __sym2));
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
    //     Factor = Factor "*" (*) Term [")", "*", "+", "-", "/", EOF]
    //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/", EOF]
    //     Term = (*) Num [")", "*", "+", "-", "/", EOF]
    //
    //   "(" -> S4
    //   Num -> S5
    //
    //     Term -> S13
    pub fn __state8<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __sym0: ((), i32, ()),
        __sym1: ((), Tok, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state4(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(scale, __tokens, __sym2));
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
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__state13(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
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
    //     Factor = Factor "/" (*) Term [")", "*", "+", "-", "/", EOF]
    //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/", EOF]
    //     Term = (*) Num [")", "*", "+", "-", "/", EOF]
    //
    //   "(" -> S4
    //   Num -> S5
    //
    //     Term -> S14
    pub fn __state9<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __sym0: ((), i32, ()),
        __sym1: ((), Tok, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state4(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(scale, __tokens, __sym2));
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
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__state14(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
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
    //     Term = "(" Expr (*) ")" [")", "*", "+", "-", "/", EOF]
    //
    //   ")" -> S15
    //   "+" -> S6
    //   "-" -> S7
    //
    pub fn __state10<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), Tok, ())>,
        __sym1: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                let __sym0 = __sym0.take().unwrap();
                __result = try!(__state15(scale, __tokens, __sym0, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state6(scale, __tokens, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state7(scale, __tokens, __sym1, __sym2));
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

    // State 11
    //     AllInputs = [Expr, "+", Factor]
    //     OptionalInputs = [Expr, "+"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "+" Factor (*) [")", "+", "-", EOF]
    //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/", EOF]
    //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/", EOF]
    //
    //   "*" -> S8
    //   "/" -> S9
    //   [")", "+", "-", EOF] -> Expr = Expr, "+", Factor => ActionFn(2);
    //
    pub fn __state11<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
        __sym1: &mut Option<((), Tok, ())>,
        __sym2: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state8(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state9(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(scale, __sym0, __sym1, __sym2);
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

    // State 12
    //     AllInputs = [Expr, "-", Factor]
    //     OptionalInputs = [Expr, "-"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "-" Factor (*) [")", "+", "-", EOF]
    //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/", EOF]
    //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/", EOF]
    //
    //   "*" -> S8
    //   "/" -> S9
    //   [")", "+", "-", EOF] -> Expr = Expr, "-", Factor => ActionFn(1);
    //
    pub fn __state12<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
        __sym1: &mut Option<((), Tok, ())>,
        __sym2: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state8(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state9(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(scale, __sym0, __sym1, __sym2);
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
    //     AllInputs = [Factor, "*", Term]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "*", Term]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "*" Term (*) [")", "*", "+", "-", "/", EOF]
    //
    //   [")", "*", "+", "-", "/", EOF] -> Factor = Factor, "*", Term => ActionFn(4);
    //
    pub fn __state13<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
        __sym1: ((), Tok, ()),
        __sym2: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
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

    // State 14
    //     AllInputs = [Factor, "/", Term]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "/", Term]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "/" Term (*) [")", "*", "+", "-", "/", EOF]
    //
    //   [")", "*", "+", "-", "/", EOF] -> Factor = Factor, "/", Term => ActionFn(5);
    //
    pub fn __state14<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
        __sym1: ((), Tok, ()),
        __sym2: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
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
    //     AllInputs = ["(", Expr, ")"]
    //     OptionalInputs = []
    //     FixedInputs = ["(", Expr, ")"]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = Some(Term)
    //
    //     Term = "(" Expr ")" (*) [")", "*", "+", "-", "/", EOF]
    //
    //   [")", "*", "+", "-", "/", EOF] -> Term = "(", Expr, ")" => ActionFn(8);
    //
    pub fn __state15<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __sym0: ((), Tok, ()),
        __sym1: ((), i32, ()),
        __sym2: ((), Tok, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
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

pub fn __action0<
>(
    scale: i32,
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

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

pub fn __action3<
>(
    scale: i32,
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

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

pub fn __action6<
>(
    scale: i32,
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

pub fn __action7<
>(
    scale: i32,
    (_, n, _): ((), i32, ()),
) -> i32
{
    n * scale
}

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
