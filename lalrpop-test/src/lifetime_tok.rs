#![allow(unused_imports)]
use lifetime_tok_lib::LtTok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use lifetime_tok_lib::LtTok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        TermOther(&'input str),
        NtExpr(Vec<&'input str>),
        NtOther_2a(::std::vec::Vec<&'input str>),
        NtOther_2b(::std::vec::Vec<&'input str>),
        Nt____Expr(Vec<&'input str>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        //     Expr = (*) [EOF]
        //     Expr = (*) Other+ [EOF]
        //     Other+ = (*) Other+ Other [Other]
        //     Other+ = (*) Other+ Other [EOF]
        //     Other+ = (*) Other [Other]
        //     Other+ = (*) Other [EOF]
        //     __Expr = (*) Expr [EOF]
        4, // on Other, goto 3
        // State 1
        //     __Expr = Expr (*) [EOF]
        0, // on Other, error
        // State 2
        //     Expr = Other+ (*) [EOF]
        //     Other+ = Other+ (*) Other [Other, EOF]
        5, // on Other, goto 4
        // State 3
        //     Other+ = Other (*) [Other, EOF]
        -5, // on Other, reduce `Other+ = Other => ActionFn(4);`
        // State 4
        //     Other+ = Other+ Other (*) [Other, EOF]
        -6, // on Other, reduce `Other+ = Other+, Other => ActionFn(5);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -1, // on EOF, reduce `Expr =  => ActionFn(6);`
        -7, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
        -2, // on EOF, reduce `Expr = Other+ => ActionFn(7);`
        -5, // on EOF, reduce `Other+ = Other => ActionFn(4);`
        -6, // on EOF, reduce `Other+ = Other+, Other => ActionFn(5);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Expr, goto 1
        0, // on Other*, error
        3, // on Other+, goto 2
        0, // on __Expr, error
        // State 1
        0, // on Expr, error
        0, // on Other*, error
        0, // on Other+, error
        0, // on __Expr, error
        // State 2
        0, // on Expr, error
        0, // on Other*, error
        0, // on Other+, error
        0, // on __Expr, error
        // State 3
        0, // on Expr, error
        0, // on Other*, error
        0, // on Other+, error
        0, // on __Expr, error
        // State 4
        0, // on Expr, error
        0, // on Other*, error
        0, // on Other+, error
        0, // on __Expr, error
    ];
    pub fn parse_Expr<
        'input,
        __TOKEN: __ToTriple<'input, Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<Vec<&'input str>, __ParseError<(),LtTok<'input>,()>>
    {
        let __tokens = __tokens.into_iter();
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
                (_, LtTok::Other(_), _) if true => 0,
                _ => {
                    return Err(__ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 1 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            LtTok::Other(__tok0) => __Symbol::TermOther(__tok0),
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
        'input,
    >(
        __action: i32,
        __lookahead_start: Option<&()>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>,
    ) -> Option<Result<Vec<&'input str>,__ParseError<(),LtTok<'input>,()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Expr =  => ActionFn(6);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action6(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            2 => {
                // Expr = Other+ => ActionFn(7);
                let __sym0 = __pop_NtOther_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            3 => {
                // Other* =  => ActionFn(2);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action2(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtOther_2a(__nt), __end));
                1
            }
            4 => {
                // Other* = Other+ => ActionFn(3);
                let __sym0 = __pop_NtOther_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOther_2a(__nt), __end));
                1
            }
            5 => {
                // Other+ = Other => ActionFn(4);
                let __sym0 = __pop_TermOther(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOther_2b(__nt), __end));
                2
            }
            6 => {
                // Other+ = Other+, Other => ActionFn(5);
                let __sym1 = __pop_TermOther(__symbols);
                let __sym0 = __pop_NtOther_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOther_2b(__nt), __end));
                2
            }
            7 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 4 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_TermOther<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), &'input str, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermOther(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Vec<&'input str>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOther_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), ::std::vec::Vec<&'input str>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOther_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOther_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), ::std::vec::Vec<&'input str>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOther_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Vec<&'input str>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Expr::parse_Expr;

pub fn __action0<
    'input,
>(
    (_, __0, _): ((), Vec<&'input str>, ()),
) -> Vec<&'input str>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    (_, __0, _): ((), ::std::vec::Vec<&'input str>, ()),
) -> Vec<&'input str>
{
    (__0)
}

pub fn __action2<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> ::std::vec::Vec<&'input str>
{
    vec![]
}

pub fn __action3<
    'input,
>(
    (_, v, _): ((), ::std::vec::Vec<&'input str>, ()),
) -> ::std::vec::Vec<&'input str>
{
    v
}

pub fn __action4<
    'input,
>(
    (_, __0, _): ((), &'input str, ()),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

pub fn __action5<
    'input,
>(
    (_, v, _): ((), ::std::vec::Vec<&'input str>, ()),
    (_, e, _): ((), &'input str, ()),
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action6<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> Vec<&'input str>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action2(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

pub fn __action7<
    'input,
>(
    __0: ((), ::std::vec::Vec<&'input str>, ()),
) -> Vec<&'input str>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action3(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<((),LtTok<'input>,()),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for LtTok<'input> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),LtTok<'input>,()),()> {
        Ok(((), value, ()))
    }
}
impl<'input, > __ToTriple<'input, > for Result<(LtTok<'input>),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),LtTok<'input>,()),()> {
        value.map(|v| ((), v, ()))
    }
}
