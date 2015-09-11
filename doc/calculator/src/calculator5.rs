#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use ast::{Expr, Opcode};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Exprs {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Expr, Opcode};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Exprs<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Box<Expr>>, __ParseError<usize,(usize, &'input str),()>>
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
            (_, None, __Nonterminal::____Exprs(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        Tier_3cFactorOp_2c_20Term_3e(Box<Expr>),
        Comma_3cExpr_3e(Vec<Box<Expr>>),
        Exprs(Vec<Box<Expr>>),
        _28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<Box<Expr>>),
        Factor(Box<Expr>),
        Expr_3f(::std::option::Option<Box<Expr>>),
        ExprOp(Opcode),
        Tier_3cExprOp_2c_20Factor_3e(Box<Expr>),
        Expr(Box<Expr>),
        ____Exprs(Vec<Box<Expr>>),
        Num(i32),
        Term(Box<Expr>),
        FactorOp(Opcode),
        _28_3cExpr_3e_20_22_2c_22_29(Box<Expr>),
    }

    // State 0
    //   (<Expr> ",")* = (*) [EOF]
    //   (<Expr> ",")* = (*) ["("]
    //   (<Expr> ",")* = (*) [r#"[0-9]+"#]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [EOF]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["("]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [r#"[0-9]+"#]
    //   Comma<Expr> = (*) (<Expr> ",")* Expr? [EOF]
    //   Exprs = (*) Comma<Expr> [EOF]
    //   __Exprs = (*) Exprs [EOF]
    //
    //   "(" -> Reduce((<Expr> ",")* =  => Call(ActionFn(18));)
    //   r#"[0-9]+"# -> Reduce((<Expr> ",")* =  => Call(ActionFn(18));)
    //   EOF -> Reduce((<Expr> ",")* =  => Call(ActionFn(18));)
    //
    //   (<Expr> ",")* -> S3
    //   Exprs -> S1
    //   Comma<Expr> -> S2
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
            Some((_, (7, _), _)) |
            Some((_, (2, _), _)) |
            None => {
                let __nt = super::__action18(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
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
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Exprs(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __Exprs = Exprs (*) [EOF]
    //
    //   EOF -> Reduce(__Exprs = Exprs => Call(ActionFn(0));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Vec<Box<Expr>>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Exprs(__nt)));
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
    //   Exprs = Comma<Expr> (*) [EOF]
    //
    //   EOF -> Reduce(Exprs = Comma<Expr> => Call(ActionFn(1));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Vec<Box<Expr>>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action1(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Exprs(__nt)));
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
    //   (<Expr> ",") = (*) Expr "," [EOF]
    //   (<Expr> ",") = (*) Expr "," ["("]
    //   (<Expr> ",") = (*) Expr "," [r#"[0-9]+"#]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") [EOF]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") ["("]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") [r#"[0-9]+"#]
    //   Comma<Expr> = (<Expr> ",")* (*) Expr? [EOF]
    //   Expr = (*) Tier<ExprOp, Factor> [EOF]
    //   Expr = (*) Tier<ExprOp, Factor> [","]
    //   Expr? = (*) [EOF]
    //   Expr? = (*) Expr [EOF]
    //   Factor = (*) Tier<FactorOp, Term> [EOF]
    //   Factor = (*) Tier<FactorOp, Term> ["+"]
    //   Factor = (*) Tier<FactorOp, Term> [","]
    //   Factor = (*) Tier<FactorOp, Term> ["-"]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# [","]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num [","]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Tier<ExprOp, Factor> = (*) Factor [EOF]
    //   Tier<ExprOp, Factor> = (*) Factor ["+"]
    //   Tier<ExprOp, Factor> = (*) Factor [","]
    //   Tier<ExprOp, Factor> = (*) Factor ["-"]
    //   Tier<ExprOp, Factor> = (*) Tier<ExprOp, Factor> ExprOp Factor [EOF]
    //   Tier<ExprOp, Factor> = (*) Tier<ExprOp, Factor> ExprOp Factor ["+"]
    //   Tier<ExprOp, Factor> = (*) Tier<ExprOp, Factor> ExprOp Factor [","]
    //   Tier<ExprOp, Factor> = (*) Tier<ExprOp, Factor> ExprOp Factor ["-"]
    //   Tier<FactorOp, Term> = (*) Term [EOF]
    //   Tier<FactorOp, Term> = (*) Term ["*"]
    //   Tier<FactorOp, Term> = (*) Term ["+"]
    //   Tier<FactorOp, Term> = (*) Term [","]
    //   Tier<FactorOp, Term> = (*) Term ["-"]
    //   Tier<FactorOp, Term> = (*) Term ["/"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term [EOF]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["*"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["+"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term [","]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["-"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["/"]
    //
    //   r#"[0-9]+"# -> Shift(S7)
    //   "(" -> Shift(S11)
    //   EOF -> Reduce(Expr? =  => Call(ActionFn(17));)
    //
    //   Num -> S6
    //   Expr? -> S5
    //   Tier<ExprOp, Factor> -> S10
    //   Term -> S4
    //   Factor -> S12
    //   Expr -> S13
    //   Tier<FactorOp, Term> -> S8
    //   (<Expr> ",") -> S9
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Box<Expr>>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym1));
            }
            None => {
                let __nt = super::__action17(input, );
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
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expr_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Tier_3cExprOp_2c_20Factor_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Tier_3cFactorOp_2c_20Term_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   Tier<FactorOp, Term> = Term (*) [EOF]
    //   Tier<FactorOp, Term> = Term (*) ["*"]
    //   Tier<FactorOp, Term> = Term (*) ["+"]
    //   Tier<FactorOp, Term> = Term (*) [","]
    //   Tier<FactorOp, Term> = Term (*) ["-"]
    //   Tier<FactorOp, Term> = Term (*) ["/"]
    //
    //   "," -> Reduce(Tier<FactorOp, Term> = Term => Call(ActionFn(12));)
    //   "*" -> Reduce(Tier<FactorOp, Term> = Term => Call(ActionFn(12));)
    //   EOF -> Reduce(Tier<FactorOp, Term> = Term => Call(ActionFn(12));)
    //   "+" -> Reduce(Tier<FactorOp, Term> = Term => Call(ActionFn(12));)
    //   "/" -> Reduce(Tier<FactorOp, Term> = Term => Call(ActionFn(12));)
    //   "-" -> Reduce(Tier<FactorOp, Term> = Term => Call(ActionFn(12));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            None |
            Some((_, (3, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Tier_3cFactorOp_2c_20Term_3e(__nt)));
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
    //   Comma<Expr> = (<Expr> ",")* Expr? (*) [EOF]
    //
    //   EOF -> Reduce(Comma<Expr> = (<Expr> ",")*, Expr? => Call(ActionFn(15));)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Box<Expr>>>,
        __sym1: &mut Option<::std::option::Option<Box<Expr>>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action15(input, __sym0, __sym1);
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

    // State 6
    //   Term = Num (*) [EOF]
    //   Term = Num (*) ["*"]
    //   Term = Num (*) ["+"]
    //   Term = Num (*) [","]
    //   Term = Num (*) ["-"]
    //   Term = Num (*) ["/"]
    //
    //   "-" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   EOF -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "," -> Reduce(Term = Num => Call(ActionFn(8));)
    //
    pub fn __state6<
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
            Some((_, (6, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            None |
            Some((_, (3, _), _)) |
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0);
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

    // State 7
    //   Num = r#"[0-9]+"# (*) [EOF]
    //   Num = r#"[0-9]+"# (*) ["*"]
    //   Num = r#"[0-9]+"# (*) ["+"]
    //   Num = r#"[0-9]+"# (*) [","]
    //   Num = r#"[0-9]+"# (*) ["-"]
    //   Num = r#"[0-9]+"# (*) ["/"]
    //
    //   "," -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(10));)
    //   "/" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(10));)
    //   EOF -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(10));)
    //   "*" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(10));)
    //   "-" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(10));)
    //   "+" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(10));)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (0, _), _)) |
            None |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
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

    // State 8
    //   Factor = Tier<FactorOp, Term> (*) [EOF]
    //   Factor = Tier<FactorOp, Term> (*) ["+"]
    //   Factor = Tier<FactorOp, Term> (*) [","]
    //   Factor = Tier<FactorOp, Term> (*) ["-"]
    //   FactorOp = (*) "*" ["("]
    //   FactorOp = (*) "*" [r#"[0-9]+"#]
    //   FactorOp = (*) "/" ["("]
    //   FactorOp = (*) "/" [r#"[0-9]+"#]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> (*) FactorOp Term [EOF]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> (*) FactorOp Term ["*"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> (*) FactorOp Term ["+"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> (*) FactorOp Term [","]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> (*) FactorOp Term ["-"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> (*) FactorOp Term ["/"]
    //
    //   "*" -> Shift(S16)
    //   "," -> Reduce(Factor = Tier<FactorOp, Term> => Call(ActionFn(3));)
    //   "+" -> Reduce(Factor = Tier<FactorOp, Term> => Call(ActionFn(3));)
    //   EOF -> Reduce(Factor = Tier<FactorOp, Term> => Call(ActionFn(3));)
    //   "/" -> Shift(S14)
    //   "-" -> Reduce(Factor = Tier<FactorOp, Term> => Call(ActionFn(3));)
    //
    //   FactorOp -> S15
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            None |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
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
                __Nonterminal::FactorOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 9
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) [EOF]
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) ["("]
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) [r#"[0-9]+"#]
    //
    //   r#"[0-9]+"# -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => Call(ActionFn(19));)
    //   EOF -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => Call(ActionFn(19));)
    //   "(" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => Call(ActionFn(19));)
    //
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Box<Expr>>>,
        __sym1: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            None |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action19(input, __sym0, __sym1);
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

    // State 10
    //   Expr = Tier<ExprOp, Factor> (*) [EOF]
    //   Expr = Tier<ExprOp, Factor> (*) [","]
    //   ExprOp = (*) "+" ["("]
    //   ExprOp = (*) "+" [r#"[0-9]+"#]
    //   ExprOp = (*) "-" ["("]
    //   ExprOp = (*) "-" [r#"[0-9]+"#]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> (*) ExprOp Factor [EOF]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> (*) ExprOp Factor ["+"]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> (*) ExprOp Factor [","]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> (*) ExprOp Factor ["-"]
    //
    //   "-" -> Shift(S18)
    //   EOF -> Reduce(Expr = Tier<ExprOp, Factor> => Call(ActionFn(2));)
    //   "+" -> Shift(S19)
    //   "," -> Reduce(Expr = Tier<ExprOp, Factor> => Call(ActionFn(2));)
    //
    //   ExprOp -> S17
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
            }
            None |
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
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
                __Nonterminal::ExprOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 11
    //   Expr = (*) Tier<ExprOp, Factor> [")"]
    //   Factor = (*) Tier<FactorOp, Term> [")"]
    //   Factor = (*) Tier<FactorOp, Term> ["+"]
    //   Factor = (*) Tier<FactorOp, Term> ["-"]
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
    //   Term = "(" (*) Expr ")" [","]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Tier<ExprOp, Factor> = (*) Factor [")"]
    //   Tier<ExprOp, Factor> = (*) Factor ["+"]
    //   Tier<ExprOp, Factor> = (*) Factor ["-"]
    //   Tier<ExprOp, Factor> = (*) Tier<ExprOp, Factor> ExprOp Factor [")"]
    //   Tier<ExprOp, Factor> = (*) Tier<ExprOp, Factor> ExprOp Factor ["+"]
    //   Tier<ExprOp, Factor> = (*) Tier<ExprOp, Factor> ExprOp Factor ["-"]
    //   Tier<FactorOp, Term> = (*) Term [")"]
    //   Tier<FactorOp, Term> = (*) Term ["*"]
    //   Tier<FactorOp, Term> = (*) Term ["+"]
    //   Tier<FactorOp, Term> = (*) Term ["-"]
    //   Tier<FactorOp, Term> = (*) Term ["/"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term [")"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["*"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["+"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["-"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["/"]
    //
    //   r#"[0-9]+"# -> Shift(S26)
    //   "(" -> Shift(S20)
    //
    //   Term -> S21
    //   Tier<ExprOp, Factor> -> S27
    //   Factor -> S23
    //   Expr -> S22
    //   Tier<FactorOp, Term> -> S25
    //   Num -> S24
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
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
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Tier_3cExprOp_2c_20Factor_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Tier_3cFactorOp_2c_20Term_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 12
    //   Tier<ExprOp, Factor> = Factor (*) [EOF]
    //   Tier<ExprOp, Factor> = Factor (*) ["+"]
    //   Tier<ExprOp, Factor> = Factor (*) [","]
    //   Tier<ExprOp, Factor> = Factor (*) ["-"]
    //
    //   EOF -> Reduce(Tier<ExprOp, Factor> = Factor => Call(ActionFn(14));)
    //   "+" -> Reduce(Tier<ExprOp, Factor> = Factor => Call(ActionFn(14));)
    //   "," -> Reduce(Tier<ExprOp, Factor> = Factor => Call(ActionFn(14));)
    //   "-" -> Reduce(Tier<ExprOp, Factor> = Factor => Call(ActionFn(14));)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action14(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Tier_3cExprOp_2c_20Factor_3e(__nt)));
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
    //   (<Expr> ",") = Expr (*) "," [EOF]
    //   (<Expr> ",") = Expr (*) "," ["("]
    //   (<Expr> ",") = Expr (*) "," [r#"[0-9]+"#]
    //   Expr? = Expr (*) [EOF]
    //
    //   EOF -> Reduce(Expr? = Expr => Call(ActionFn(16));)
    //   "," -> Shift(S28)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0);
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

    // State 14
    //   FactorOp = "/" (*) ["("]
    //   FactorOp = "/" (*) [r#"[0-9]+"#]
    //
    //   r#"[0-9]+"# -> Reduce(FactorOp = "/" => Call(ActionFn(7));)
    //   "(" -> Reduce(FactorOp = "/" => Call(ActionFn(7));)
    //
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::FactorOp(__nt)));
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
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# [","]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num [","]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp (*) Term [EOF]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp (*) Term ["*"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp (*) Term ["+"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp (*) Term [","]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp (*) Term ["-"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp (*) Term ["/"]
    //
    //   "(" -> Shift(S11)
    //   r#"[0-9]+"# -> Shift(S7)
    //
    //   Term -> S29
    //   Num -> S6
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
        __sym1: &mut Option<Opcode>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state29(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 16
    //   FactorOp = "*" (*) ["("]
    //   FactorOp = "*" (*) [r#"[0-9]+"#]
    //
    //   r#"[0-9]+"# -> Reduce(FactorOp = "*" => Call(ActionFn(6));)
    //   "(" -> Reduce(FactorOp = "*" => Call(ActionFn(6));)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::FactorOp(__nt)));
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
    //   Factor = (*) Tier<FactorOp, Term> [EOF]
    //   Factor = (*) Tier<FactorOp, Term> ["+"]
    //   Factor = (*) Tier<FactorOp, Term> [","]
    //   Factor = (*) Tier<FactorOp, Term> ["-"]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# ["*"]
    //   Num = (*) r#"[0-9]+"# ["+"]
    //   Num = (*) r#"[0-9]+"# [","]
    //   Num = (*) r#"[0-9]+"# ["-"]
    //   Num = (*) r#"[0-9]+"# ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num [","]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp (*) Factor [EOF]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp (*) Factor ["+"]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp (*) Factor [","]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp (*) Factor ["-"]
    //   Tier<FactorOp, Term> = (*) Term [EOF]
    //   Tier<FactorOp, Term> = (*) Term ["*"]
    //   Tier<FactorOp, Term> = (*) Term ["+"]
    //   Tier<FactorOp, Term> = (*) Term [","]
    //   Tier<FactorOp, Term> = (*) Term ["-"]
    //   Tier<FactorOp, Term> = (*) Term ["/"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term [EOF]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["*"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["+"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term [","]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["-"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["/"]
    //
    //   "(" -> Shift(S11)
    //   r#"[0-9]+"# -> Shift(S7)
    //
    //   Term -> S4
    //   Tier<FactorOp, Term> -> S8
    //   Factor -> S30
    //   Num -> S6
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
        __sym1: &mut Option<Opcode>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Tier_3cFactorOp_2c_20Term_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state30(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   ExprOp = "-" (*) ["("]
    //   ExprOp = "-" (*) [r#"[0-9]+"#]
    //
    //   "(" -> Reduce(ExprOp = "-" => Call(ActionFn(5));)
    //   r#"[0-9]+"# -> Reduce(ExprOp = "-" => Call(ActionFn(5));)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (7, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ExprOp(__nt)));
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
    //   ExprOp = "+" (*) ["("]
    //   ExprOp = "+" (*) [r#"[0-9]+"#]
    //
    //   "(" -> Reduce(ExprOp = "+" => Call(ActionFn(4));)
    //   r#"[0-9]+"# -> Reduce(ExprOp = "+" => Call(ActionFn(4));)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (7, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ExprOp(__nt)));
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
    //   Expr = (*) Tier<ExprOp, Factor> [")"]
    //   Factor = (*) Tier<FactorOp, Term> [")"]
    //   Factor = (*) Tier<FactorOp, Term> ["+"]
    //   Factor = (*) Tier<FactorOp, Term> ["-"]
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
    //   Tier<ExprOp, Factor> = (*) Factor [")"]
    //   Tier<ExprOp, Factor> = (*) Factor ["+"]
    //   Tier<ExprOp, Factor> = (*) Factor ["-"]
    //   Tier<ExprOp, Factor> = (*) Tier<ExprOp, Factor> ExprOp Factor [")"]
    //   Tier<ExprOp, Factor> = (*) Tier<ExprOp, Factor> ExprOp Factor ["+"]
    //   Tier<ExprOp, Factor> = (*) Tier<ExprOp, Factor> ExprOp Factor ["-"]
    //   Tier<FactorOp, Term> = (*) Term [")"]
    //   Tier<FactorOp, Term> = (*) Term ["*"]
    //   Tier<FactorOp, Term> = (*) Term ["+"]
    //   Tier<FactorOp, Term> = (*) Term ["-"]
    //   Tier<FactorOp, Term> = (*) Term ["/"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term [")"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["*"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["+"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["-"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["/"]
    //
    //   r#"[0-9]+"# -> Shift(S26)
    //   "(" -> Shift(S20)
    //
    //   Tier<FactorOp, Term> -> S25
    //   Num -> S24
    //   Factor -> S23
    //   Term -> S21
    //   Tier<ExprOp, Factor> -> S27
    //   Expr -> S31
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
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
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1));
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
                __Nonterminal::Tier_3cFactorOp_2c_20Term_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Tier_3cExprOp_2c_20Factor_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state31(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 21
    //   Tier<FactorOp, Term> = Term (*) [")"]
    //   Tier<FactorOp, Term> = Term (*) ["*"]
    //   Tier<FactorOp, Term> = Term (*) ["+"]
    //   Tier<FactorOp, Term> = Term (*) ["-"]
    //   Tier<FactorOp, Term> = Term (*) ["/"]
    //
    //   "+" -> Reduce(Tier<FactorOp, Term> = Term => Call(ActionFn(12));)
    //   "*" -> Reduce(Tier<FactorOp, Term> = Term => Call(ActionFn(12));)
    //   "-" -> Reduce(Tier<FactorOp, Term> = Term => Call(ActionFn(12));)
    //   "/" -> Reduce(Tier<FactorOp, Term> = Term => Call(ActionFn(12));)
    //   ")" -> Reduce(Tier<FactorOp, Term> = Term => Call(ActionFn(12));)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Tier_3cFactorOp_2c_20Term_3e(__nt)));
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
    //   Term = "(" Expr (*) ")" [EOF]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" [","]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   ")" -> Shift(S32)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state32(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 23
    //   Tier<ExprOp, Factor> = Factor (*) [")"]
    //   Tier<ExprOp, Factor> = Factor (*) ["+"]
    //   Tier<ExprOp, Factor> = Factor (*) ["-"]
    //
    //   "-" -> Reduce(Tier<ExprOp, Factor> = Factor => Call(ActionFn(14));)
    //   "+" -> Reduce(Tier<ExprOp, Factor> = Factor => Call(ActionFn(14));)
    //   ")" -> Reduce(Tier<ExprOp, Factor> = Factor => Call(ActionFn(14));)
    //
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (6, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action14(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Tier_3cExprOp_2c_20Factor_3e(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 24
    //   Term = Num (*) [")"]
    //   Term = Num (*) ["*"]
    //   Term = Num (*) ["+"]
    //   Term = Num (*) ["-"]
    //   Term = Num (*) ["/"]
    //
    //   ")" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = Num => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = Num => Call(ActionFn(8));)
    //
    pub fn __state24<
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
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0);
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
    //   Factor = Tier<FactorOp, Term> (*) [")"]
    //   Factor = Tier<FactorOp, Term> (*) ["+"]
    //   Factor = Tier<FactorOp, Term> (*) ["-"]
    //   FactorOp = (*) "*" ["("]
    //   FactorOp = (*) "*" [r#"[0-9]+"#]
    //   FactorOp = (*) "/" ["("]
    //   FactorOp = (*) "/" [r#"[0-9]+"#]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> (*) FactorOp Term [")"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> (*) FactorOp Term ["*"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> (*) FactorOp Term ["+"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> (*) FactorOp Term ["-"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> (*) FactorOp Term ["/"]
    //
    //   ")" -> Reduce(Factor = Tier<FactorOp, Term> => Call(ActionFn(3));)
    //   "+" -> Reduce(Factor = Tier<FactorOp, Term> => Call(ActionFn(3));)
    //   "*" -> Shift(S16)
    //   "-" -> Reduce(Factor = Tier<FactorOp, Term> => Call(ActionFn(3));)
    //   "/" -> Shift(S14)
    //
    //   FactorOp -> S33
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
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
                __Nonterminal::FactorOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state33(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 26
    //   Num = r#"[0-9]+"# (*) [")"]
    //   Num = r#"[0-9]+"# (*) ["*"]
    //   Num = r#"[0-9]+"# (*) ["+"]
    //   Num = r#"[0-9]+"# (*) ["-"]
    //   Num = r#"[0-9]+"# (*) ["/"]
    //
    //   "*" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(10));)
    //   ")" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(10));)
    //   "-" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(10));)
    //   "/" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(10));)
    //   "+" -> Reduce(Num = r#"[0-9]+"# => Call(ActionFn(10));)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
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

    // State 27
    //   Expr = Tier<ExprOp, Factor> (*) [")"]
    //   ExprOp = (*) "+" ["("]
    //   ExprOp = (*) "+" [r#"[0-9]+"#]
    //   ExprOp = (*) "-" ["("]
    //   ExprOp = (*) "-" [r#"[0-9]+"#]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> (*) ExprOp Factor [")"]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> (*) ExprOp Factor ["+"]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> (*) ExprOp Factor ["-"]
    //
    //   "+" -> Shift(S19)
    //   ")" -> Reduce(Expr = Tier<ExprOp, Factor> => Call(ActionFn(2));)
    //   "-" -> Shift(S18)
    //
    //   ExprOp -> S34
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
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
                __Nonterminal::ExprOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 28
    //   (<Expr> ",") = Expr "," (*) [EOF]
    //   (<Expr> ",") = Expr "," (*) ["("]
    //   (<Expr> ",") = Expr "," (*) [r#"[0-9]+"#]
    //
    //   "(" -> Reduce((<Expr> ",") = Expr, "," => Call(ActionFn(20));)
    //   r#"[0-9]+"# -> Reduce((<Expr> ",") = Expr, "," => Call(ActionFn(20));)
    //   EOF -> Reduce((<Expr> ",") = Expr, "," => Call(ActionFn(20));)
    //
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Box<Expr>>,
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
            Some((_, (7, _), _)) |
            Some((_, (2, _), _)) |
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action20(input, __sym0, __sym1);
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

    // State 29
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp Term (*) [EOF]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp Term (*) ["*"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp Term (*) ["+"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp Term (*) [","]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp Term (*) ["-"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp Term (*) ["/"]
    //
    //   "*" -> Reduce(Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => Call(ActionFn(11));)
    //   EOF -> Reduce(Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => Call(ActionFn(11));)
    //   "/" -> Reduce(Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => Call(ActionFn(11));)
    //   "," -> Reduce(Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => Call(ActionFn(11));)
    //   "-" -> Reduce(Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => Call(ActionFn(11));)
    //   "+" -> Reduce(Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => Call(ActionFn(11));)
    //
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
        __sym1: &mut Option<Opcode>,
        __sym2: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (4, _), _)) |
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action11(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Tier_3cFactorOp_2c_20Term_3e(__nt)));
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
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp Factor (*) [EOF]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp Factor (*) ["+"]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp Factor (*) [","]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp Factor (*) ["-"]
    //
    //   "-" -> Reduce(Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => Call(ActionFn(13));)
    //   EOF -> Reduce(Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => Call(ActionFn(13));)
    //   "," -> Reduce(Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => Call(ActionFn(13));)
    //   "+" -> Reduce(Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => Call(ActionFn(13));)
    //
    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
        __sym1: &mut Option<Opcode>,
        __sym2: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (6, _), _)) |
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Tier_3cExprOp_2c_20Factor_3e(__nt)));
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
    //   Term = "(" Expr (*) ")" [")"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   ")" -> Shift(S35)
    //
    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state35(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 32
    //   Term = "(" Expr ")" (*) [EOF]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) [","]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   EOF -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "," -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //
    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expr>>,
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
            Some((_, (4, _), _)) |
            Some((_, (0, _), _)) |
            None |
            Some((_, (6, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(input, __sym0, __sym1, __sym2);
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

    // State 33
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
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp (*) Term [")"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp (*) Term ["*"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp (*) Term ["+"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp (*) Term ["-"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp (*) Term ["/"]
    //
    //   r#"[0-9]+"# -> Shift(S26)
    //   "(" -> Shift(S20)
    //
    //   Term -> S36
    //   Num -> S24
    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
        __sym1: &mut Option<Opcode>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 34
    //   Factor = (*) Tier<FactorOp, Term> [")"]
    //   Factor = (*) Tier<FactorOp, Term> ["+"]
    //   Factor = (*) Tier<FactorOp, Term> ["-"]
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
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp (*) Factor [")"]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp (*) Factor ["+"]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp (*) Factor ["-"]
    //   Tier<FactorOp, Term> = (*) Term [")"]
    //   Tier<FactorOp, Term> = (*) Term ["*"]
    //   Tier<FactorOp, Term> = (*) Term ["+"]
    //   Tier<FactorOp, Term> = (*) Term ["-"]
    //   Tier<FactorOp, Term> = (*) Term ["/"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term [")"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["*"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["+"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["-"]
    //   Tier<FactorOp, Term> = (*) Tier<FactorOp, Term> FactorOp Term ["/"]
    //
    //   "(" -> Shift(S20)
    //   r#"[0-9]+"# -> Shift(S26)
    //
    //   Num -> S24
    //   Factor -> S37
    //   Term -> S21
    //   Tier<FactorOp, Term> -> S25
    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
        __sym1: &mut Option<Opcode>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym2));
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
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Tier_3cFactorOp_2c_20Term_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 35
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   ")" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //
    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Expr>>,
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
            Some((_, (6, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(input, __sym0, __sym1, __sym2);
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

    // State 36
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp Term (*) [")"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp Term (*) ["*"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp Term (*) ["+"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp Term (*) ["-"]
    //   Tier<FactorOp, Term> = Tier<FactorOp, Term> FactorOp Term (*) ["/"]
    //
    //   "+" -> Reduce(Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => Call(ActionFn(11));)
    //   "/" -> Reduce(Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => Call(ActionFn(11));)
    //   ")" -> Reduce(Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => Call(ActionFn(11));)
    //   "-" -> Reduce(Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => Call(ActionFn(11));)
    //   "*" -> Reduce(Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => Call(ActionFn(11));)
    //
    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
        __sym1: &mut Option<Opcode>,
        __sym2: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, _), _)) |
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action11(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Tier_3cFactorOp_2c_20Term_3e(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 37
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp Factor (*) [")"]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp Factor (*) ["+"]
    //   Tier<ExprOp, Factor> = Tier<ExprOp, Factor> ExprOp Factor (*) ["-"]
    //
    //   ")" -> Reduce(Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => Call(ActionFn(13));)
    //   "+" -> Reduce(Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => Call(ActionFn(13));)
    //   "-" -> Reduce(Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => Call(ActionFn(13));)
    //
    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
        __sym1: &mut Option<Opcode>,
        __sym2: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Tier_3cExprOp_2c_20Factor_3e(__nt)));
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
pub use self::__parse__Exprs::parse_Exprs;
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
                            __current_match = Some((7, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        ')' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '*' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        '+' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        ',' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '-' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        '/' => {
                            __current_match = Some((0, __index + 1));
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
                9 => {
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
    __0: Vec<Box<Expr>>,
) -> Vec<Box<Expr>>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: Vec<Box<Expr>>,
) -> Vec<Box<Expr>>
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
) -> Box<Expr>
{
    (__0)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
) -> Box<Expr>
{
    (__0)
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Opcode
{
    Opcode::Add
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Opcode
{
    Opcode::Sub
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Opcode
{
    Opcode::Mul
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Opcode
{
    Opcode::Div
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: i32,
) -> Box<Expr>
{
    Box::new(Expr::Number(__0))
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: Box<Expr>,
    _: &'input str,
) -> Box<Expr>
{
    (__0)
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> i32
{
    i32::from_str(__0).unwrap()
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
    __1: Opcode,
    __2: Box<Expr>,
) -> Box<Expr>
{
    Box::new(Expr::Op(__0, __1, __2))
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
) -> Box<Expr>
{
    (__0)
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
    __1: Opcode,
    __2: Box<Expr>,
) -> Box<Expr>
{
    Box::new(Expr::Op(__0, __1, __2))
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
) -> Box<Expr>
{
    (__0)
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<Box<Expr>>,
    e: ::std::option::Option<Box<Expr>>,
) -> Vec<Box<Expr>>
{
    match e {
        None=> v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
) -> ::std::option::Option<Box<Expr>>
{
    Some(__0)
}

pub fn __action17<
    'input,
>(
    input: &'input str,
) -> ::std::option::Option<Box<Expr>>
{
    None
}

pub fn __action18<
    'input,
>(
    input: &'input str,
) -> ::std::vec::Vec<Box<Expr>>
{
    vec![]
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<Box<Expr>>,
    e: Box<Expr>,
) -> ::std::vec::Vec<Box<Expr>>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
    _: &'input str,
) -> Box<Expr>
{
    (__0)
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
