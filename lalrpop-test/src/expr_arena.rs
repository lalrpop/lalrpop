#![allow(unused_imports)]
use expr_arena_ast::{Arena, Node, Op};
use util::tok::Tok;

#[allow(non_snake_case)]
pub fn parse_Expr<
    'ast,
    __TOKENS: IntoIterator<Item=Tok>,
>(
    arena: &'ast Arena<'ast>,
    __tokens: __TOKENS,
) -> Result<(Option<Tok>, &'ast Node<'ast>), Option<Tok>>
{
    let mut __tokens = __tokens.into_iter();
    let __lookahead = __tokens.next();
    match try!(__parse__Expr::__state0(arena, __lookahead, &mut __tokens)) {
        (__lookahead, __parse__Expr::__Nonterminal::____Expr(__nt)) => Ok((__lookahead, __nt)),
        _ => unreachable!(),
    }
}

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use expr_arena_ast::{Arena, Node, Op};
    use util::tok::Tok;

    pub enum __Nonterminal<'ast, > {
        _28_3cExpr_3e_20_22_2c_22_29(&'ast Node<'ast>),
        Comma_3cExpr_3e(Vec<&'ast Node<'ast>>),
        _28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'ast Node<'ast>>),
        Expr(&'ast Node<'ast>),
        Factor(&'ast Node<'ast>),
        Term(&'ast Node<'ast>),
        ____Expr(&'ast Node<'ast>),
        Expr_3f(::std::option::Option<&'ast Node<'ast>>),
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
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //   __Expr = (*) Expr [EOF]
    //
    //   "*" -> Shift(S5)
    //   "(" -> Shift(S3)
    //   "Num" -> Shift(S6)
    //
    //   Expr -> S4
    //   Term -> S2
    //   Factor -> S1
    pub fn __state0<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym0 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state5(arena, __lookahead, __tokens, __sym0));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym0 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state3(arena, __lookahead, __tokens, __sym0));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym0 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state6(arena, __lookahead, __tokens, __sym0));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(arena, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(arena, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(arena, __lookahead, __tokens, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 1
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
    //   "/" -> Shift(S8)
    //   EOF -> Reduce(Expr = Factor => ActionFn(3);)
    //   "*" -> Shift(S7)
    //   "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //
    pub fn __state1<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state8(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state7(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 2
    //   Factor = Term (*) [EOF]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   EOF -> Reduce(Factor = Term => ActionFn(7);)
    //   "/" -> Reduce(Factor = Term => ActionFn(7);)
    //   "-" -> Reduce(Factor = Term => ActionFn(7);)
    //   "+" -> Reduce(Factor = Term => ActionFn(7);)
    //   "*" -> Reduce(Factor = Term => ActionFn(7);)
    //
    pub fn __state2<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 3
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "*" -> Shift(S9)
    //   "Num" -> Shift(S14)
    //   "(" -> Shift(S13)
    //
    //   Factor -> S10
    //   Expr -> S11
    //   Term -> S12
    pub fn __state3<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookahead, __tokens, __sym1));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookahead, __tokens, __sym1));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state13(arena, __lookahead, __tokens, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(arena, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(arena, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookahead, __tokens, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   Expr = Expr (*) "+" Factor [EOF]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [EOF]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   __Expr = Expr (*) [EOF]
    //
    //   "-" -> Shift(S16)
    //   EOF -> Reduce(__Expr = Expr => ActionFn(0);)
    //   "+" -> Shift(S15)
    //
    pub fn __state4<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Minus(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state16(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(__tok @ Tok::Plus(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state15(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action0(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::____Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
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
    //   "(" -> Shift(S17)
    //
    pub fn __state5<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state17(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 6
    //   Term = "Num" (*) [EOF]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "+" -> Reduce(Term = "Num" => ActionFn(8);)
    //   "-" -> Reduce(Term = "Num" => ActionFn(8);)
    //   "/" -> Reduce(Term = "Num" => ActionFn(8);)
    //   EOF -> Reduce(Term = "Num" => ActionFn(8);)
    //   "*" -> Reduce(Term = "Num" => ActionFn(8);)
    //
    pub fn __state6<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 7
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
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S6)
    //   "(" -> Shift(S3)
    //
    //   Term -> S18
    pub fn __state7<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state6(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state3(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
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
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S3)
    //   "Num" -> Shift(S6)
    //
    //   Term -> S19
    pub fn __state8<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state3(arena, __lookahead, __tokens, __sym2));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state6(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state19(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 9
    //   Factor = "*" (*) "(" Comma<Expr> ")" [")"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["*"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["+"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["-"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["/"]
    //
    //   "(" -> Shift(S20)
    //
    pub fn __state9<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state20(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 10
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
    //   "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "*" -> Shift(S22)
    //   "/" -> Shift(S21)
    //   ")" -> Reduce(Expr = Factor => ActionFn(3);)
    //
    pub fn __state10<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state22(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state21(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 11
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
    //   "-" -> Shift(S23)
    //   ")" -> Shift(S24)
    //   "+" -> Shift(S25)
    //
    pub fn __state11<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Minus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state23(arena, __lookahead, __tokens, __sym1, __sym2));
            }
            Some(__tok @ Tok::RParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state24(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            Some(__tok @ Tok::Plus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state25(arena, __lookahead, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 12
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "*" -> Reduce(Factor = Term => ActionFn(7);)
    //   ")" -> Reduce(Factor = Term => ActionFn(7);)
    //   "-" -> Reduce(Factor = Term => ActionFn(7);)
    //   "+" -> Reduce(Factor = Term => ActionFn(7);)
    //   "/" -> Reduce(Factor = Term => ActionFn(7);)
    //
    pub fn __state12<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 13
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S13)
    //   "*" -> Shift(S9)
    //   "Num" -> Shift(S14)
    //
    //   Expr -> S26
    //   Term -> S12
    //   Factor -> S10
    pub fn __state13<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state13(arena, __lookahead, __tokens, __sym1));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookahead, __tokens, __sym1));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookahead, __tokens, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state26(arena, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(arena, __lookahead, __tokens, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 14
    //   Term = "Num" (*) [")"]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "+" -> Reduce(Term = "Num" => ActionFn(8);)
    //   "*" -> Reduce(Term = "Num" => ActionFn(8);)
    //   "-" -> Reduce(Term = "Num" => ActionFn(8);)
    //   "/" -> Reduce(Term = "Num" => ActionFn(8);)
    //   ")" -> Reduce(Term = "Num" => ActionFn(8);)
    //
    pub fn __state14<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 15
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
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S6)
    //   "*" -> Shift(S5)
    //   "(" -> Shift(S3)
    //
    //   Factor -> S27
    //   Term -> S2
    pub fn __state15<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state6(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state5(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state3(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state27(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(arena, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 16
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
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S3)
    //   "*" -> Shift(S5)
    //   "Num" -> Shift(S6)
    //
    //   Term -> S2
    //   Factor -> S28
    pub fn __state16<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state3(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state5(arena, __lookahead, __tokens, __sym2));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state6(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(arena, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 17
    //   (<Expr> ",")* = (*) ["("]
    //   (<Expr> ",")* = (*) [")"]
    //   (<Expr> ",")* = (*) ["*"]
    //   (<Expr> ",")* = (*) ["Num"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["("]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [")"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["Num"]
    //   Comma<Expr> = (*) (<Expr> ",")* Expr? [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [EOF]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["*"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["+"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["-"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["/"]
    //
    //   "(" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //   "*" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //   ")" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //   "Num" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //
    //   (<Expr> ",")* -> S29
    //   Comma<Expr> -> S30
    pub fn __state17<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::LParen(..)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some(Tok::Times(..)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some(Tok::RParen(..)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some(Tok::Num(_)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(arena, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state30(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   Factor = Factor "*" Term (*) [EOF]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "-" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "*" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   EOF -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "/" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "+" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //
    pub fn __state18<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 19
    //   Factor = Factor "/" Term (*) [EOF]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "-" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   EOF -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "*" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "+" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //
    pub fn __state19<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 20
    //   (<Expr> ",")* = (*) ["("]
    //   (<Expr> ",")* = (*) [")"]
    //   (<Expr> ",")* = (*) ["*"]
    //   (<Expr> ",")* = (*) ["Num"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["("]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [")"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["Num"]
    //   Comma<Expr> = (*) (<Expr> ",")* Expr? [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["*"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["+"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["-"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["/"]
    //
    //   "Num" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //   ")" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //   "(" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //   "*" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //
    //   (<Expr> ",")* -> S29
    //   Comma<Expr> -> S31
    pub fn __state20<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Num(_)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some(Tok::RParen(..)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some(Tok::LParen(..)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some(Tok::Times(..)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(arena, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 21
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S14)
    //   "(" -> Shift(S13)
    //
    //   Term -> S32
    pub fn __state21<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state13(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state32(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 22
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S14)
    //   "(" -> Shift(S13)
    //
    //   Term -> S33
    pub fn __state22<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state13(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state33(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 23
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S13)
    //   "*" -> Shift(S9)
    //   "Num" -> Shift(S14)
    //
    //   Factor -> S34
    //   Term -> S12
    pub fn __state23<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state13(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookahead, __tokens, __sym2));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state34(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
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
    //   "*" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   EOF -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "-" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "/" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "+" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //
    pub fn __state24<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 25
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "*" -> Shift(S9)
    //   "Num" -> Shift(S14)
    //   "(" -> Shift(S13)
    //
    //   Factor -> S35
    //   Term -> S12
    pub fn __state25<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookahead, __tokens, __sym2));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state13(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state35(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 26
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
    //   "-" -> Shift(S23)
    //   "+" -> Shift(S25)
    //   ")" -> Shift(S36)
    //
    pub fn __state26<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Minus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state23(arena, __lookahead, __tokens, __sym1, __sym2));
            }
            Some(__tok @ Tok::Plus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state25(arena, __lookahead, __tokens, __sym1, __sym2));
            }
            Some(__tok @ Tok::RParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state36(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 27
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
    //   "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "*" -> Shift(S7)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   EOF -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "/" -> Shift(S8)
    //
    pub fn __state27<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state7(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state8(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 28
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
    //   "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   EOF -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "*" -> Shift(S7)
    //   "/" -> Shift(S8)
    //
    pub fn __state28<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state7(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state8(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 29
    //   (<Expr> ",") = (*) Expr "," ["("]
    //   (<Expr> ",") = (*) Expr "," [")"]
    //   (<Expr> ",") = (*) Expr "," ["*"]
    //   (<Expr> ",") = (*) Expr "," ["Num"]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") ["("]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") [")"]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") ["Num"]
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" [","]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S44)
    //   "*" -> Shift(S41)
    //   "(" -> Shift(S37)
    //   ")" -> Reduce(Expr? =  => ActionFn(12);)
    //
    //   Term -> S39
    //   (<Expr> ",") -> S40
    //   Expr? -> S42
    //   Factor -> S38
    //   Expr -> S43
    pub fn __state29<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Num(__tok0)) => {
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state44(arena, __lookahead, __tokens, __sym1));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state41(arena, __lookahead, __tokens, __sym1));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state37(arena, __lookahead, __tokens, __sym1));
            }
            Some(Tok::RParen(..)) => {
                let __nt = super::__actions::__action12(arena, );
                __result = (__lookahead, __Nonterminal::Expr_3f(__nt));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state39(arena, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state40(arena, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Expr_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state42(arena, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state38(arena, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state43(arena, __lookahead, __tokens, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 30
    //   Factor = "*" "(" Comma<Expr> (*) ")" [EOF]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["*"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["+"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["-"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["/"]
    //
    //   ")" -> Shift(S45)
    //
    pub fn __state30<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::RParen(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state45(arena, __lookahead, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 31
    //   Factor = "*" "(" Comma<Expr> (*) ")" [")"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["*"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["+"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["-"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["/"]
    //
    //   ")" -> Shift(S46)
    //
    pub fn __state31<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::RParen(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state46(arena, __lookahead, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 32
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "-" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   ")" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "/" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "+" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "*" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //
    pub fn __state32<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 33
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "-" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "*" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "+" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   ")" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "/" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //
    pub fn __state33<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 34
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
    //   "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "*" -> Shift(S22)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   ")" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "/" -> Shift(S21)
    //
    pub fn __state34<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state22(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state21(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 35
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
    //   "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "*" -> Shift(S22)
    //   ")" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //
    pub fn __state35<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state21(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state22(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 36
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   ")" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "/" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "*" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "+" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "-" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //
    pub fn __state36<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 37
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "*" -> Shift(S9)
    //   "(" -> Shift(S13)
    //   "Num" -> Shift(S14)
    //
    //   Term -> S12
    //   Expr -> S47
    //   Factor -> S10
    pub fn __state37<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookahead, __tokens, __sym1));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state13(arena, __lookahead, __tokens, __sym1));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookahead, __tokens, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state47(arena, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(arena, __lookahead, __tokens, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 38
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
    //   "/" -> Shift(S48)
    //   "," -> Reduce(Expr = Factor => ActionFn(3);)
    //   "*" -> Shift(S49)
    //   "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //   ")" -> Reduce(Expr = Factor => ActionFn(3);)
    //
    pub fn __state38<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state48(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state49(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(Tok::Comma(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 39
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) [","]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Term => ActionFn(7);)
    //   "," -> Reduce(Factor = Term => ActionFn(7);)
    //   "-" -> Reduce(Factor = Term => ActionFn(7);)
    //   "+" -> Reduce(Factor = Term => ActionFn(7);)
    //   ")" -> Reduce(Factor = Term => ActionFn(7);)
    //   "*" -> Reduce(Factor = Term => ActionFn(7);)
    //
    pub fn __state39<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Comma(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 40
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) ["("]
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) [")"]
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) ["*"]
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) ["Num"]
    //
    //   "(" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => ActionFn(14);)
    //   "*" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => ActionFn(14);)
    //   ")" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => ActionFn(14);)
    //   "Num" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => ActionFn(14);)
    //
    pub fn __state40<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::LParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__actions::__action14(arena, __sym0, __sym1);
                return Ok((__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__actions::__action14(arena, __sym0, __sym1);
                return Ok((__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__actions::__action14(arena, __sym0, __sym1);
                return Ok((__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt)));
            }
            Some(Tok::Num(_)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__actions::__action14(arena, __sym0, __sym1);
                return Ok((__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 41
    //   Factor = "*" (*) "(" Comma<Expr> ")" [")"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["*"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["+"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" [","]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["-"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["/"]
    //
    //   "(" -> Shift(S50)
    //
    pub fn __state41<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state50(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 42
    //   Comma<Expr> = (<Expr> ",")* Expr? (*) [")"]
    //
    //   ")" -> Reduce(Comma<Expr> = (<Expr> ",")*, Expr? => ActionFn(10);)
    //
    pub fn __state42<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
        __sym1: &mut Option<::std::option::Option<&'ast Node<'ast>>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__actions::__action10(arena, __sym0, __sym1);
                return Ok((__lookahead, __Nonterminal::Comma_3cExpr_3e(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 43
    //   (<Expr> ",") = Expr (*) "," ["("]
    //   (<Expr> ",") = Expr (*) "," [")"]
    //   (<Expr> ",") = Expr (*) "," ["*"]
    //   (<Expr> ",") = Expr (*) "," ["Num"]
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
    //   "-" -> Shift(S52)
    //   "," -> Shift(S53)
    //   ")" -> Reduce(Expr? = Expr => ActionFn(11);)
    //   "+" -> Shift(S51)
    //
    pub fn __state43<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Minus(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state52(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(__tok @ Tok::Comma(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state53(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(__tok @ Tok::Plus(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state51(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action11(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr_3f(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 44
    //   Term = "Num" (*) [")"]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) [","]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "," -> Reduce(Term = "Num" => ActionFn(8);)
    //   "-" -> Reduce(Term = "Num" => ActionFn(8);)
    //   "*" -> Reduce(Term = "Num" => ActionFn(8);)
    //   ")" -> Reduce(Term = "Num" => ActionFn(8);)
    //   "+" -> Reduce(Term = "Num" => ActionFn(8);)
    //   "/" -> Reduce(Term = "Num" => ActionFn(8);)
    //
    pub fn __state44<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Comma(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 45
    //   Factor = "*" "(" Comma<Expr> ")" (*) [EOF]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["*"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["+"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["-"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["/"]
    //
    //   "*" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   "/" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   "-" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   EOF -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   "+" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //
    pub fn __state45<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 46
    //   Factor = "*" "(" Comma<Expr> ")" (*) [")"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["*"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["+"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["-"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["/"]
    //
    //   "-" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   "*" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   "+" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   ")" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   "/" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //
    pub fn __state46<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 47
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
    //   ")" -> Shift(S54)
    //   "-" -> Shift(S23)
    //   "+" -> Shift(S25)
    //
    pub fn __state47<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::RParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state54(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            Some(__tok @ Tok::Minus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state23(arena, __lookahead, __tokens, __sym1, __sym2));
            }
            Some(__tok @ Tok::Plus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state25(arena, __lookahead, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 48
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" [","]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S44)
    //   "(" -> Shift(S37)
    //
    //   Term -> S55
    pub fn __state48<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state44(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state37(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state55(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 49
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" [","]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S37)
    //   "Num" -> Shift(S44)
    //
    //   Term -> S56
    pub fn __state49<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state37(arena, __lookahead, __tokens, __sym2));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state44(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state56(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 50
    //   (<Expr> ",")* = (*) ["("]
    //   (<Expr> ",")* = (*) [")"]
    //   (<Expr> ",")* = (*) ["*"]
    //   (<Expr> ",")* = (*) ["Num"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["("]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [")"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["Num"]
    //   Comma<Expr> = (*) (<Expr> ",")* Expr? [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["*"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["+"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [","]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["-"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["/"]
    //
    //   "(" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //   "Num" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //   "*" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //   ")" -> Reduce((<Expr> ",")* =  => ActionFn(13);)
    //
    //   Comma<Expr> -> S57
    //   (<Expr> ",")* -> S29
    pub fn __state50<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::LParen(..)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some(Tok::Num(_)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some(Tok::Times(..)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some(Tok::RParen(..)) => {
                let __nt = super::__actions::__action13(arena, );
                __result = (__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state57(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(arena, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 51
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" [","]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S44)
    //   "(" -> Shift(S37)
    //   "*" -> Shift(S41)
    //
    //   Term -> S39
    //   Factor -> S58
    pub fn __state51<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state44(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state37(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state41(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state39(arena, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state58(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 52
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
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" [","]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S44)
    //   "*" -> Shift(S41)
    //   "(" -> Shift(S37)
    //
    //   Factor -> S59
    //   Term -> S39
    pub fn __state52<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state44(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state41(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state37(arena, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state59(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state39(arena, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 53
    //   (<Expr> ",") = Expr "," (*) ["("]
    //   (<Expr> ",") = Expr "," (*) [")"]
    //   (<Expr> ",") = Expr "," (*) ["*"]
    //   (<Expr> ",") = Expr "," (*) ["Num"]
    //
    //   ")" -> Reduce((<Expr> ",") = Expr, "," => ActionFn(15);)
    //   "Num" -> Reduce((<Expr> ",") = Expr, "," => ActionFn(15);)
    //   "*" -> Reduce((<Expr> ",") = Expr, "," => ActionFn(15);)
    //   "(" -> Reduce((<Expr> ",") = Expr, "," => ActionFn(15);)
    //
    pub fn __state53<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__actions::__action15(arena, __sym0, __sym1);
                return Ok((__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt)));
            }
            Some(Tok::Num(_)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__actions::__action15(arena, __sym0, __sym1);
                return Ok((__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__actions::__action15(arena, __sym0, __sym1);
                return Ok((__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt)));
            }
            Some(Tok::LParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__actions::__action15(arena, __sym0, __sym1);
                return Ok((__lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 54
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) [","]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   ")" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "/" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "+" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "," -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "-" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //   "*" -> Reduce(Term = "(", Expr, ")" => ActionFn(9);)
    //
    pub fn __state54<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Comma(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
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
    //   "," -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "+" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "-" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "/" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "*" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   ")" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //
    pub fn __state55<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Comma(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 56
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) [","]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "*" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "-" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "," -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "/" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "+" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   ")" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //
    pub fn __state56<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Comma(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
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
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::RParen(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state60(arena, __lookahead, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 58
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
    //   "," -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   ")" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "/" -> Shift(S48)
    //   "*" -> Shift(S49)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //
    pub fn __state58<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state48(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state49(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(Tok::Comma(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 59
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
    //   ")" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "*" -> Shift(S49)
    //   "/" -> Shift(S48)
    //   "," -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //
    pub fn __state59<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state49(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state48(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Comma(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
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
    //   "*" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   "+" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   "/" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   ")" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   "-" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //   "," -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);)
    //
    pub fn __state60<
        'ast,
        __TOKENS: Iterator<Item=Tok>,
    >(
        arena: &'ast Arena<'ast>,
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<'ast, >), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<'ast, >);
        match __lookahead {
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Comma(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }
}

mod __actions {
    #![allow(unused_variables)]
    use expr_arena_ast::{Arena, Node, Op};
    use util::tok::Tok;


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
}
