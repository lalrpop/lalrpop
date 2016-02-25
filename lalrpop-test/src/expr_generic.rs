#![allow(unused_imports)]
#![allow(unused_variables)]
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::fmt::Debug;
    use std::ops::{Add, Div, Mul, Sub};
    use std::str::FromStr;
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Expr<
        'input,
        F,
    >(
        input: &'input str,
    ) -> Result<F, __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __lookbehind: usize = ::std::default::Default::default();
        match try!(__state0(input, __lookbehind, &mut __tokens, __lookahead)) {
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
    pub enum __Nonterminal<F> {
        Expr(F),
        Factor(F),
        Term(F),
        ____Expr(F),
    }

    pub fn __state0<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (6, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0));
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
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (4, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action0(input, __sym0, &__start, &__end);
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
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action3(input, __sym0, &__start, &__end);
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
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action6(input, __sym0, &__start, &__end);
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
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state5<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action7(input, __sym0, &__start, &__end);
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

    pub fn __state6<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
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

    pub fn __state7<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
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

    pub fn __state8<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym2));
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
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state9<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state10<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym1, __sym2));
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

    pub fn __state11<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action3(input, __sym0, &__start, &__end);
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

    pub fn __state12<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action6(input, __sym0, &__start, &__end);
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

    pub fn __state13<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state14<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
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
                let __sym0 = __sym0.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action7(input, __sym0, &__start, &__end);
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

    pub fn __state15<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action2(input, __sym0, __sym1, __sym2, &__start, &__end);
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

    pub fn __state16<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action1(input, __sym0, __sym1, __sym2, &__start, &__end);
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

    pub fn __state17<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action4(input, __sym0, __sym1, __sym2, &__start, &__end);
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

    pub fn __state18<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action5(input, __sym0, __sym1, __sym2, &__start, &__end);
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

    pub fn __state19<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<F>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action8(input, __sym0, __sym1, __sym2, &__start, &__end);
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

    pub fn __state20<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state21<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state22<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state23<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state24<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym1, __sym2));
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

    pub fn __state25<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action2(input, __sym0, __sym1, __sym2, &__start, &__end);
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

    pub fn __state26<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let __lookbehind = __loc;
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action1(input, __sym0, __sym1, __sym2, &__start, &__end);
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

    pub fn __state27<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action4(input, __sym0, __sym1, __sym2, &__start, &__end);
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

    pub fn __state28<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action5(input, __sym0, __sym1, __sym2, &__start, &__end);
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

    pub fn __state29<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: usize,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<F>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (usize, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
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
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __lookbehind.clone();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action8(input, __sym0, __sym1, __sym2, &__start, &__end);
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
    F,
>(
    input: &'input str,
    __0: F,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    (__0)
}

pub fn __action1<
    'input,
    F,
>(
    input: &'input str,
    l: F,
    _: &'input str,
    r: F,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    l - r
}

pub fn __action2<
    'input,
    F,
>(
    input: &'input str,
    l: F,
    _: &'input str,
    r: F,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    l + r
}

pub fn __action3<
    'input,
    F,
>(
    input: &'input str,
    __0: F,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    (__0)
}

pub fn __action4<
    'input,
    F,
>(
    input: &'input str,
    l: F,
    _: &'input str,
    r: F,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    l * r
}

pub fn __action5<
    'input,
    F,
>(
    input: &'input str,
    l: F,
    _: &'input str,
    r: F,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    l / r
}

pub fn __action6<
    'input,
    F,
>(
    input: &'input str,
    __0: F,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    (__0)
}

pub fn __action7<
    'input,
    F,
>(
    input: &'input str,
    n: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    match n.parse() { Ok(v) => v, Err(_) => panic!("can't parse") }
}

pub fn __action8<
    'input,
    F,
>(
    input: &'input str,
    _: &'input str,
    __0: F,
    _: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    (__0)
}

pub trait __ToTriple<'input, F, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, F, > __ToTriple<'input, F, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, F, > __ToTriple<'input, F, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
