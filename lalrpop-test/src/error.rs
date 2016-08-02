#![allow(unused_imports)]
use util::tok::Tok;
use lalrpop_util::ParseError;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Items {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    use lalrpop_util::ParseError;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_Items<
        __TOKEN: __ToTriple<Error=char>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Vec<(usize, usize)>, __ParseError<usize,Tok,char>> where
      __TOKENS: Clone,
    {
        let __ascent = __ascent::parse_Items(
            __tokens0.clone(),
        );
        let __parse_table = __parse_table::parse_Items(
            __tokens0.clone(),
        );
        assert_eq!(__ascent, __parse_table);
        return __ascent;
    }
    mod __ascent {

        mod __parse__Items {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use util::tok::Tok;
            use lalrpop_util::ParseError;
            extern crate lalrpop_util as __lalrpop_util;
            use self::__lalrpop_util::ParseError as __ParseError;
            use super::super::super::__ToTriple;
            pub fn parse_Items<
                __TOKEN: __ToTriple<Error=char>,
                __TOKENS: IntoIterator<Item=__TOKEN>,
            >(
                __tokens0: __TOKENS,
            ) -> Result<Vec<(usize, usize)>, __ParseError<usize,Tok,char>>
            {
                let __tokens = __tokens0.into_iter();
                let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__ParseError::User { error: e }),
                };
                match try!(__state0(&mut __tokens, __lookahead)) {
                    (Some(__lookahead), _) => {
                        Err(__ParseError::ExtraToken { token: __lookahead })
                    }
                    (None, __Nonterminal::____Items((_, __nt, _))) => {
                        Ok(__nt)
                    }
                    _ => unreachable!(),
                }
            }

            #[allow(dead_code)]
            pub enum __Nonterminal<> {
                Items((usize, Vec<(usize, usize)>, usize)),
                ____Items((usize, Vec<(usize, usize)>, usize)),
            }

            // State 0
            //     AllInputs = []
            //     OptionalInputs = []
            //     FixedInputs = []
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Items = (*) ["+"]
            //     Items = (*) ["-"]
            //     Items = (*) [EOF]
            //     Items = (*) Items "+" ["+"]
            //     Items = (*) Items "+" ["-"]
            //     Items = (*) Items "+" [EOF]
            //     Items = (*) Items "-" ["+"]
            //     Items = (*) Items "-" ["-"]
            //     Items = (*) Items "-" [EOF]
            //     __Items = (*) Items [EOF]
            //
            //   ["+"] -> Items =  => ActionFn(1);
            //   ["-"] -> Items =  => ActionFn(1);
            //   [EOF] -> Items =  => ActionFn(1);
            //
            //     Items -> S1
            pub fn __state0<
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __start: usize = ::std::default::Default::default();
                        let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action1(&__start, &__end);
                        let __nt = __Nonterminal::Items((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
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
                        __Nonterminal::Items(__sym0) => {
                            __result = try!(__state1(__tokens, __lookahead, __sym0));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 1
            //     AllInputs = [Items]
            //     OptionalInputs = []
            //     FixedInputs = [Items]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Items = Items (*) "+" ["+", "-", EOF]
            //     Items = Items (*) "-" ["+", "-", EOF]
            //     __Items = Items (*) [EOF]
            //
            //   "+" -> S2
            //   "-" -> S3
            //   [EOF] -> __Items = Items => ActionFn(0);
            //
            pub fn __state1<
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
            >(
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Tok, usize)>,
                __sym0: (usize, Vec<(usize, usize)>, usize),
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state2(__tokens, __sym0, __sym1));
                        return Ok(__result);
                    }
                    Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                        let __sym1 = (__loc1, (__tok), __loc2);
                        __result = try!(__state3(__tokens, __sym0, __sym1));
                        return Ok(__result);
                    }
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0(__sym0);
                        let __nt = __Nonterminal::____Items((
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
            //     AllInputs = [Items, "+"]
            //     OptionalInputs = []
            //     FixedInputs = [Items, "+"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Items)
            //
            //     Items = Items "+" (*) ["+", "-", EOF]
            //
            //   ["+", "-", EOF] -> Items = Items, "+" => ActionFn(2);
            //
            pub fn __state2<
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: (usize, Vec<(usize, usize)>, usize),
                __sym1: (usize, Tok, usize),
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = try!(super::super::super::__action2(__sym0, __sym1));
                        let __nt = __Nonterminal::Items((
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
            //     AllInputs = [Items, "-"]
            //     OptionalInputs = []
            //     FixedInputs = [Items, "-"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Items)
            //
            //     Items = Items "-" (*) ["+", "-", EOF]
            //
            //   ["+", "-", EOF] -> Items = Items, "-" => ActionFn(3);
            //
            pub fn __state3<
                __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
            >(
                __tokens: &mut __TOKENS,
                __sym0: (usize, Vec<(usize, usize)>, usize),
                __sym1: (usize, Tok, usize),
            ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
            {
                let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(__ParseError::User { error: e }),
                };
                match __lookahead {
                    Some((_, Tok::Plus, _)) |
                    Some((_, Tok::Minus, _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = try!(super::super::super::__action3(__sym0, __sym1));
                        let __nt = __Nonterminal::Items((
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
        pub use self::__parse__Items::parse_Items;
    }
    mod __parse_table {

        mod __parse__Items {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

            use util::tok::Tok;
            use lalrpop_util::ParseError;
            extern crate lalrpop_util as __lalrpop_util;
            use self::__lalrpop_util::ParseError as __ParseError;
            use super::super::super::__ToTriple;
            #[allow(dead_code)]
            pub enum __Symbol<> {
                Term_22_2b_22(Tok),
                Term_22_2d_22(Tok),
                NtItems(Vec<(usize, usize)>),
                Nt____Items(Vec<(usize, usize)>),
            }
            const __ACTION: &'static [i32] = &[
                // State 0
                //     Items = (*) ["+"]
                //     Items = (*) ["-"]
                //     Items = (*) [EOF]
                //     Items = (*) Items "+" ["+"]
                //     Items = (*) Items "+" ["-"]
                //     Items = (*) Items "+" [EOF]
                //     Items = (*) Items "-" ["+"]
                //     Items = (*) Items "-" ["-"]
                //     Items = (*) Items "-" [EOF]
                //     __Items = (*) Items [EOF]
                -1, // on "+", reduce `Items =  => ActionFn(1);`
                -1, // on "-", reduce `Items =  => ActionFn(1);`
                // State 1
                //     Items = Items (*) "+" ["+", "-", EOF]
                //     Items = Items (*) "-" ["+", "-", EOF]
                //     __Items = Items (*) [EOF]
                3, // on "+", goto 2
                4, // on "-", goto 3
                // State 2
                //     Items = Items "+" (*) ["+", "-", EOF]
                -2, // on "+", reduce `Items = Items, "+" => ActionFn(2);`
                -2, // on "-", reduce `Items = Items, "+" => ActionFn(2);`
                // State 3
                //     Items = Items "-" (*) ["+", "-", EOF]
                -3, // on "+", reduce `Items = Items, "-" => ActionFn(3);`
                -3, // on "-", reduce `Items = Items, "-" => ActionFn(3);`
            ];
            const __EOF_ACTION: &'static [i32] = &[
                -1, // on EOF, reduce `Items =  => ActionFn(1);`
                -4, // on EOF, reduce `__Items = Items => ActionFn(0);`
                -2, // on EOF, reduce `Items = Items, "+" => ActionFn(2);`
                -3, // on EOF, reduce `Items = Items, "-" => ActionFn(3);`
            ];
            const __GOTO: &'static [i32] = &[
                // State 0
                2, // on Items, goto 1
                0, // on __Items, error
                // State 1
                0, // on Items, error
                0, // on __Items, error
                // State 2
                0, // on Items, error
                0, // on __Items, error
                // State 3
                0, // on Items, error
                0, // on __Items, error
            ];
            pub fn parse_Items<
                __TOKEN: __ToTriple<Error=char>,
                __TOKENS: IntoIterator<Item=__TOKEN>,
            >(
                __tokens0: __TOKENS,
            ) -> Result<Vec<(usize, usize)>, __ParseError<usize,Tok,char>>
            {
                let __tokens = __tokens0.into_iter();
                let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
                let mut __states = vec![0_i32];
                let mut __symbols = vec![];
                '__shift: loop {
                    let __lookahead = match __tokens.next() {
                        Some(Ok(v)) => v,
                        None => break '__shift,
                        Some(Err(e)) => return Err(__ParseError::User { error: e }),
                    };
                    let __integer = match __lookahead {
                        (_, Tok::Plus, _) if true => 0,
                        (_, Tok::Minus, _) if true => 1,
                        _ => {
                            return Err(__ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: vec![],
                            });
                        }
                    };
                    loop {
                        let __state = *__states.last().unwrap() as usize;
                        let __action = __ACTION[__state * 2 + __integer];
                        if __action > 0 {
                            let __symbol = match __integer {
                                0 => match __lookahead.1 {
                                    __tok @ Tok::Plus => __Symbol::Term_22_2b_22(__tok),
                                    _ => unreachable!(),
                                },
                                1 => match __lookahead.1 {
                                    __tok @ Tok::Minus => __Symbol::Term_22_2d_22(__tok),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            __states.push(__action - 1);
                            __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                            continue '__shift;
                        } else if __action < 0 {
                            if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                                return r;
                            }
                        } else {
                            return Err(__ParseError::UnrecognizedToken {
                                token: Some(__lookahead),
                                expected: vec![],
                            });
                        }
                    }
                }
                loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __EOF_ACTION[__state];
                    if __action < 0 {
                        if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols) {
                            return r;
                        }
                    } else {
                        return Err(__ParseError::UnrecognizedToken {
                            token: None,
                            expected: vec![],
                        });
                    }
                }
            }
            pub fn __reduce<
            >(
                __action: i32,
                __lookahead_start: Option<&usize>,
                __states: &mut ::std::vec::Vec<i32>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
            ) -> Option<Result<Vec<(usize, usize)>,__ParseError<usize,Tok,char>>>
            {
                let __nonterminal = match -__action {
                    1 => {
                        // Items =  => ActionFn(1);
                        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                        let __nt = super::super::super::__action1(&__start, &__end);
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 0);
                        __symbols.push((__start, __Symbol::NtItems(__nt), __end));
                        0
                    }
                    2 => {
                        // Items = Items, "+" => ActionFn(2);
                        let __sym1 = __pop_Term_22_2b_22(__symbols);
                        let __sym0 = __pop_NtItems(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = match super::super::super::__action2(__sym0, __sym1) {
                            Ok(v) => v,
                            Err(e) => return Some(Err(e)),
                        };
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 2);
                        __symbols.push((__start, __Symbol::NtItems(__nt), __end));
                        0
                    }
                    3 => {
                        // Items = Items, "-" => ActionFn(3);
                        let __sym1 = __pop_Term_22_2d_22(__symbols);
                        let __sym0 = __pop_NtItems(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym1.2.clone();
                        let __nt = match super::super::super::__action3(__sym0, __sym1) {
                            Ok(v) => v,
                            Err(e) => return Some(Err(e)),
                        };
                        let __states_len = __states.len();
                        __states.truncate(__states_len - 2);
                        __symbols.push((__start, __Symbol::NtItems(__nt), __end));
                        0
                    }
                    4 => {
                        // __Items = Items => ActionFn(0);
                        let __sym0 = __pop_NtItems(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0(__sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 2 + __nonterminal] - 1;
                __states.push(__next_state);
                None
            }
            fn __pop_Term_22_2b_22<
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
            ) -> (usize, Tok, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Term_22_2d_22<
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
            ) -> (usize, Tok, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_NtItems<
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
            ) -> (usize, Vec<(usize, usize)>, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::NtItems(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Nt____Items<
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
            ) -> (usize, Vec<(usize, usize)>, usize) {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Nt____Items(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
        }
        pub use self::__parse__Items::parse_Items;
    }
}
pub use self::__parse__Items::parse_Items;

pub fn __action0<
>(
    (_, __0, _): (usize, Vec<(usize, usize)>, usize),
) -> Vec<(usize, usize)>
{
    (__0)
}

pub fn __action1<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(usize, usize)>
{
    vec![]
}

pub fn __action2<
>(
    (_, __0, _): (usize, Vec<(usize, usize)>, usize),
    (_, __1, _): (usize, Tok, usize),
) -> Result<Vec<(usize, usize)>,__ParseError<usize,Tok,char>>
{
    Err(ParseError::User { error: '+' })
}

pub fn __action3<
>(
    (_, v, _): (usize, Vec<(usize, usize)>, usize),
    (_, _, _): (usize, Tok, usize),
) -> Result<Vec<(usize, usize)>,__ParseError<usize,Tok,char>>
{
    Ok(v)
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),Self::Error>;
}

impl<> __ToTriple<> for (usize, Tok, usize) {
    type Error = char;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),char> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, Tok, usize),char> {
    type Error = char;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),char> {
        value
    }
}
