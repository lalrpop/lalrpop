#![allow(unused_imports)]
#![allow(unused_variables)]
use expr_arena_ast::{Arena, Node, Op};
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;
#[allow(non_snake_case)]
pub fn parse_Expr<
    'ast,
    __TOKEN: __ToTriple<'ast, Error=()>,
    __TOKENS: IntoIterator<Item=__TOKEN>,
>(
    arena: &'ast Arena<'ast>,
    __tokens: __TOKENS,
) -> Result<&'ast Node<'ast>, __ParseError<usize,Tok,()>>
{
    let mut __tokens = __tokens.into_iter();
    let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
    let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
    match try!(__parse__Expr::__state0(arena, None, __lookahead, &mut __tokens)) {
        (_, Some(__lookahead), _) => {
            Err(__ParseError::ExtraToken { token: __lookahead })
        }
        (_, None, __parse__Expr::__Nonterminal::____Expr(__nt)) => {
            Ok(__nt)
        }
        _ => unreachable!(),
    }
}

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use expr_arena_ast::{Arena, Node, Op};
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;

    #[allow(dead_code)]
    pub enum __Nonterminal<'ast, > {
        Expr(&'ast Node<'ast>),
        Factor(&'ast Node<'ast>),
        Expr_3f(::std::option::Option<&'ast Node<'ast>>),
        Term(&'ast Node<'ast>),
        _28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'ast Node<'ast>>),
        Comma_3cExpr_3e(Vec<&'ast Node<'ast>>),
        ____Expr(&'ast Node<'ast>),
        _28_3cExpr_3e_20_22_2c_22_29(&'ast Node<'ast>),
    }

    // State 0
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
    //   Factor = (*) "*" "(" Comma<Expr> ")" [EOF]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   __Expr = (*) Expr [EOF]
    //
    //   "*" -> Shift(S5)
    //   "(" -> Shift(S6)
    //   Num -> Shift(S1)
    //
    //   Term -> S2
    //   Factor -> S4
    //   Expr -> S3
    pub fn __state0<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state5(arena, __lookbehind, __lookahead, __tokens, __sym0));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state6(arena, __lookbehind, __lookahead, __tokens, __sym0));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state1(arena, __lookbehind, __lookahead, __tokens, __sym0));
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
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(arena, __lookbehind, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(arena, __lookbehind, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(arena, __lookbehind, __lookahead, __tokens, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Term = Num (*) [EOF]
    //   Term = Num (*) ["*"]
    //   Term = Num (*) ["+"]
    //   Term = Num (*) ["-"]
    //   Term = Num (*) ["/"]
    //
    //   EOF -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = Num => Call(ActionFn(8));)
    //
    pub fn __state1<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            None |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
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

    // State 2
    //   Factor = Term (*) [EOF]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   EOF -> Reduce(Factor = Term => Call(ActionFn(7));)
    //
    pub fn __state2<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
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

    // State 3
    //   Expr = Expr (*) "+" Factor [EOF]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [EOF]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   __Expr = Expr (*) [EOF]
    //
    //   EOF -> Reduce(__Expr = Expr => Call(ActionFn(0));)
    //   "-" -> Shift(S8)
    //   "+" -> Shift(S7)
    //
    pub fn __state3<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state8(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state7(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(arena, __sym0);
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

    // State 4
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
    //   EOF -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "/" -> Shift(S10)
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "*" -> Shift(S9)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //
    pub fn __state4<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state10(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state9(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            None |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
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

    // State 5
    //   Factor = "*" (*) "(" Comma<Expr> ")" [EOF]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["*"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["+"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["-"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["/"]
    //
    //   "(" -> Shift(S11)
    //
    pub fn __state5<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state11(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
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
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
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
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   Num -> Shift(S14)
    //   "(" -> Shift(S17)
    //   "*" -> Shift(S12)
    //
    //   Expr -> S15
    //   Factor -> S16
    //   Term -> S13
    pub fn __state6<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state17(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state12(arena, __lookbehind, __lookahead, __tokens, __sym1));
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
                    __result = try!(__state15(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state16(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
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
    //   Factor = (*) "*" "(" Comma<Expr> ")" [EOF]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S6)
    //   "*" -> Shift(S5)
    //   Num -> Shift(S1)
    //
    //   Factor -> S18
    //   Term -> S2
    pub fn __state7<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state6(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state5(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state1(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state18(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 8
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
    //   Factor = (*) "*" "(" Comma<Expr> ")" [EOF]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   Num -> Shift(S1)
    //   "*" -> Shift(S5)
    //   "(" -> Shift(S6)
    //
    //   Term -> S2
    //   Factor -> S19
    pub fn __state8<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state1(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state5(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state6(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state2(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state19(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 9
    //   Factor = Factor "*" (*) Term [EOF]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S6)
    //   Num -> Shift(S1)
    //
    //   Term -> S20
    pub fn __state9<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state6(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state1(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state20(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   Factor = Factor "/" (*) Term [EOF]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   Num -> Shift(S1)
    //   "(" -> Shift(S6)
    //
    //   Term -> S21
    pub fn __state10<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state1(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state6(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state21(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 11
    //   (<Expr> ",")* = (*) ["("]
    //   (<Expr> ",")* = (*) [")"]
    //   (<Expr> ",")* = (*) ["*"]
    //   (<Expr> ",")* = (*) [Num]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["("]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [")"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [Num]
    //   Comma<Expr> = (*) (<Expr> ",")* Expr? [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [EOF]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["*"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["+"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["-"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["/"]
    //
    //   ")" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   Num -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "(" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "*" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //
    //   Comma<Expr> -> S23
    //   (<Expr> ",")* -> S22
    pub fn __state11<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::LParen(..), _)) |
            Some((_, Tok::Times(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
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
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 12
    //   Factor = "*" (*) "(" Comma<Expr> ")" [")"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["*"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["+"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["-"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["/"]
    //
    //   "(" -> Shift(S24)
    //
    pub fn __state12<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state24(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
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

    // State 13
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   ")" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //
    pub fn __state13<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
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

    // State 14
    //   Term = Num (*) [")"]
    //   Term = Num (*) ["*"]
    //   Term = Num (*) ["+"]
    //   Term = Num (*) ["-"]
    //   Term = Num (*) ["/"]
    //
    //   "/" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   ")" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = Num => Call(ActionFn(8));)
    //
    pub fn __state14<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
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
    //   "-" -> Shift(S25)
    //   "+" -> Shift(S26)
    //   ")" -> Shift(S27)
    //
    pub fn __state15<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state25(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state26(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state27(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
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

    // State 16
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
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   ")" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "*" -> Shift(S29)
    //   "/" -> Shift(S28)
    //
    pub fn __state16<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state29(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state28(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
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

    // State 17
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
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
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
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "*" -> Shift(S12)
    //   Num -> Shift(S14)
    //   "(" -> Shift(S17)
    //
    //   Term -> S13
    //   Factor -> S16
    //   Expr -> S30
    pub fn __state17<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state12(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state17(arena, __lookbehind, __lookahead, __tokens, __sym1));
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
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state16(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state30(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
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
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   EOF -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "*" -> Shift(S9)
    //   "/" -> Shift(S10)
    //
    pub fn __state18<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state9(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state10(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
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
    //   EOF -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "*" -> Shift(S9)
    //   "/" -> Shift(S10)
    //
    pub fn __state19<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state9(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state10(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
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

    // State 20
    //   Factor = Factor "*" Term (*) [EOF]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   EOF -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state20<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            None |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
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

    // State 21
    //   Factor = Factor "/" Term (*) [EOF]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   EOF -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state21<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            None |
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
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

    // State 22
    //   (<Expr> ",") = (*) Expr "," ["("]
    //   (<Expr> ",") = (*) Expr "," [")"]
    //   (<Expr> ",") = (*) Expr "," ["*"]
    //   (<Expr> ",") = (*) Expr "," [Num]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") ["("]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") [")"]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") [Num]
    //   Comma<Expr> = (<Expr> ",")* (*) Expr? [")"]
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor [","]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor [","]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor [","]
    //   Expr = (*) Factor ["-"]
    //   Expr? = (*) [")"]
    //   Expr? = (*) Expr [")"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term [","]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term [","]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term [","]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [","]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num [","]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S34)
    //   "*" -> Shift(S35)
    //   ")" -> Reduce(Expr? =  => Call(ActionFn(12));)
    //   Num -> Shift(S33)
    //
    //   Expr -> S36
    //   Term -> S37
    //   Factor -> S32
    //   (<Expr> ",") -> S31
    //   Expr? -> S38
    pub fn __state22<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state34(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state35(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state33(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __nt = super::__action12(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::Expr_3f(__nt));
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
                    __result = try!(__state36(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state37(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state32(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state31(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Expr_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state38(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 23
    //   Factor = "*" "(" Comma<Expr> (*) ")" [EOF]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["*"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["+"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["-"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["/"]
    //
    //   ")" -> Shift(S39)
    //
    pub fn __state23<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state39(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2, __sym3));
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
    //   (<Expr> ",")* = (*) ["("]
    //   (<Expr> ",")* = (*) [")"]
    //   (<Expr> ",")* = (*) ["*"]
    //   (<Expr> ",")* = (*) [Num]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["("]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [")"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [Num]
    //   Comma<Expr> = (*) (<Expr> ",")* Expr? [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["*"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["+"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["-"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["/"]
    //
    //   ")" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "(" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   Num -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "*" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //
    //   (<Expr> ",")* -> S22
    //   Comma<Expr> -> S40
    pub fn __state24<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::LParen(..), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::Times(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
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
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state40(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
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
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S17)
    //   Num -> Shift(S14)
    //   "*" -> Shift(S12)
    //
    //   Term -> S13
    //   Factor -> S41
    pub fn __state25<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state17(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state12(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state13(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state41(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

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
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   Num -> Shift(S14)
    //   "*" -> Shift(S12)
    //   "(" -> Shift(S17)
    //
    //   Factor -> S42
    //   Term -> S13
    pub fn __state26<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state12(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state17(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state42(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 27
    //   Term = "(" Expr ")" (*) [EOF]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   EOF -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //
    pub fn __state27<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            None |
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
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

    // State 28
    //   Factor = Factor "/" (*) Term [")"]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   Num -> Shift(S14)
    //   "(" -> Shift(S17)
    //
    //   Term -> S43
    pub fn __state28<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state17(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state43(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 29
    //   Factor = Factor "*" (*) Term [")"]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S17)
    //   Num -> Shift(S14)
    //
    //   Term -> S44
    pub fn __state29<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state17(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state44(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 30
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
    //   ")" -> Shift(S45)
    //   "-" -> Shift(S25)
    //   "+" -> Shift(S26)
    //
    pub fn __state30<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state45(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state25(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state26(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
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
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) ["("]
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) [")"]
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) ["*"]
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) [Num]
    //
    //   "*" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => Call(ActionFn(14));)
    //   ")" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => Call(ActionFn(14));)
    //   Num -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => Call(ActionFn(14));)
    //   "(" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => Call(ActionFn(14));)
    //
    pub fn __state31<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::LParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 32
    //   Expr = Factor (*) [")"]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) [","]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term [","]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term [","]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "," -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "/" -> Shift(S47)
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "*" -> Shift(S46)
    //   ")" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //
    pub fn __state32<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state47(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state46(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::Comma(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
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

    // State 33
    //   Term = Num (*) [")"]
    //   Term = Num (*) ["*"]
    //   Term = Num (*) ["+"]
    //   Term = Num (*) [","]
    //   Term = Num (*) ["-"]
    //   Term = Num (*) ["/"]
    //
    //   ")" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "," -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = Num => Call(ActionFn(8));)
    //
    pub fn __state33<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Comma(..), _)) |
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
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

    // State 34
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
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [")"]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" [","]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S17)
    //   "*" -> Shift(S12)
    //   Num -> Shift(S14)
    //
    //   Term -> S13
    //   Factor -> S16
    //   Expr -> S48
    pub fn __state34<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state17(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state12(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym1));
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
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state16(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state48(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 35
    //   Factor = "*" (*) "(" Comma<Expr> ")" [")"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["*"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["+"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" [","]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["-"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["/"]
    //
    //   "(" -> Shift(S49)
    //
    pub fn __state35<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state49(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
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

    // State 36
    //   (<Expr> ",") = Expr (*) "," ["("]
    //   (<Expr> ",") = Expr (*) "," [")"]
    //   (<Expr> ",") = Expr (*) "," ["*"]
    //   (<Expr> ",") = Expr (*) "," [Num]
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor [","]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor [","]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Expr? = Expr (*) [")"]
    //
    //   "-" -> Shift(S51)
    //   "+" -> Shift(S52)
    //   ")" -> Reduce(Expr? = Expr => Call(ActionFn(11));)
    //   "," -> Shift(S50)
    //
    pub fn __state36<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state51(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state52(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Comma(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state50(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr_3f(__nt)));
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

    // State 37
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) [","]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "," -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   ")" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //
    pub fn __state37<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Comma(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
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

    // State 38
    //   Comma<Expr> = (<Expr> ",")* Expr? (*) [")"]
    //
    //   ")" -> Reduce(Comma<Expr> = (<Expr> ",")*, Expr? => Call(ActionFn(10));)
    //
    pub fn __state38<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
        __sym1: &mut Option<::std::option::Option<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action10(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Comma_3cExpr_3e(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 39
    //   Factor = "*" "(" Comma<Expr> ")" (*) [EOF]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["*"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["+"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["-"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["/"]
    //
    //   EOF -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "+" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "/" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "-" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //
    pub fn __state39<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            None |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
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

    // State 40
    //   Factor = "*" "(" Comma<Expr> (*) ")" [")"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["*"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["+"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["-"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["/"]
    //
    //   ")" -> Shift(S53)
    //
    pub fn __state40<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state53(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2, __sym3));
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

    // State 41
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
    //   "*" -> Shift(S29)
    //   ")" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "/" -> Shift(S28)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //
    pub fn __state41<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state29(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state28(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
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

    // State 42
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
    //   "*" -> Shift(S29)
    //   ")" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "/" -> Shift(S28)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //
    pub fn __state42<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state29(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state28(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
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

    // State 43
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   ")" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state43<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
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

    // State 44
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   ")" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state44<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
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

    // State 45
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   ")" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //
    pub fn __state45<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
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

    // State 46
    //   Factor = Factor "*" (*) Term [")"]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term [","]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num [","]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S34)
    //   Num -> Shift(S33)
    //
    //   Term -> S54
    pub fn __state46<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state34(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state33(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state54(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 47
    //   Factor = Factor "/" (*) Term [")"]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term [","]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num [","]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S34)
    //   Num -> Shift(S33)
    //
    //   Term -> S55
    pub fn __state47<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state34(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state33(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state55(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 48
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [")"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" [","]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   "-" -> Shift(S25)
    //   "+" -> Shift(S26)
    //   ")" -> Shift(S56)
    //
    pub fn __state48<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state25(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state26(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state56(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
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

    // State 49
    //   (<Expr> ",")* = (*) ["("]
    //   (<Expr> ",")* = (*) [")"]
    //   (<Expr> ",")* = (*) ["*"]
    //   (<Expr> ",")* = (*) [Num]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["("]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [")"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [Num]
    //   Comma<Expr> = (*) (<Expr> ",")* Expr? [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["*"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["+"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [","]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["-"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["/"]
    //
    //   "(" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "*" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   Num -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   ")" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //
    //   (<Expr> ",")* -> S22
    //   Comma<Expr> -> S57
    pub fn __state49<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::LParen(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::RParen(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
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
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state57(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 50
    //   (<Expr> ",") = Expr "," (*) ["("]
    //   (<Expr> ",") = Expr "," (*) [")"]
    //   (<Expr> ",") = Expr "," (*) ["*"]
    //   (<Expr> ",") = Expr "," (*) [Num]
    //
    //   "*" -> Reduce((<Expr> ",") = Expr, "," => Call(ActionFn(15));)
    //   "(" -> Reduce((<Expr> ",") = Expr, "," => Call(ActionFn(15));)
    //   ")" -> Reduce((<Expr> ",") = Expr, "," => Call(ActionFn(15));)
    //   Num -> Reduce((<Expr> ",") = Expr, "," => Call(ActionFn(15));)
    //
    pub fn __state50<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::LParen(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action15(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 51
    //   Expr = Expr "-" (*) Factor [")"]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor [","]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term [","]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term [","]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term [","]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [","]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num [","]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   Num -> Shift(S33)
    //   "*" -> Shift(S35)
    //   "(" -> Shift(S34)
    //
    //   Factor -> S58
    //   Term -> S37
    pub fn __state51<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state33(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state35(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state34(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state58(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state37(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 52
    //   Expr = Expr "+" (*) Factor [")"]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor [","]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term [","]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term [","]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term [","]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [","]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num [","]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S34)
    //   "*" -> Shift(S35)
    //   Num -> Shift(S33)
    //
    //   Factor -> S59
    //   Term -> S37
    pub fn __state52<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state34(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state35(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state33(arena, __lookbehind, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state59(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state37(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 53
    //   Factor = "*" "(" Comma<Expr> ")" (*) [")"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["*"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["+"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["-"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["/"]
    //
    //   "*" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   ")" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "+" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "-" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "/" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //
    pub fn __state53<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
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

    // State 54
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) [","]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "," -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state54<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Comma(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
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

    // State 55
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) [","]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "," -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   ")" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state55<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Comma(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
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

    // State 56
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) [","]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "," -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   ")" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //
    pub fn __state56<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Comma(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
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

    // State 57
    //   Factor = "*" "(" Comma<Expr> (*) ")" [")"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["*"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["+"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" [","]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["-"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["/"]
    //
    //   ")" -> Shift(S60)
    //
    pub fn __state57<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state60(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2, __sym3));
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

    // State 58
    //   Expr = Expr "-" Factor (*) [")"]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) [","]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term [","]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term [","]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "," -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   ")" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "/" -> Shift(S47)
    //   "*" -> Shift(S46)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //
    pub fn __state58<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state47(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state46(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::Comma(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
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

    // State 59
    //   Expr = Expr "+" Factor (*) [")"]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) [","]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term [","]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term [","]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   ")" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "," -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "/" -> Shift(S47)
    //   "*" -> Shift(S46)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //
    pub fn __state59<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state47(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
                __result = try!(__state46(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Comma(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
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

    // State 60
    //   Factor = "*" "(" Comma<Expr> ")" (*) [")"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["*"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["+"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) [","]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["-"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["/"]
    //
    //   "/" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   ")" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "-" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "," -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "+" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //
    pub fn __state60<
        'ast,
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize, Tok, usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<usize>, Option<(usize, Tok, usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Comma(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
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

pub fn __action0<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
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
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Add, l, r))
}

pub fn __action3<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
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
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Reduce(Op::Mul, __0))
}

pub fn __action7<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action8<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    n: i32,
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
) -> Vec<&'ast Node<'ast>>
{
    h.into_iter().chain(t).collect()
}

pub fn __action11<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
) -> ::std::option::Option<&'ast Node<'ast>>
{
    Some(__0)
}

pub fn __action12<
    'ast,
>(
    arena: &'ast Arena<'ast>,
) -> ::std::option::Option<&'ast Node<'ast>>
{
    None
}

pub fn __action13<
    'ast,
>(
    arena: &'ast Arena<'ast>,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    vec![]
}

pub fn __action14<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    v: ::std::vec::Vec<&'ast Node<'ast>>,
    e: &'ast Node<'ast>,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action15<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
    _: Tok,
) -> &'ast Node<'ast>
{
    (__0)
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
