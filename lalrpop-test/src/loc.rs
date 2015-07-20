#![allow(unused_imports)]
use util::tok::Tok;

#[allow(non_snake_case)]
pub fn parse_Items<
    __TOKENS: IntoIterator<Item=(usize,Tok,usize)>,
>(
    __tokens: __TOKENS,
) -> Result<(Option<(usize,Tok,usize)>, Vec<(Option<usize>, Option<usize>)>), Option<(usize,Tok,usize)>>
{
    let mut __tokens = __tokens.into_iter();
    let __lookahead = __tokens.next();
    match try!(__parse__Items::__state0(None, __lookahead, &mut __tokens)) {
        (_, __lookahead, __parse__Items::__Nonterminal::____Items(__nt)) => Ok((__lookahead, __nt)),
        _ => unreachable!(),
    }
}

mod __parse__Items {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;

    pub enum __Nonterminal<> {
        Items(Vec<(Option<usize>, Option<usize>)>),
        _40_3e(::std::option::Option<usize>),
        _40_3c(::std::option::Option<usize>),
        ____Items(Vec<(Option<usize>, Option<usize>)>),
        Spanned_3c_22_2b_22_3e((Option<usize>, Option<usize>)),
    }

    // State 0
    //   Items = (*) [EOF]
    //   Items = (*) ["+"]
    //   Items = (*) ["-"]
    //   Items = (*) Items Spanned<"+"> [EOF]
    //   Items = (*) Items Spanned<"+"> ["+"]
    //   Items = (*) Items Spanned<"+"> ["-"]
    //   Items = (*) Items "-" [EOF]
    //   Items = (*) Items "-" ["+"]
    //   Items = (*) Items "-" ["-"]
    //   __Items = (*) Items [EOF]
    //
    //   "-" -> Reduce(Items =  => Call(ActionFn(1));)
    //   "+" -> Reduce(Items =  => Call(ActionFn(1));)
    //   EOF -> Reduce(Items =  => Call(ActionFn(1));)
    //
    //   Items -> S1
    pub fn __state0<
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) => {
                let __nt = super::__action1();
                __result = (__lookbehind, __lookahead, __Nonterminal::Items(__nt));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __nt = super::__action1();
                __result = (__lookbehind, __lookahead, __Nonterminal::Items(__nt));
            }
            None => {
                let __nt = super::__action1();
                __result = (__lookbehind, __lookahead, __Nonterminal::Items(__nt));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Items(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__lookbehind, __lookahead, __tokens, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   @< = (*) ["+"]
    //   Items = Items (*) Spanned<"+"> [EOF]
    //   Items = Items (*) Spanned<"+"> ["+"]
    //   Items = Items (*) Spanned<"+"> ["-"]
    //   Items = Items (*) "-" [EOF]
    //   Items = Items (*) "-" ["+"]
    //   Items = Items (*) "-" ["-"]
    //   Spanned<"+"> = (*) @< "+" @> [EOF]
    //   Spanned<"+"> = (*) @< "+" @> ["+"]
    //   Spanned<"+"> = (*) @< "+" @> ["-"]
    //   __Items = Items (*) [EOF]
    //
    //   "+" -> Reduce(@< =  => Lookahead;)
    //   EOF -> Reduce(__Items = Items => Call(ActionFn(0));)
    //   "-" -> Shift(S2)
    //
    //   Spanned<"+"> -> S4
    //   @< -> S3
    pub fn __state1<
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Vec<(Option<usize>, Option<usize>)>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state2(__lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __nt = __lookahead.as_ref().map(|o| ::std::clone::Clone::clone(&o.0));
                __result = (__lookbehind, __lookahead, __Nonterminal::_40_3c(__nt));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(__sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Items(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Spanned_3c_22_2b_22_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(__lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::_40_3c(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state3(__lookbehind, __lookahead, __tokens, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 2
    //   Items = Items "-" (*) [EOF]
    //   Items = Items "-" (*) ["+"]
    //   Items = Items "-" (*) ["-"]
    //
    //   "-" -> Reduce(Items = Items, "-" => Call(ActionFn(3));)
    //   "+" -> Reduce(Items = Items, "-" => Call(ActionFn(3));)
    //   EOF -> Reduce(Items = Items, "-" => Call(ActionFn(3));)
    //
    pub fn __state2<
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Vec<(Option<usize>, Option<usize>)>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action3(__sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Items(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action3(__sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Items(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action3(__sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Items(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 3
    //   Spanned<"+"> = @< (*) "+" @> [EOF]
    //   Spanned<"+"> = @< (*) "+" @> ["+"]
    //   Spanned<"+"> = @< (*) "+" @> ["-"]
    //
    //   "+" -> Shift(S5)
    //
    pub fn __state3<
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::option::Option<usize>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state5(__lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 4
    //   Items = Items Spanned<"+"> (*) [EOF]
    //   Items = Items Spanned<"+"> (*) ["+"]
    //   Items = Items Spanned<"+"> (*) ["-"]
    //
    //   EOF -> Reduce(Items = Items, Spanned<"+"> => Call(ActionFn(2));)
    //   "-" -> Reduce(Items = Items, Spanned<"+"> => Call(ActionFn(2));)
    //   "+" -> Reduce(Items = Items, Spanned<"+"> => Call(ActionFn(2));)
    //
    pub fn __state4<
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Vec<(Option<usize>, Option<usize>)>>,
        __sym1: &mut Option<(Option<usize>, Option<usize>)>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action2(__sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Items(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action2(__sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Items(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action2(__sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Items(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 5
    //   @> = (*) [EOF]
    //   @> = (*) ["+"]
    //   @> = (*) ["-"]
    //   Spanned<"+"> = @< "+" (*) @> [EOF]
    //   Spanned<"+"> = @< "+" (*) @> ["+"]
    //   Spanned<"+"> = @< "+" (*) @> ["-"]
    //
    //   EOF -> Reduce(@> =  => Lookbehind;)
    //   "+" -> Reduce(@> =  => Lookbehind;)
    //   "-" -> Reduce(@> =  => Lookbehind;)
    //
    //   @> -> S6
    pub fn __state5<
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::option::Option<usize>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __nt = ::std::clone::Clone::clone(&__lookbehind);
                __result = (__lookbehind, __lookahead, __Nonterminal::_40_3e(__nt));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __nt = ::std::clone::Clone::clone(&__lookbehind);
                __result = (__lookbehind, __lookahead, __Nonterminal::_40_3e(__nt));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __nt = ::std::clone::Clone::clone(&__lookbehind);
                __result = (__lookbehind, __lookahead, __Nonterminal::_40_3e(__nt));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_40_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(__lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 6
    //   Spanned<"+"> = @< "+" @> (*) [EOF]
    //   Spanned<"+"> = @< "+" @> (*) ["+"]
    //   Spanned<"+"> = @< "+" @> (*) ["-"]
    //
    //   "-" -> Reduce(Spanned<"+"> = @<, "+", @> => Call(ActionFn(4));)
    //   "+" -> Reduce(Spanned<"+"> = @<, "+", @> => Call(ActionFn(4));)
    //   EOF -> Reduce(Spanned<"+"> = @<, "+", @> => Call(ActionFn(4));)
    //
    pub fn __state6<
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::option::Option<usize>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<::std::option::Option<usize>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(__sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Spanned_3c_22_2b_22_3e(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(__sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Spanned_3c_22_2b_22_3e(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(__sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Spanned_3c_22_2b_22_3e(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }
}

pub fn __action0<
>(
    __0: Vec<(Option<usize>, Option<usize>)>,
) -> Vec<(Option<usize>, Option<usize>)>
{
    (__0)
}

pub fn __action1<
>(
) -> Vec<(Option<usize>, Option<usize>)>
{
    vec![]
}

pub fn __action2<
>(
    v: Vec<(Option<usize>, Option<usize>)>,
    e: (Option<usize>, Option<usize>),
) -> Vec<(Option<usize>, Option<usize>)>
{
    {
        let mut v = v;
        v.push(e);
        v
    }
}

pub fn __action3<
>(
    v: Vec<(Option<usize>, Option<usize>)>,
    _: Tok,
) -> Vec<(Option<usize>, Option<usize>)>
{
    v
}

pub fn __action4<
>(
    __0: ::std::option::Option<usize>,
    _: Tok,
    __1: ::std::option::Option<usize>,
) -> (Option<usize>, Option<usize>)
{
    (__0, __1)
}
