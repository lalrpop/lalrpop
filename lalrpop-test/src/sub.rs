use util::tok::Tok;

#[allow(non_snake_case)]
pub fn parse_S<TOKENS: IntoIterator<Item=Tok>>(
    tokens: TOKENS)
    -> Result<(Option<Tok>, i32), Option<Tok>>
{
    let mut tokens = tokens.into_iter();
    let lookahead = tokens.next();
    match try!(__parse__S::__state0(lookahead, &mut tokens)) {
        (lookahead, __parse__S::__Nonterminal::__S(nt)) => Ok((lookahead, nt)),
        _ => unreachable!(),
    }
}

mod __parse__S {
    #![allow(non_snake_case, unused_mut, unused_variables)]

    use util::tok::Tok;

    pub enum __Nonterminal {
        E(i32),
        __S(i32),
        T(i32),
        S(i32),
    }

    // State 0
    //   E = (*) E "-" T [EOF]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T [EOF]
    //   E = (*) T ["-"]
    //   S = (*) E [EOF]
    //   T = (*) "(" E ")" [EOF]
    //   T = (*) "(" E ")" ["-"]
    //   T = (*) "Num" [EOF]
    //   T = (*) "Num" ["-"]
    //   __S = (*) S [EOF]
    //
    //   "(" -> Shift(S5)
    //   "Num" -> Shift(S1)
    //
    //   S -> S3
    //   T -> S2
    //   E -> S4
    pub fn __state0<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::LParen(..)) => {
                let sym0 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym0));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym0 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state1(lookahead, tokens, sym0));
            }
            _ => {
                return Err(lookahead);
            }
        }
        loop {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::S(nt) => {
                    let sym0 = &mut Some(nt);
                    result = try!(__state3(lookahead, tokens, sym0));
                }
                __Nonterminal::T(nt) => {
                    let sym0 = &mut Some(nt);
                    result = try!(__state2(lookahead, tokens, sym0));
                }
                __Nonterminal::E(nt) => {
                    let sym0 = &mut Some(nt);
                    result = try!(__state4(lookahead, tokens, sym0));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
    }

    // State 1
    //   T = "Num" (*) [EOF]
    //   T = "Num" (*) ["-"]
    //
    //   EOF -> Reduce(T = "Num" => ActionFn(4);)
    //   "-" -> Reduce(T = "Num" => ActionFn(4);)
    //
    pub fn __state1<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action4(sym0);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action4(sym0);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 2
    //   E = T (*) [EOF]
    //   E = T (*) ["-"]
    //
    //   EOF -> Reduce(E = T => ActionFn(3);)
    //   "-" -> Reduce(E = T => ActionFn(3);)
    //
    pub fn __state2<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 3
    //   __S = S (*) [EOF]
    //
    //   EOF -> Reduce(__S = S => ActionFn(0);)
    //
    pub fn __state3<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action0(sym0);
                return Ok((lookahead, __Nonterminal::__S(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 4
    //   E = E (*) "-" T [EOF]
    //   E = E (*) "-" T ["-"]
    //   S = E (*) [EOF]
    //
    //   "-" -> Shift(S6)
    //   EOF -> Reduce(S = E => ActionFn(1);)
    //
    pub fn __state4<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Minus(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state6(lookahead, tokens, sym0, sym1));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action1(sym0);
                return Ok((lookahead, __Nonterminal::S(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 5
    //   E = (*) E "-" T [")"]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T [")"]
    //   E = (*) T ["-"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "(" E ")" ["-"]
    //   T = "(" (*) E ")" [EOF]
    //   T = "(" (*) E ")" ["-"]
    //   T = (*) "Num" [")"]
    //   T = (*) "Num" ["-"]
    //
    //   "(" -> Shift(S10)
    //   "Num" -> Shift(S8)
    //
    //   E -> S7
    //   T -> S9
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
                result = try!(__state10(lookahead, tokens, sym1));
            }
            Some(tok @ Tok::Num(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state8(lookahead, tokens, sym1));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym0.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::E(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state7(lookahead, tokens, sym0, sym1));
                }
                __Nonterminal::T(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state9(lookahead, tokens, sym1));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 6
    //   E = E "-" (*) T [EOF]
    //   E = E "-" (*) T ["-"]
    //   T = (*) "(" E ")" [EOF]
    //   T = (*) "(" E ")" ["-"]
    //   T = (*) "Num" [EOF]
    //   T = (*) "Num" ["-"]
    //
    //   "(" -> Shift(S5)
    //   "Num" -> Shift(S1)
    //
    //   T -> S11
    pub fn __state6<TOKENS: Iterator<Item=Tok>>(
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
                result = try!(__state1(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::T(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state11(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 7
    //   E = E (*) "-" T [")"]
    //   E = E (*) "-" T ["-"]
    //   T = "(" E (*) ")" [EOF]
    //   T = "(" E (*) ")" ["-"]
    //
    //   "-" -> Shift(S13)
    //   ")" -> Shift(S12)
    //
    pub fn __state7<TOKENS: Iterator<Item=Tok>>(
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
                result = try!(__state13(lookahead, tokens, sym1, sym2));
            }
            Some(tok @ Tok::RParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state12(lookahead, tokens, sym0, sym1, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 8
    //   T = "Num" (*) [")"]
    //   T = "Num" (*) ["-"]
    //
    //   ")" -> Reduce(T = "Num" => ActionFn(4);)
    //   "-" -> Reduce(T = "Num" => ActionFn(4);)
    //
    pub fn __state8<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action4(sym0);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action4(sym0);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 9
    //   E = T (*) [")"]
    //   E = T (*) ["-"]
    //
    //   ")" -> Reduce(E = T => ActionFn(3);)
    //   "-" -> Reduce(E = T => ActionFn(3);)
    //
    pub fn __state9<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 10
    //   E = (*) E "-" T [")"]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T [")"]
    //   E = (*) T ["-"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "(" E ")" ["-"]
    //   T = "(" (*) E ")" [")"]
    //   T = "(" (*) E ")" ["-"]
    //   T = (*) "Num" [")"]
    //   T = (*) "Num" ["-"]
    //
    //   "Num" -> Shift(S8)
    //   "(" -> Shift(S10)
    //
    //   T -> S9
    //   E -> S14
    pub fn __state10<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::Num(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state8(lookahead, tokens, sym1));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym1 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state10(lookahead, tokens, sym1));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym0.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::T(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state9(lookahead, tokens, sym1));
                }
                __Nonterminal::E(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state14(lookahead, tokens, sym0, sym1));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 11
    //   E = E "-" T (*) [EOF]
    //   E = E "-" T (*) ["-"]
    //
    //   EOF -> Reduce(E = E, "-", T => ActionFn(2);)
    //   "-" -> Reduce(E = E, "-", T => ActionFn(2);)
    //
    pub fn __state11<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<i32>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 12
    //   T = "(" E ")" (*) [EOF]
    //   T = "(" E ")" (*) ["-"]
    //
    //   EOF -> Reduce(T = "(", E, ")" => ActionFn(5);)
    //   "-" -> Reduce(T = "(", E, ")" => ActionFn(5);)
    //
    pub fn __state12<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<i32>,
        sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 13
    //   E = E "-" (*) T [")"]
    //   E = E "-" (*) T ["-"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "(" E ")" ["-"]
    //   T = (*) "Num" [")"]
    //   T = (*) "Num" ["-"]
    //
    //   "Num" -> Shift(S8)
    //   "(" -> Shift(S10)
    //
    //   T -> S15
    pub fn __state13<TOKENS: Iterator<Item=Tok>>(
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
                result = try!(__state8(lookahead, tokens, sym2));
            }
            Some(tok @ Tok::LParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state10(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::T(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state15(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 14
    //   E = E (*) "-" T [")"]
    //   E = E (*) "-" T ["-"]
    //   T = "(" E (*) ")" [")"]
    //   T = "(" E (*) ")" ["-"]
    //
    //   ")" -> Shift(S16)
    //   "-" -> Shift(S13)
    //
    pub fn __state14<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(tok @ Tok::RParen(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state16(lookahead, tokens, sym0, sym1, sym2));
            }
            Some(tok @ Tok::Minus(..)) => {
                let sym2 = &mut Some(tok);
                let lookahead = tokens.next();
                result = try!(__state13(lookahead, tokens, sym1, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 15
    //   E = E "-" T (*) [")"]
    //   E = E "-" T (*) ["-"]
    //
    //   "-" -> Reduce(E = E, "-", T => ActionFn(2);)
    //   ")" -> Reduce(E = E, "-", T => ActionFn(2);)
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
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action2(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 16
    //   T = "(" E ")" (*) [")"]
    //   T = "(" E ")" (*) ["-"]
    //
    //   "-" -> Reduce(T = "(", E, ")" => ActionFn(5);)
    //   ")" -> Reduce(T = "(", E, ")" => ActionFn(5);)
    //
    pub fn __state16<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<i32>,
        sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action5(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::T(nt)));
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
    __0: i32,
) -> i32 {
    (__0)
}
fn __action2(
    l: i32,
    _: Tok,
    r: i32,
) -> i32 {
    l-r
}
fn __action3(
    __0: i32,
) -> i32 {
    (__0)
}
fn __action4(
    n: Tok,
) -> i32 {
    n.as_num()
}
fn __action5(
    _: Tok,
    __0: i32,
    _: Tok,
) -> i32 {
    (__0)
}
