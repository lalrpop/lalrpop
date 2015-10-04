#![allow(unused_imports)]
#![allow(unused_variables)]
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Items {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Items<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<(usize, usize)>, __ParseError<usize,(usize, &'input str),()>>
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
            (_, None, __Nonterminal::____Items(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        _40L(usize),
        _40R(usize),
        Items(Vec<(usize, usize)>),
        Spanned_3c_22_2b_22_3e((usize, usize)),
        ____Items(Vec<(usize, usize)>),
    }

    // State 0
    //   @L = (*) [EOF]
    //   @L = (*) ["+"]
    //   @L = (*) ["-"]
    //   Items = (*) @L @R [EOF]
    //   Items = (*) @L @R ["+"]
    //   Items = (*) @L @R ["-"]
    //   Items = (*) Items Spanned<"+"> [EOF]
    //   Items = (*) Items Spanned<"+"> ["+"]
    //   Items = (*) Items Spanned<"+"> ["-"]
    //   Items = (*) Items "-" [EOF]
    //   Items = (*) Items "-" ["+"]
    //   Items = (*) Items "-" ["-"]
    //   __Items = (*) Items [EOF]
    //
    //   EOF -> Reduce(@L =  => Lookahead;)
    //   "+" -> Reduce(@L =  => Lookahead;)
    //   "-" -> Reduce(@L =  => Lookahead;)
    //
    //   @L -> S1
    //   Items -> S2
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
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) => {
                let __nt = __lookahead.as_ref().map(|o| ::std::clone::Clone::clone(&o.0)).or_else(|| ::std::clone::Clone::clone(&__lookbehind)).unwrap_or_default();
                __result = (__lookbehind, __lookahead, __Nonterminal::_40L(__nt));
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
                __Nonterminal::_40L(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Items(__nt) => {
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
    //   @R = (*) [EOF]
    //   @R = (*) ["+"]
    //   @R = (*) ["-"]
    //   Items = @L (*) @R [EOF]
    //   Items = @L (*) @R ["+"]
    //   Items = @L (*) @R ["-"]
    //
    //   EOF -> Reduce(@R =  => Lookbehind;)
    //   "+" -> Reduce(@R =  => Lookbehind;)
    //   "-" -> Reduce(@R =  => Lookbehind;)
    //
    //   @R -> S3
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<usize>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) => {
                let __nt = ::std::clone::Clone::clone(&__lookbehind).unwrap_or_default();
                __result = (__lookbehind, __lookahead, __Nonterminal::_40R(__nt));
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
                __Nonterminal::_40R(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 2
    //   @L = (*) ["+"]
    //   Items = Items (*) Spanned<"+"> [EOF]
    //   Items = Items (*) Spanned<"+"> ["+"]
    //   Items = Items (*) Spanned<"+"> ["-"]
    //   Items = Items (*) "-" [EOF]
    //   Items = Items (*) "-" ["+"]
    //   Items = Items (*) "-" ["-"]
    //   Spanned<"+"> = (*) @L "+" @R [EOF]
    //   Spanned<"+"> = (*) @L "+" @R ["+"]
    //   Spanned<"+"> = (*) @L "+" @R ["-"]
    //   __Items = Items (*) [EOF]
    //
    //   EOF -> Reduce(__Items = Items => Call(ActionFn(0));)
    //   "+" -> Reduce(@L =  => Lookahead;)
    //   "-" -> Shift(S6)
    //
    //   @L -> S4
    //   Spanned<"+"> -> S5
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Vec<(usize, usize)>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (0, _), _)) => {
                let __nt = __lookahead.as_ref().map(|o| ::std::clone::Clone::clone(&o.0)).or_else(|| ::std::clone::Clone::clone(&__lookbehind)).unwrap_or_default();
                __result = (__lookbehind, __lookahead, __Nonterminal::_40L(__nt));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Items(__nt)));
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
                __Nonterminal::_40L(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Spanned_3c_22_2b_22_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 3
    //   Items = @L @R (*) [EOF]
    //   Items = @L @R (*) ["+"]
    //   Items = @L @R (*) ["-"]
    //
    //   EOF -> Reduce(Items = @L, @R => Call(ActionFn(1));)
    //   "+" -> Reduce(Items = @L, @R => Call(ActionFn(1));)
    //   "-" -> Reduce(Items = @L, @R => Call(ActionFn(1));)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<usize>,
        __sym1: &mut Option<usize>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action1(input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Items(__nt)));
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
    //   Spanned<"+"> = @L (*) "+" @R [EOF]
    //   Spanned<"+"> = @L (*) "+" @R ["+"]
    //   Spanned<"+"> = @L (*) "+" @R ["-"]
    //
    //   "+" -> Shift(S7)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<usize>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym0, __sym1));
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
    //   Items = Items Spanned<"+"> (*) [EOF]
    //   Items = Items Spanned<"+"> (*) ["+"]
    //   Items = Items Spanned<"+"> (*) ["-"]
    //
    //   EOF -> Reduce(Items = Items, Spanned<"+"> => Call(ActionFn(2));)
    //   "+" -> Reduce(Items = Items, Spanned<"+"> => Call(ActionFn(2));)
    //   "-" -> Reduce(Items = Items, Spanned<"+"> => Call(ActionFn(2));)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Vec<(usize, usize)>>,
        __sym1: &mut Option<(usize, usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action2(input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Items(__nt)));
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
    //   Items = Items "-" (*) [EOF]
    //   Items = Items "-" (*) ["+"]
    //   Items = Items "-" (*) ["-"]
    //
    //   EOF -> Reduce(Items = Items, "-" => Call(ActionFn(3));)
    //   "+" -> Reduce(Items = Items, "-" => Call(ActionFn(3));)
    //   "-" -> Reduce(Items = Items, "-" => Call(ActionFn(3));)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Vec<(usize, usize)>>,
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
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action3(input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Items(__nt)));
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
    //   @R = (*) [EOF]
    //   @R = (*) ["+"]
    //   @R = (*) ["-"]
    //   Spanned<"+"> = @L "+" (*) @R [EOF]
    //   Spanned<"+"> = @L "+" (*) @R ["+"]
    //   Spanned<"+"> = @L "+" (*) @R ["-"]
    //
    //   EOF -> Reduce(@R =  => Lookbehind;)
    //   "+" -> Reduce(@R =  => Lookbehind;)
    //   "-" -> Reduce(@R =  => Lookbehind;)
    //
    //   @R -> S8
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<usize>,
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
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) => {
                let __nt = ::std::clone::Clone::clone(&__lookbehind).unwrap_or_default();
                __result = (__lookbehind, __lookahead, __Nonterminal::_40R(__nt));
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
                __Nonterminal::_40R(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 8
    //   Spanned<"+"> = @L "+" @R (*) [EOF]
    //   Spanned<"+"> = @L "+" @R (*) ["+"]
    //   Spanned<"+"> = @L "+" @R (*) ["-"]
    //
    //   EOF -> Reduce(Spanned<"+"> = @L, "+", @R => Call(ActionFn(4));)
    //   "+" -> Reduce(Spanned<"+"> = @L, "+", @R => Call(ActionFn(4));)
    //   "-" -> Reduce(Spanned<"+"> = @L, "+", @R => Call(ActionFn(4));)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<usize>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<usize>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Spanned_3c_22_2b_22_3e(__nt)));
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
                        '+' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '-' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
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
    __0: Vec<(usize, usize)>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<(usize, usize)>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: usize,
    __1: usize,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<(usize, usize)>
{
    vec![(__0, __1)]
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    v: Vec<(usize, usize)>,
    e: (usize, usize),
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<(usize, usize)>
{
    {
        let mut v = v;
        v.push(e);
        v
    }
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    v: Vec<(usize, usize)>,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<(usize, usize)>
{
    v
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    __0: usize,
    _: &'input str,
    __1: usize,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> (usize, usize)
{
    (__0, __1)
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
