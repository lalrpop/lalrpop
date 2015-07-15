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
        (__lookahead, __parse__Expr::__Nonterminal::__Expr(__nt)) => Ok((__lookahead, __nt)),
        _ => unreachable!(),
    }
}

mod __parse__Expr {
    #![allow(non_snake_case, unused_mut, unused_variables)]

    use expr_arena_ast::{Arena, Node, Op};
    use util::tok::Tok;

    pub enum __Nonterminal<'ast, > {
        __Expr(&'ast Node<'ast>),
        Factor(&'ast Node<'ast>),
        Expr(&'ast Node<'ast>),
        Term(&'ast Node<'ast>),
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
    //   "Num" -> Shift(S3)
    //   "(" -> Shift(S4)
    //
    //   Expr -> S1
    //   Factor -> S5
    //   Term -> S2
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
            Some(Tok::Num(__tok0)) => {
                let mut __sym0 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state3(arena, __lookahead, __tokens, __sym0));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym0 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state4(arena, __lookahead, __tokens, __sym0));
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
                    __result = try!(__state1(arena, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(arena, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(arena, __lookahead, __tokens, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Expr = Expr (*) "+" Factor [EOF]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [EOF]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   __Expr = Expr (*) [EOF]
    //
    //   "+" -> Shift(S6)
    //   "-" -> Shift(S7)
    //   EOF -> Reduce(__Expr = Expr => ActionFn(0);)
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
            Some(__tok @ Tok::Plus(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state6(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(__tok @ Tok::Minus(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state7(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action0(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::__Expr(__nt)));
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
    //   "/" -> Reduce(Factor = Term => ActionFn(6);)
    //   "-" -> Reduce(Factor = Term => ActionFn(6);)
    //   EOF -> Reduce(Factor = Term => ActionFn(6);)
    //   "+" -> Reduce(Factor = Term => ActionFn(6);)
    //   "*" -> Reduce(Factor = Term => ActionFn(6);)
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
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 3
    //   Term = "Num" (*) [EOF]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "*" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "/" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "-" -> Reduce(Term = "Num" => ActionFn(7);)
    //   EOF -> Reduce(Term = "Num" => ActionFn(7);)
    //   "+" -> Reduce(Term = "Num" => ActionFn(7);)
    //
    pub fn __state3<
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
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 4
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
    //   "(" -> Shift(S8)
    //   "Num" -> Shift(S9)
    //
    //   Expr -> S12
    //   Term -> S11
    //   Factor -> S10
    pub fn __state4<
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
                __result = try!(__state8(arena, __lookahead, __tokens, __sym1));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookahead, __tokens, __sym1));
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
                    __result = try!(__state12(arena, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(arena, __lookahead, __tokens, __sym1));
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

    // State 5
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
    //   EOF -> Reduce(Expr = Factor => ActionFn(3);)
    //   "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "*" -> Shift(S14)
    //   "/" -> Shift(S13)
    //
    pub fn __state5<
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
                __result = try!(__state14(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state13(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Expr(__nt)));
            }
            Some(Tok::Plus(..)) => {
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

    // State 6
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
    //   "Num" -> Shift(S3)
    //   "(" -> Shift(S4)
    //
    //   Factor -> S15
    //   Term -> S2
    pub fn __state6<
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
                __result = try!(__state3(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state4(arena, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state15(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
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

    // State 7
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
    //   "Num" -> Shift(S3)
    //   "(" -> Shift(S4)
    //
    //   Term -> S2
    //   Factor -> S16
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
                __result = try!(__state3(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state4(arena, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state16(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 8
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
    //   "Num" -> Shift(S9)
    //   "(" -> Shift(S8)
    //
    //   Factor -> S10
    //   Term -> S11
    //   Expr -> S17
    pub fn __state8<
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
            Some(Tok::Num(__tok0)) => {
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookahead, __tokens, __sym1));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state8(arena, __lookahead, __tokens, __sym1));
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
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(arena, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(arena, __lookahead, __tokens, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 9
    //   Term = "Num" (*) [")"]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "-" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "+" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "/" -> Reduce(Term = "Num" => ActionFn(7);)
    //   ")" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "*" -> Reduce(Term = "Num" => ActionFn(7);)
    //
    pub fn __state9<
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
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action7(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
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
    //   "/" -> Shift(S19)
    //   "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "*" -> Shift(S18)
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
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state19(arena, __lookahead, __tokens, __sym0, __sym1));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state18(arena, __lookahead, __tokens, __sym0, __sym1));
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
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Term => ActionFn(6);)
    //   "*" -> Reduce(Factor = Term => ActionFn(6);)
    //   "+" -> Reduce(Factor = Term => ActionFn(6);)
    //   ")" -> Reduce(Factor = Term => ActionFn(6);)
    //   "-" -> Reduce(Factor = Term => ActionFn(6);)
    //
    pub fn __state11<
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
                let __nt = super::__actions::__action6(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action6(arena, __sym0);
                return Ok((__lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 12
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
    //   ")" -> Shift(S20)
    //   "+" -> Shift(S21)
    //   "-" -> Shift(S22)
    //
    pub fn __state12<
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
                __result = try!(__state20(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            Some(__tok @ Tok::Plus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state21(arena, __lookahead, __tokens, __sym1, __sym2));
            }
            Some(__tok @ Tok::Minus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state22(arena, __lookahead, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 13
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
    //   "(" -> Shift(S4)
    //   "Num" -> Shift(S3)
    //
    //   Term -> S23
    pub fn __state13<
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
                __result = try!(__state4(arena, __lookahead, __tokens, __sym2));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
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
                    __result = try!(__state23(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 14
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
    //   "Num" -> Shift(S3)
    //   "(" -> Shift(S4)
    //
    //   Term -> S24
    pub fn __state14<
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
                __result = try!(__state3(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state4(arena, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state24(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 15
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
    //   "/" -> Shift(S13)
    //   EOF -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "*" -> Shift(S14)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //
    pub fn __state15<
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
                __result = try!(__state13(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(Tok::Plus(..)) => {
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

    // State 16
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
    //   "/" -> Shift(S13)
    //   "*" -> Shift(S14)
    //   EOF -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //
    pub fn __state16<
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
                __result = try!(__state13(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            None => {
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

    // State 17
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
    //   ")" -> Shift(S25)
    //   "+" -> Shift(S21)
    //   "-" -> Shift(S22)
    //
    pub fn __state17<
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
                __result = try!(__state25(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            Some(__tok @ Tok::Plus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state21(arena, __lookahead, __tokens, __sym1, __sym2));
            }
            Some(__tok @ Tok::Minus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state22(arena, __lookahead, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 18
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
    //   "(" -> Shift(S8)
    //   "Num" -> Shift(S9)
    //
    //   Term -> S26
    pub fn __state18<
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
                __result = try!(__state8(arena, __lookahead, __tokens, __sym2));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state26(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 19
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
    //   "Num" -> Shift(S9)
    //   "(" -> Shift(S8)
    //
    //   Term -> S27
    pub fn __state19<
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
                __result = try!(__state9(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state8(arena, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state27(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 20
    //   Term = "(" Expr ")" (*) [EOF]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "-" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "/" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "*" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "+" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   EOF -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //
    pub fn __state20<
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
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Times(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 21
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
    //   "(" -> Shift(S8)
    //   "Num" -> Shift(S9)
    //
    //   Term -> S11
    //   Factor -> S28
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
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state8(arena, __lookahead, __tokens, __sym2));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state11(arena, __lookahead, __tokens, __sym2));
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

    // State 22
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
    //   "Num" -> Shift(S9)
    //   "(" -> Shift(S8)
    //
    //   Term -> S11
    //   Factor -> S29
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
                __result = try!(__state9(arena, __lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state8(arena, __lookahead, __tokens, __sym2));
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
                    __result = try!(__state11(arena, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(arena, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 23
    //   Factor = Factor "/" Term (*) [EOF]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   EOF -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "-" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "*" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "/" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "+" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //
    pub fn __state23<
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
            None => {
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
            Some(Tok::Times(..)) => {
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
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 24
    //   Factor = Factor "*" Term (*) [EOF]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   EOF -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "*" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "+" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "-" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //
    pub fn __state24<
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
            Some(Tok::Minus(..)) => {
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

    // State 25
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "*" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   ")" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "/" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "+" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //   "-" -> Reduce(Term = "(", Expr, ")" => ActionFn(8);)
    //
    pub fn __state25<
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
                let __nt = super::__actions::__action8(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Div(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Plus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action8(arena, __sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 26
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "*" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   ")" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "-" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "/" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //
    pub fn __state26<
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
            Some(Tok::Plus(..)) => {
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
            Some(Tok::RParen(..)) => {
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

    // State 27
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   ")" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "/" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "-" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "*" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
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
            Some(Tok::Plus(..)) => {
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
            Some(Tok::Minus(..)) => {
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

    // State 28
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
    //   "/" -> Shift(S19)
    //   "*" -> Shift(S18)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   ")" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
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
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state19(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(__tok @ Tok::Times(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state18(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(Tok::Plus(..)) => {
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

    // State 29
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
    //   "*" -> Shift(S18)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   "/" -> Shift(S19)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //   ")" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //
    pub fn __state29<
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
                __result = try!(__state18(arena, __lookahead, __tokens, __sym2, __sym3));
            }
            Some(__tok @ Tok::Div(..)) => {
                let mut __sym3 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state19(arena, __lookahead, __tokens, __sym2, __sym3));
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
        __0: &'ast Node<'ast>,
    ) -> &'ast Node<'ast>
    {
        (__0)
    }

    pub fn __action7<
        'ast,
    >(
        arena: &'ast Arena<'ast>,
        n: i32,
    ) -> &'ast Node<'ast>
    {
        arena.alloc(Node::Value(n))
    }

    pub fn __action8<
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
}
