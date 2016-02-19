#![allow(unused_imports)]
#![allow(unused_variables)]
use expr_arena_ast::{Arena, Node, Op};
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use expr_arena_ast::{Arena, Node, Op};
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_Expr<
        'ast,
        __TOKEN: __ToTriple<'ast, Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        arena: &'ast Arena<'ast>,
        __tokens: __TOKENS,
    ) -> Result<&'ast Node<'ast>, __ParseError<usize,Tok,()>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match try!(__state0(arena, None, &mut __tokens, __lookahead)) {
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
    pub enum __Nonterminal<'ast> {
        _28_3cExpr_3e_20_22_2c_22_29(&'ast Node<'ast>),
        _28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'ast Node<'ast>>),
        _28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'ast Node<'ast>>),
        Comma_3cExpr_3e(Vec<&'ast Node<'ast>>),
        Expr(&'ast Node<'ast>),
        Expr_3f(::std::option::Option<&'ast Node<'ast>>),
        Factor(&'ast Node<'ast>),
        Term(&'ast Node<'ast>),
        ____Expr(&'ast Node<'ast>),
    }

    pub fn __state0<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok));
                __result = try!(__state4(arena, __lookbehind, __tokens, __sym0));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok));
                __result = try!(__state5(arena, __lookbehind, __tokens, __sym0));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state6(arena, __lookbehind, __tokens, __sym0));
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
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(arena, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(arena, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(arena, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Plus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state7(arena, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Minus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state8(arena, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(arena, __sym0, &__lookbehind, &__lookahead);
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

    pub fn __state2<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state9(arena, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Div, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state10(arena, __lookbehind, __tokens, __sym0, __sym1));
            }
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0, &__lookbehind, &__lookahead);
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

    pub fn __state3<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            None |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0, &__lookbehind, &__lookahead);
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

    pub fn __state4<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state14(arena, __lookbehind, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state15(arena, __lookbehind, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(arena, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state5<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state17(arena, __lookbehind, __tokens, __sym0, __sym1));
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

    pub fn __state6<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0, &__lookbehind, &__lookahead);
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

    pub fn __state7<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state4(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state5(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state6(arena, __lookbehind, __tokens, __sym2));
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
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state8<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state4(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state5(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state6(arena, __lookbehind, __tokens, __sym2));
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
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state19(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state9<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state4(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state6(arena, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state20(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state10<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state4(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state6(arena, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state21(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state11<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::RParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state22(arena, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Plus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state23(arena, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Minus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state24(arena, __lookbehind, __tokens, __sym1, __sym2));
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

    pub fn __state12<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state25(arena, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Div, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state26(arena, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0, &__lookbehind, &__lookahead);
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

    pub fn __state13<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0, &__lookbehind, &__lookahead);
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

    pub fn __state14<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state14(arena, __lookbehind, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state15(arena, __lookbehind, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(arena, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state27(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state15<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state28(arena, __lookbehind, __tokens, __sym0, __sym1));
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

    pub fn __state16<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0, &__lookbehind, &__lookahead);
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

    pub fn __state17<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state34(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state35(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state36(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::RParen, _)) => {
                let __nt = super::__action23(arena, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Comma_3cExpr_3e(__nt));
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
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state30(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state32(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state33(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state18<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state9(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state10(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state19<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state9(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state10(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state20<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            None |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state21<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            None |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state22<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state23<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state14(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state15(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(arena, __lookbehind, __tokens, __sym2));
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
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state37(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state24<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state14(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state15(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(arena, __lookbehind, __tokens, __sym2));
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
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state38(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state25<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state14(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(arena, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state39(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state26<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state14(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(arena, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state40(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state27<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::RParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state41(arena, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Plus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state23(arena, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Minus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state24(arena, __lookbehind, __tokens, __sym1, __sym2));
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

    pub fn __state28<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state34(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state35(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state36(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::RParen, _)) => {
                let __nt = super::__action23(arena, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Comma_3cExpr_3e(__nt));
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
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state42(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state32(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state33(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state29<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state34(arena, __lookbehind, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state35(arena, __lookbehind, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state36(arena, __lookbehind, __tokens, __sym1));
            }
            Some((_, Tok::RParen, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action25(arena, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Comma_3cExpr_3e(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state43(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state32(arena, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state33(arena, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state30<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::RParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state44(arena, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
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

    pub fn __state31<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Plus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state45(arena, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Comma, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state46(arena, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Minus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state47(arena, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::RParen, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(arena, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Comma_3cExpr_3e(__nt)));
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

    pub fn __state32<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state48(arena, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Div, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state49(arena, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Comma, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0, &__lookbehind, &__lookahead);
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

    pub fn __state33<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Comma, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0, &__lookbehind, &__lookahead);
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

    pub fn __state34<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state14(arena, __lookbehind, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state15(arena, __lookbehind, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(arena, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state50(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state35<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state51(arena, __lookbehind, __tokens, __sym0, __sym1));
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

    pub fn __state36<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Comma, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0, &__lookbehind, &__lookahead);
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

    pub fn __state37<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state25(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state26(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state38<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state25(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state26(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state39<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state40<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state41<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state42<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::RParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state52(arena, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
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

    pub fn __state43<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Plus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state45(arena, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Comma, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state53(arena, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Minus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state47(arena, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, Tok::RParen, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action24(arena, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Comma_3cExpr_3e(__nt)));
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

    pub fn __state44<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3, &__lookbehind, &__lookahead);
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

    pub fn __state45<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state34(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state35(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state36(arena, __lookbehind, __tokens, __sym2));
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
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state54(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state33(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state46<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action18(arena, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state47<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state34(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state35(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state36(arena, __lookbehind, __tokens, __sym2));
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
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state55(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state33(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state48<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state34(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state36(arena, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state56(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state49<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state34(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state36(arena, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state57(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state50<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::RParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state58(arena, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Plus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state23(arena, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Minus, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state24(arena, __lookbehind, __tokens, __sym1, __sym2));
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

    pub fn __state51<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, __tok @ Tok::LParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state34(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state35(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state36(arena, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::RParen, _)) => {
                let __nt = super::__action23(arena, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Comma_3cExpr_3e(__nt));
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
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state59(arena, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state32(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state33(arena, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state52<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
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
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3, &__lookbehind, &__lookahead);
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

    pub fn __state53<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action19(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state54<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state48(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state49(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Comma, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state55<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::Times, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state48(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state49(arena, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Comma, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state56<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Comma, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state57<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Comma, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state58<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Comma, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    pub fn __state59<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        match __lookahead {
            Some((_, __tok @ Tok::RParen, __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state60(arena, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
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

    pub fn __state60<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Comma, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3, &__lookbehind, &__lookahead);
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
}
pub use self::__parse__Expr::parse_Expr;

pub fn __action0<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action1<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    l: &'ast Node<'ast>,
    _: Tok,
    r: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Sub, l, r))
}

pub fn __action2<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    l: &'ast Node<'ast>,
    _: Tok,
    r: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Add, l, r))
}

pub fn __action3<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action4<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    l: &'ast Node<'ast>,
    _: Tok,
    r: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Mul, l, r))
}

pub fn __action5<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    l: &'ast Node<'ast>,
    _: Tok,
    r: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Div, l, r))
}

pub fn __action6<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    _: Tok,
    _: Tok,
    __0: Vec<&'ast Node<'ast>>,
    _: Tok,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Reduce(Op::Mul, __0))
}

pub fn __action7<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action8<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    n: i32,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Value(n))
}

pub fn __action9<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    _: Tok,
    __0: &'ast Node<'ast>,
    _: Tok,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action10<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    h: ::std::vec::Vec<&'ast Node<'ast>>,
    t: ::std::option::Option<&'ast Node<'ast>>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> Vec<&'ast Node<'ast>>
{
    h.into_iter().chain(t).collect()
}

pub fn __action11<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> ::std::option::Option<&'ast Node<'ast>>
{
    Some(__0)
}

pub fn __action12<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> ::std::option::Option<&'ast Node<'ast>>
{
    None
}

pub fn __action13<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    vec![]
}

pub fn __action14<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    v: ::std::vec::Vec<&'ast Node<'ast>>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    v
}

pub fn __action15<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
    _: Tok,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action16<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    vec![__0]
}

pub fn __action17<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    v: ::std::vec::Vec<&'ast Node<'ast>>,
    e: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action18<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
    __1: Tok,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    let __temp0 = __action15(
        arena,
        __0,
        __1,
        __lookbehind,
        __lookahead,
    );
    __action16(
        arena,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action19<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: ::std::vec::Vec<&'ast Node<'ast>>,
    __1: &'ast Node<'ast>,
    __2: Tok,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    let __temp0 = __action15(
        arena,
        __1,
        __2,
        __lookbehind,
        __lookahead,
    );
    __action17(
        arena,
        __0,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action20<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: ::std::option::Option<&'ast Node<'ast>>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> Vec<&'ast Node<'ast>>
{
    let __temp0 = __action13(
        arena,
        __lookbehind,
        __lookahead,
    );
    __action10(
        arena,
        __temp0,
        __0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action21<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: ::std::vec::Vec<&'ast Node<'ast>>,
    __1: ::std::option::Option<&'ast Node<'ast>>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> Vec<&'ast Node<'ast>>
{
    let __temp0 = __action14(
        arena,
        __0,
        __lookbehind,
        __lookahead,
    );
    __action10(
        arena,
        __temp0,
        __1,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action22<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> Vec<&'ast Node<'ast>>
{
    let __temp0 = __action11(
        arena,
        __0,
        __lookbehind,
        __lookahead,
    );
    __action20(
        arena,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action23<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> Vec<&'ast Node<'ast>>
{
    let __temp0 = __action12(
        arena,
        __lookbehind,
        __lookahead,
    );
    __action20(
        arena,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action24<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: ::std::vec::Vec<&'ast Node<'ast>>,
    __1: &'ast Node<'ast>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> Vec<&'ast Node<'ast>>
{
    let __temp0 = __action11(
        arena,
        __1,
        __lookbehind,
        __lookahead,
    );
    __action21(
        arena,
        __0,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action25<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: ::std::vec::Vec<&'ast Node<'ast>>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, Tok, usize)>,
) -> Vec<&'ast Node<'ast>>
{
    let __temp0 = __action12(
        arena,
        __lookbehind,
        __lookahead,
    );
    __action21(
        arena,
        __0,
        __temp0,
        __lookbehind,
        __lookahead,
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
