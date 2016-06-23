#![allow(unused_imports)]
use super::util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__S {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use super::super::util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        Term_22_28_22(Tok),
        Term_22_29_22(Tok),
        NtS(i32),
        Nt____S(i32),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        //     S = (*) "(" ")" [EOF]
        //     __S = (*) S [EOF]
        3, // on "(", goto 2
        0, // on ")", error
        // State 1
        //     __S = S (*) [EOF]
        0, // on "(", error
        0, // on ")", error
        // State 2
        //     S = "(" (*) ")" [EOF]
        0, // on "(", error
        4, // on ")", goto 3
        // State 3
        //     S = "(" ")" (*) [EOF]
        0, // on "(", error
        0, // on ")", error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -2, // on EOF, reduce `__S = S => ActionFn(0);`
        0, // on EOF, error
        -1, // on EOF, reduce `S = "(", ")" => ActionFn(1);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on S, goto 1
        0, // on __S, error
        // State 1
        0, // on S, error
        0, // on __S, error
        // State 2
        0, // on S, error
        0, // on __S, error
        // State 3
        0, // on S, error
        0, // on __S, error
    ];
    pub fn parse_S<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<i32, __ParseError<(),Tok,()>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            println!("outer loop");
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__ParseError::User { error: e }),
            };
            let __integer = match __lookahead {
                (_, Tok::LParen, _) if true => 0,
                (_, Tok::RParen, _) if true => 1,
                _ => {
                    return Err(__ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                println!("inner loop");
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 2 + __integer];
                println!("state: {} lookahead: {} action: {} stack-depth: {}", __state, __integer, __action, __symbols.len());
                if __action > 0 {
                    println!("--> shift");
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Tok::LParen => __Symbol::Term_22_28_22(__tok),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Tok::RParen => __Symbol::Term_22_29_22(__tok),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    println!("--> reduce");
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
            println!("EOF loop state: {}", __state);
            let __action = __EOF_ACTION[__state];
            println!("EOF in state {} takes action {}", __state, __action);
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
        __lookahead_start: Option<&()>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
    ) -> Option<Result<i32,__ParseError<(),Tok,()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // S = "(", ")" => ActionFn(1);
                let __sym1 = __pop_Term_22_29_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action1(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtS(__nt), __end));
                0
            }
            2 => {
                // __S = S => ActionFn(0);
                let __sym0 = __pop_NtS(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 2 + __nonterminal] - 1;
        println!("goto state {} from {} due to nonterminal {}", __next_state, __state, __nonterminal);
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Tok, ()) {
        println!("pop_Term_22_28_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Tok, ()) {
        println!("pop_Term_22_29_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtS<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), i32, ()) {
        println!("pop_NtS");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____S<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), i32, ()) {
        println!("pop_Nt____S");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____S(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__S::parse_S;

pub fn __action0<
>(
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

pub fn __action1<
>(
    (_, __0, _): ((), Tok, ()),
    (_, __1, _): ((), Tok, ()),
) -> i32
{
    super::ZERO
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<((),Tok,()),Self::Error>;
}

impl<> __ToTriple<> for Tok {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Tok,()),()> {
        Ok(((), value, ()))
    }
}
impl<> __ToTriple<> for Result<(Tok),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Tok,()),()> {
        value.map(|v| ((), v, ()))
    }
}
