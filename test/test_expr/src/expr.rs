use util::tok::Tok;

#[allow(non_snake_case)]
pub fn parse_Expr<TOKENS: IntoIterator<Item=Tok>>(
    tokens: TOKENS)
    -> Result<(Option<Tok>, i32), Option<Tok>>
{
    let mut tokens = tokens.into_iter();
    let lookahead = tokens.next();
    match try!(__parseExpr::__state0(lookahead, &mut tokens)) {
        (lookahead, __parseExpr::__Nonterminal::Expr(nt)) => Ok((lookahead, nt)),
        _ => unreachable!(),
    }
}

mod __parseExpr {
    #![allow(non_snake_case, unused_mut, unused_variables)]

    use util::tok::Tok;

    pub enum __Nonterminal {
        Term(i32),
        Expr(i32),
        Factor(i32),
        Expr1(i32),
    }

    // State 0
    //   Expr = (*) Expr1 [EOF]
    //   Expr1 = (*) Expr1 "+" Factor [EOF]
    //   Expr1 = (*) Expr1 "+" Factor ["+"]
    //   Expr1 = (*) Expr1 "+" Factor ["-"]
    //   Expr1 = (*) Expr1 "-" Factor [EOF]
    //   Expr1 = (*) Expr1 "-" Factor ["+"]
    //   Expr1 = (*) Expr1 "-" Factor ["-"]
    //   Expr1 = (*) Factor [EOF]
    //   Expr1 = (*) Factor ["+"]
    //   Expr1 = (*) Factor ["-"]
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
    //   Term = (*) "(" Expr1 ")" [EOF]
    //   Term = (*) "(" Expr1 ")" ["*"]
    //   Term = (*) "(" Expr1 ")" ["+"]
    //   Term = (*) "(" Expr1 ")" ["-"]
    //   Term = (*) "(" Expr1 ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S2)
    //   "(" -> Shift(S5)
    //
    //   Factor -> S3
    //   Term -> S4
    //   Expr1 -> S1
    pub fn __state0<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Num(..)) => {
                let sym0 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state2(lookahead, tokens, sym0));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym0 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym0));
            }
            _ => {
                return Err(lookahead);
            }
        }
        loop {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Factor(nt) => {
                    let sym0 = &mut Some(nt);
                    result = try!(__state3(lookahead, tokens, sym0));
                }
                __Nonterminal::Term(nt) => {
                    let sym0 = &mut Some(nt);
                    result = try!(__state4(lookahead, tokens, sym0));
                }
                __Nonterminal::Expr1(nt) => {
                    let sym0 = &mut Some(nt);
                    result = try!(__state1(lookahead, tokens, sym0));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
    }

    // State 1
    //   Expr = Expr1 (*) [EOF]
    //   Expr1 = Expr1 (*) "+" Factor [EOF]
    //   Expr1 = Expr1 (*) "+" Factor ["+"]
    //   Expr1 = Expr1 (*) "+" Factor ["-"]
    //   Expr1 = Expr1 (*) "-" Factor [EOF]
    //   Expr1 = Expr1 (*) "-" Factor ["+"]
    //   Expr1 = Expr1 (*) "-" Factor ["-"]
    //
    //   EOF -> Reduce(Expr = Expr1 => ActionFn(0);)
    //   "+" -> Shift(S7)
    //   "-" -> Shift(S6)
    //
    pub fn __state1<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Plus(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state7(lookahead, tokens, sym0, sym1));
            }
            Some(tok @ Tok::Minus(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state6(lookahead, tokens, sym0, sym1));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action0(sym0);
                return Ok((lookahead, __Nonterminal::Expr(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 2
    //   Term = "Num" (*) [EOF]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "-" -> Reduce(Term = "Num" => ActionFn(7);)
    //   EOF -> Reduce(Term = "Num" => ActionFn(7);)
    //   "*" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "+" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "/" -> Reduce(Term = "Num" => ActionFn(7);)
    //
    pub fn __state2<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 3
    //   Expr1 = Factor (*) [EOF]
    //   Expr1 = Factor (*) ["+"]
    //   Expr1 = Factor (*) ["-"]
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
    //   EOF -> Reduce(Expr1 = Factor => ActionFn(3);)
    //   "-" -> Reduce(Expr1 = Factor => ActionFn(3);)
    //   "/" -> Shift(S8)
    //   "+" -> Reduce(Expr1 = Factor => ActionFn(3);)
    //   "*" -> Shift(S9)
    //
    pub fn __state3<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Div(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state8(lookahead, tokens, sym0, sym1));
            }
            Some(tok @ Tok::Times(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state9(lookahead, tokens, sym0, sym1));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 4
    //   Factor = Term (*) [EOF]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "*" -> Reduce(Factor = Term => ActionFn(6);)
    //   "+" -> Reduce(Factor = Term => ActionFn(6);)
    //   EOF -> Reduce(Factor = Term => ActionFn(6);)
    //   "-" -> Reduce(Factor = Term => ActionFn(6);)
    //   "/" -> Reduce(Factor = Term => ActionFn(6);)
    //
    pub fn __state4<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 5
    //   Expr1 = (*) Expr1 "+" Factor [")"]
    //   Expr1 = (*) Expr1 "+" Factor ["+"]
    //   Expr1 = (*) Expr1 "+" Factor ["-"]
    //   Expr1 = (*) Expr1 "-" Factor [")"]
    //   Expr1 = (*) Expr1 "-" Factor ["+"]
    //   Expr1 = (*) Expr1 "-" Factor ["-"]
    //   Expr1 = (*) Factor [")"]
    //   Expr1 = (*) Factor ["+"]
    //   Expr1 = (*) Factor ["-"]
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
    //   Term = (*) "(" Expr1 ")" [")"]
    //   Term = (*) "(" Expr1 ")" ["*"]
    //   Term = (*) "(" Expr1 ")" ["+"]
    //   Term = (*) "(" Expr1 ")" ["-"]
    //   Term = (*) "(" Expr1 ")" ["/"]
    //   Term = "(" (*) Expr1 ")" [EOF]
    //   Term = "(" (*) Expr1 ")" ["*"]
    //   Term = "(" (*) Expr1 ")" ["+"]
    //   Term = "(" (*) Expr1 ")" ["-"]
    //   Term = "(" (*) Expr1 ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S12)
    //   "Num" -> Shift(S13)
    //
    //   Expr1 -> S11
    //   Factor -> S14
    //   Term -> S10
    pub fn __state5<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state12(lookahead, tokens, sym1));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym1));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym0.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Expr1(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state11(lookahead, tokens, sym0, sym1));
                }
                __Nonterminal::Factor(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state14(lookahead, tokens, sym1));
                }
                __Nonterminal::Term(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state10(lookahead, tokens, sym1));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 6
    //   Expr1 = Expr1 "-" (*) Factor [EOF]
    //   Expr1 = Expr1 "-" (*) Factor ["+"]
    //   Expr1 = Expr1 "-" (*) Factor ["-"]
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
    //   Term = (*) "(" Expr1 ")" [EOF]
    //   Term = (*) "(" Expr1 ")" ["*"]
    //   Term = (*) "(" Expr1 ")" ["+"]
    //   Term = (*) "(" Expr1 ")" ["-"]
    //   Term = (*) "(" Expr1 ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S2)
    //   "(" -> Shift(S5)
    //
    //   Factor -> S15
    //   Term -> S4
    pub fn __state6<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state2(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Factor(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state15(lookahead, tokens, sym0, sym1, sym2));
                }
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state4(lookahead, tokens, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 7
    //   Expr1 = Expr1 "+" (*) Factor [EOF]
    //   Expr1 = Expr1 "+" (*) Factor ["+"]
    //   Expr1 = Expr1 "+" (*) Factor ["-"]
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
    //   Term = (*) "(" Expr1 ")" [EOF]
    //   Term = (*) "(" Expr1 ")" ["*"]
    //   Term = (*) "(" Expr1 ")" ["+"]
    //   Term = (*) "(" Expr1 ")" ["-"]
    //   Term = (*) "(" Expr1 ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S2)
    //   "(" -> Shift(S5)
    //
    //   Factor -> S16
    //   Term -> S4
    pub fn __state7<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state2(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Factor(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state16(lookahead, tokens, sym0, sym1, sym2));
                }
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state4(lookahead, tokens, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 8
    //   Factor = Factor "/" (*) Term [EOF]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr1 ")" [EOF]
    //   Term = (*) "(" Expr1 ")" ["*"]
    //   Term = (*) "(" Expr1 ")" ["+"]
    //   Term = (*) "(" Expr1 ")" ["-"]
    //   Term = (*) "(" Expr1 ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S2)
    //   "(" -> Shift(S5)
    //
    //   Term -> S17
    pub fn __state8<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state2(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state17(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 9
    //   Factor = Factor "*" (*) Term [EOF]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr1 ")" [EOF]
    //   Term = (*) "(" Expr1 ")" ["*"]
    //   Term = (*) "(" Expr1 ")" ["+"]
    //   Term = (*) "(" Expr1 ")" ["-"]
    //   Term = (*) "(" Expr1 ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S5)
    //   "Num" -> Shift(S2)
    //
    //   Term -> S18
    pub fn __state9<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state2(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state18(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 10
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "*" -> Reduce(Factor = Term => ActionFn(6);)
    //   "/" -> Reduce(Factor = Term => ActionFn(6);)
    //   ")" -> Reduce(Factor = Term => ActionFn(6);)
    //   "-" -> Reduce(Factor = Term => ActionFn(6);)
    //   "+" -> Reduce(Factor = Term => ActionFn(6);)
    //
    pub fn __state10<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action6(sym0);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 11
    //   Expr1 = Expr1 (*) "+" Factor [")"]
    //   Expr1 = Expr1 (*) "+" Factor ["+"]
    //   Expr1 = Expr1 (*) "+" Factor ["-"]
    //   Expr1 = Expr1 (*) "-" Factor [")"]
    //   Expr1 = Expr1 (*) "-" Factor ["+"]
    //   Expr1 = Expr1 (*) "-" Factor ["-"]
    //   Term = "(" Expr1 (*) ")" [EOF]
    //   Term = "(" Expr1 (*) ")" ["*"]
    //   Term = "(" Expr1 (*) ")" ["+"]
    //   Term = "(" Expr1 (*) ")" ["-"]
    //   Term = "(" Expr1 (*) ")" ["/"]
    //
    //   "-" -> Shift(S19)
    //   ")" -> Shift(S20)
    //   "+" -> Shift(S21)
    //
    pub fn __state11<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Minus(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state19(lookahead, tokens, sym1, sym2));
            }
            Some(tok @ Tok::RParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state20(lookahead, tokens, sym0, sym1, sym2));
            }
            Some(tok @ Tok::Plus(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state21(lookahead, tokens, sym1, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 12
    //   Expr1 = (*) Expr1 "+" Factor [")"]
    //   Expr1 = (*) Expr1 "+" Factor ["+"]
    //   Expr1 = (*) Expr1 "+" Factor ["-"]
    //   Expr1 = (*) Expr1 "-" Factor [")"]
    //   Expr1 = (*) Expr1 "-" Factor ["+"]
    //   Expr1 = (*) Expr1 "-" Factor ["-"]
    //   Expr1 = (*) Factor [")"]
    //   Expr1 = (*) Factor ["+"]
    //   Expr1 = (*) Factor ["-"]
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
    //   Term = (*) "(" Expr1 ")" [")"]
    //   Term = (*) "(" Expr1 ")" ["*"]
    //   Term = (*) "(" Expr1 ")" ["+"]
    //   Term = (*) "(" Expr1 ")" ["-"]
    //   Term = (*) "(" Expr1 ")" ["/"]
    //   Term = "(" (*) Expr1 ")" [")"]
    //   Term = "(" (*) Expr1 ")" ["*"]
    //   Term = "(" (*) Expr1 ")" ["+"]
    //   Term = "(" (*) Expr1 ")" ["-"]
    //   Term = "(" (*) Expr1 ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S13)
    //   "(" -> Shift(S12)
    //
    //   Term -> S10
    //   Expr1 -> S22
    //   Factor -> S14
    pub fn __state12<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Num(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym1));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state12(lookahead, tokens, sym1));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym0.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state10(lookahead, tokens, sym1));
                }
                __Nonterminal::Expr1(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state22(lookahead, tokens, sym0, sym1));
                }
                __Nonterminal::Factor(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state14(lookahead, tokens, sym1));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 13
    //   Term = "Num" (*) [")"]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "+" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "-" -> Reduce(Term = "Num" => ActionFn(7);)
    //   ")" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "*" -> Reduce(Term = "Num" => ActionFn(7);)
    //   "/" -> Reduce(Term = "Num" => ActionFn(7);)
    //
    pub fn __state13<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action7(sym0);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 14
    //   Expr1 = Factor (*) [")"]
    //   Expr1 = Factor (*) ["+"]
    //   Expr1 = Factor (*) ["-"]
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
    //   "*" -> Shift(S24)
    //   ")" -> Reduce(Expr1 = Factor => ActionFn(3);)
    //   "+" -> Reduce(Expr1 = Factor => ActionFn(3);)
    //   "/" -> Shift(S23)
    //   "-" -> Reduce(Expr1 = Factor => ActionFn(3);)
    //
    pub fn __state14<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Times(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state24(lookahead, tokens, sym0, sym1));
            }
            Some(tok @ Tok::Div(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state23(lookahead, tokens, sym0, sym1));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 15
    //   Expr1 = Expr1 "-" Factor (*) [EOF]
    //   Expr1 = Expr1 "-" Factor (*) ["+"]
    //   Expr1 = Expr1 "-" Factor (*) ["-"]
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
    //   "+" -> Reduce(Expr1 = Expr1, "-", Factor => ActionFn(1);)
    //   "-" -> Reduce(Expr1 = Expr1, "-", Factor => ActionFn(1);)
    //   EOF -> Reduce(Expr1 = Expr1, "-", Factor => ActionFn(1);)
    //   "/" -> Shift(S8)
    //   "*" -> Shift(S9)
    //
    pub fn __state15<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Div(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state8(lookahead, tokens, sym2, sym3));
            }
            Some(tok @ Tok::Times(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state9(lookahead, tokens, sym2, sym3));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 16
    //   Expr1 = Expr1 "+" Factor (*) [EOF]
    //   Expr1 = Expr1 "+" Factor (*) ["+"]
    //   Expr1 = Expr1 "+" Factor (*) ["-"]
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
    //   EOF -> Reduce(Expr1 = Expr1, "+", Factor => ActionFn(2);)
    //   "-" -> Reduce(Expr1 = Expr1, "+", Factor => ActionFn(2);)
    //   "+" -> Reduce(Expr1 = Expr1, "+", Factor => ActionFn(2);)
    //   "*" -> Shift(S9)
    //   "/" -> Shift(S8)
    //
    pub fn __state16<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Times(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state9(lookahead, tokens, sym2, sym3));
            }
            Some(tok @ Tok::Div(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state8(lookahead, tokens, sym2, sym3));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 17
    //   Factor = Factor "/" Term (*) [EOF]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "-" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "*" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   EOF -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "+" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "/" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //
    pub fn __state17<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 18
    //   Factor = Factor "*" Term (*) [EOF]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "*" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   EOF -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "+" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "-" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //
    pub fn __state18<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 19
    //   Expr1 = Expr1 "-" (*) Factor [")"]
    //   Expr1 = Expr1 "-" (*) Factor ["+"]
    //   Expr1 = Expr1 "-" (*) Factor ["-"]
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
    //   Term = (*) "(" Expr1 ")" [")"]
    //   Term = (*) "(" Expr1 ")" ["*"]
    //   Term = (*) "(" Expr1 ")" ["+"]
    //   Term = (*) "(" Expr1 ")" ["-"]
    //   Term = (*) "(" Expr1 ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S12)
    //   "Num" -> Shift(S13)
    //
    //   Term -> S10
    //   Factor -> S25
    pub fn __state19<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state12(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state10(lookahead, tokens, sym2));
                }
                __Nonterminal::Factor(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state25(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 20
    //   Term = "(" Expr1 ")" (*) [EOF]
    //   Term = "(" Expr1 ")" (*) ["*"]
    //   Term = "(" Expr1 ")" (*) ["+"]
    //   Term = "(" Expr1 ")" (*) ["-"]
    //   Term = "(" Expr1 ")" (*) ["/"]
    //
    //   "/" -> Reduce(Term = "(", Expr1, ")" => ActionFn(8);)
    //   EOF -> Reduce(Term = "(", Expr1, ")" => ActionFn(8);)
    //   "*" -> Reduce(Term = "(", Expr1, ")" => ActionFn(8);)
    //   "+" -> Reduce(Term = "(", Expr1, ")" => ActionFn(8);)
    //   "-" -> Reduce(Term = "(", Expr1, ")" => ActionFn(8);)
    //
    pub fn __state20<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<i32>,
        sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 21
    //   Expr1 = Expr1 "+" (*) Factor [")"]
    //   Expr1 = Expr1 "+" (*) Factor ["+"]
    //   Expr1 = Expr1 "+" (*) Factor ["-"]
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
    //   Term = (*) "(" Expr1 ")" [")"]
    //   Term = (*) "(" Expr1 ")" ["*"]
    //   Term = (*) "(" Expr1 ")" ["+"]
    //   Term = (*) "(" Expr1 ")" ["-"]
    //   Term = (*) "(" Expr1 ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S12)
    //   "Num" -> Shift(S13)
    //
    //   Term -> S10
    //   Factor -> S26
    pub fn __state21<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state12(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state10(lookahead, tokens, sym2));
                }
                __Nonterminal::Factor(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state26(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 22
    //   Expr1 = Expr1 (*) "+" Factor [")"]
    //   Expr1 = Expr1 (*) "+" Factor ["+"]
    //   Expr1 = Expr1 (*) "+" Factor ["-"]
    //   Expr1 = Expr1 (*) "-" Factor [")"]
    //   Expr1 = Expr1 (*) "-" Factor ["+"]
    //   Expr1 = Expr1 (*) "-" Factor ["-"]
    //   Term = "(" Expr1 (*) ")" [")"]
    //   Term = "(" Expr1 (*) ")" ["*"]
    //   Term = "(" Expr1 (*) ")" ["+"]
    //   Term = "(" Expr1 (*) ")" ["-"]
    //   Term = "(" Expr1 (*) ")" ["/"]
    //
    //   "+" -> Shift(S21)
    //   "-" -> Shift(S19)
    //   ")" -> Shift(S27)
    //
    pub fn __state22<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Plus(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state21(lookahead, tokens, sym1, sym2));
            }
            Some(tok @ Tok::Minus(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state19(lookahead, tokens, sym1, sym2));
            }
            Some(tok @ Tok::RParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state27(lookahead, tokens, sym0, sym1, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 23
    //   Factor = Factor "/" (*) Term [")"]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr1 ")" [")"]
    //   Term = (*) "(" Expr1 ")" ["*"]
    //   Term = (*) "(" Expr1 ")" ["+"]
    //   Term = (*) "(" Expr1 ")" ["-"]
    //   Term = (*) "(" Expr1 ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S12)
    //   "Num" -> Shift(S13)
    //
    //   Term -> S28
    pub fn __state23<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state12(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state28(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 24
    //   Factor = Factor "*" (*) Term [")"]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr1 ")" [")"]
    //   Term = (*) "(" Expr1 ")" ["*"]
    //   Term = (*) "(" Expr1 ")" ["+"]
    //   Term = (*) "(" Expr1 ")" ["-"]
    //   Term = (*) "(" Expr1 ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S12)
    //   "Num" -> Shift(S13)
    //
    //   Term -> S29
    pub fn __state24<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state12(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::Term(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state29(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 25
    //   Expr1 = Expr1 "-" Factor (*) [")"]
    //   Expr1 = Expr1 "-" Factor (*) ["+"]
    //   Expr1 = Expr1 "-" Factor (*) ["-"]
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
    //   ")" -> Reduce(Expr1 = Expr1, "-", Factor => ActionFn(1);)
    //   "*" -> Shift(S24)
    //   "-" -> Reduce(Expr1 = Expr1, "-", Factor => ActionFn(1);)
    //   "+" -> Reduce(Expr1 = Expr1, "-", Factor => ActionFn(1);)
    //   "/" -> Shift(S23)
    //
    pub fn __state25<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Times(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state24(lookahead, tokens, sym2, sym3));
            }
            Some(tok @ Tok::Div(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state23(lookahead, tokens, sym2, sym3));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 26
    //   Expr1 = Expr1 "+" Factor (*) [")"]
    //   Expr1 = Expr1 "+" Factor (*) ["+"]
    //   Expr1 = Expr1 "+" Factor (*) ["-"]
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
    //   ")" -> Reduce(Expr1 = Expr1, "+", Factor => ActionFn(2);)
    //   "*" -> Shift(S24)
    //   "/" -> Shift(S23)
    //   "-" -> Reduce(Expr1 = Expr1, "+", Factor => ActionFn(2);)
    //   "+" -> Reduce(Expr1 = Expr1, "+", Factor => ActionFn(2);)
    //
    pub fn __state26<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Times(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state24(lookahead, tokens, sym2, sym3));
            }
            Some(tok @ Tok::Div(..)) => {
                let sym3 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state23(lookahead, tokens, sym2, sym3));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Expr1(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 27
    //   Term = "(" Expr1 ")" (*) [")"]
    //   Term = "(" Expr1 ")" (*) ["*"]
    //   Term = "(" Expr1 ")" (*) ["+"]
    //   Term = "(" Expr1 ")" (*) ["-"]
    //   Term = "(" Expr1 ")" (*) ["/"]
    //
    //   ")" -> Reduce(Term = "(", Expr1, ")" => ActionFn(8);)
    //   "*" -> Reduce(Term = "(", Expr1, ")" => ActionFn(8);)
    //   "+" -> Reduce(Term = "(", Expr1, ")" => ActionFn(8);)
    //   "-" -> Reduce(Term = "(", Expr1, ")" => ActionFn(8);)
    //   "/" -> Reduce(Term = "(", Expr1, ")" => ActionFn(8);)
    //
    pub fn __state27<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<i32>,
        sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action8(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Term(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 28
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "/" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "*" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "-" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "+" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //
    pub fn __state28<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 29
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "-" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "+" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "*" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   ")" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //
    pub fn __state29<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Div(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Plus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::Times(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::Factor(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }
}
fn __action0(
    __0: i32,
) -> i32 {
    (__0)
}
fn __action1(
    l: i32,
    _: Tok,
    r: i32,
) -> i32 {
    l - r
}
fn __action2(
    l: i32,
    _: Tok,
    r: i32,
) -> i32 {
    l + r
}
fn __action3(
    __0: i32,
) -> i32 {
    (__0)
}
fn __action4(
    l: i32,
    _: Tok,
    r: i32,
) -> i32 {
    l * r
}
fn __action5(
    l: i32,
    _: Tok,
    r: i32,
) -> i32 {
    l / r
}
fn __action6(
    __0: i32,
) -> i32 {
    (__0)
}
fn __action7(
    n: Tok,
) -> i32 {
    n.as_num()
}
fn __action8(
    _: Tok,
    __0: i32,
    _: Tok,
) -> i32 {
    (__0)
}
