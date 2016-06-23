#![allow(unused_imports)]
#![allow(unused_variables)]
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        Term_22_28_22(Tok),
        Term_22_29_22(Tok),
        Term_22_2a_22(Tok),
        Term_22_2b_22(Tok),
        Term_22_2d_22(Tok),
        Term_22_2f_22(Tok),
        TermNum(i32),
        NtExpr(i32),
        NtFactor(i32),
        NtTerm(i32),
        Nt____Expr(i32),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        //     Expr = (*) Expr "+" Factor ["+", "-", EOF]
        //     Expr = (*) Expr "-" Factor ["+", "-", EOF]
        //     Expr = (*) Factor ["+", "-", EOF]
        //     Factor = (*) Factor "*" Term ["*", "+", "-", "/", EOF]
        //     Factor = (*) Factor "/" Term ["*", "+", "-", "/", EOF]
        //     Factor = (*) Term ["*", "+", "-", "/", EOF]
        //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
        //     Term = (*) Num ["*", "+", "-", "/", EOF]
        //     __Expr = (*) Expr [EOF]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        6, // on Num, goto 5
        // State 1
        //     Expr = Expr (*) "+" Factor ["+", "-", EOF]
        //     Expr = Expr (*) "-" Factor ["+", "-", EOF]
        //     __Expr = Expr (*) [EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        7, // on "+", goto 6
        8, // on "-", goto 7
        0, // on "/", error
        0, // on Num, error
        // State 2
        //     Expr = Factor (*) [")", "+", "-", EOF]
        //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/", EOF]
        //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/", EOF]
        0, // on "(", error
        -3, // on ")", reduce `Expr = Factor => ActionFn(3);`
        9, // on "*", goto 8
        -3, // on "+", reduce `Expr = Factor => ActionFn(3);`
        -3, // on "-", reduce `Expr = Factor => ActionFn(3);`
        10, // on "/", goto 9
        0, // on Num, error
        // State 3
        //     Factor = Term (*) [")", "*", "+", "-", "/", EOF]
        0, // on "(", error
        -6, // on ")", reduce `Factor = Term => ActionFn(6);`
        -6, // on "*", reduce `Factor = Term => ActionFn(6);`
        -6, // on "+", reduce `Factor = Term => ActionFn(6);`
        -6, // on "-", reduce `Factor = Term => ActionFn(6);`
        -6, // on "/", reduce `Factor = Term => ActionFn(6);`
        0, // on Num, error
        // State 4
        //     Expr = (*) Expr "+" Factor [")", "+", "-"]
        //     Expr = (*) Expr "-" Factor [")", "+", "-"]
        //     Expr = (*) Factor [")", "+", "-"]
        //     Factor = (*) Factor "*" Term [")", "*", "+", "-", "/"]
        //     Factor = (*) Factor "/" Term [")", "*", "+", "-", "/"]
        //     Factor = (*) Term [")", "*", "+", "-", "/"]
        //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
        //     Term = "(" (*) Expr ")" [")", "*", "+", "-", "/", EOF]
        //     Term = (*) Num [")", "*", "+", "-", "/"]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        6, // on Num, goto 5
        // State 5
        //     Term = Num (*) [")", "*", "+", "-", "/", EOF]
        0, // on "(", error
        -7, // on ")", reduce `Term = Num => ActionFn(7);`
        -7, // on "*", reduce `Term = Num => ActionFn(7);`
        -7, // on "+", reduce `Term = Num => ActionFn(7);`
        -7, // on "-", reduce `Term = Num => ActionFn(7);`
        -7, // on "/", reduce `Term = Num => ActionFn(7);`
        0, // on Num, error
        // State 6
        //     Expr = Expr "+" (*) Factor [")", "+", "-", EOF]
        //     Factor = (*) Factor "*" Term [")", "*", "+", "-", "/", EOF]
        //     Factor = (*) Factor "/" Term [")", "*", "+", "-", "/", EOF]
        //     Factor = (*) Term [")", "*", "+", "-", "/", EOF]
        //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/", EOF]
        //     Term = (*) Num [")", "*", "+", "-", "/", EOF]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        6, // on Num, goto 5
        // State 7
        //     Expr = Expr "-" (*) Factor [")", "+", "-", EOF]
        //     Factor = (*) Factor "*" Term [")", "*", "+", "-", "/", EOF]
        //     Factor = (*) Factor "/" Term [")", "*", "+", "-", "/", EOF]
        //     Factor = (*) Term [")", "*", "+", "-", "/", EOF]
        //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/", EOF]
        //     Term = (*) Num [")", "*", "+", "-", "/", EOF]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        6, // on Num, goto 5
        // State 8
        //     Factor = Factor "*" (*) Term [")", "*", "+", "-", "/", EOF]
        //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/", EOF]
        //     Term = (*) Num [")", "*", "+", "-", "/", EOF]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        6, // on Num, goto 5
        // State 9
        //     Factor = Factor "/" (*) Term [")", "*", "+", "-", "/", EOF]
        //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/", EOF]
        //     Term = (*) Num [")", "*", "+", "-", "/", EOF]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        6, // on Num, goto 5
        // State 10
        //     Expr = Expr (*) "+" Factor [")", "+", "-"]
        //     Expr = Expr (*) "-" Factor [")", "+", "-"]
        //     Term = "(" Expr (*) ")" [")", "*", "+", "-", "/", EOF]
        0, // on "(", error
        16, // on ")", goto 15
        0, // on "*", error
        7, // on "+", goto 6
        8, // on "-", goto 7
        0, // on "/", error
        0, // on Num, error
        // State 11
        //     Expr = Expr "+" Factor (*) [")", "+", "-", EOF]
        //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/", EOF]
        //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/", EOF]
        0, // on "(", error
        -2, // on ")", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        9, // on "*", goto 8
        -2, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        -2, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        10, // on "/", goto 9
        0, // on Num, error
        // State 12
        //     Expr = Expr "-" Factor (*) [")", "+", "-", EOF]
        //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/", EOF]
        //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/", EOF]
        0, // on "(", error
        -1, // on ")", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        9, // on "*", goto 8
        -1, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        -1, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        10, // on "/", goto 9
        0, // on Num, error
        // State 13
        //     Factor = Factor "*" Term (*) [")", "*", "+", "-", "/", EOF]
        0, // on "(", error
        -4, // on ")", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -4, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -4, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -4, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -4, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        0, // on Num, error
        // State 14
        //     Factor = Factor "/" Term (*) [")", "*", "+", "-", "/", EOF]
        0, // on "(", error
        -5, // on ")", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -5, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -5, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -5, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -5, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        0, // on Num, error
        // State 15
        //     Term = "(" Expr ")" (*) [")", "*", "+", "-", "/", EOF]
        0, // on "(", error
        -8, // on ")", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        -8, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        -8, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        -8, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        -8, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        0, // on Num, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -9, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
        -3, // on EOF, reduce `Expr = Factor => ActionFn(3);`
        -6, // on EOF, reduce `Factor = Term => ActionFn(6);`
        0, // on EOF, error
        -7, // on EOF, reduce `Term = Num => ActionFn(7);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -2, // on EOF, reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        -1, // on EOF, reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        -4, // on EOF, reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -5, // on EOF, reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -8, // on EOF, reduce `Term = "(", Expr, ")" => ActionFn(8);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Expr, goto 1
        3, // on Factor, goto 2
        4, // on Term, goto 3
        0, // on __Expr, error
        // State 1
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 2
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 3
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 4
        11, // on Expr, goto 10
        3, // on Factor, goto 2
        4, // on Term, goto 3
        0, // on __Expr, error
        // State 5
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 6
        0, // on Expr, error
        12, // on Factor, goto 11
        4, // on Term, goto 3
        0, // on __Expr, error
        // State 7
        0, // on Expr, error
        13, // on Factor, goto 12
        4, // on Term, goto 3
        0, // on __Expr, error
        // State 8
        0, // on Expr, error
        0, // on Factor, error
        14, // on Term, goto 13
        0, // on __Expr, error
        // State 9
        0, // on Expr, error
        0, // on Factor, error
        15, // on Term, goto 14
        0, // on __Expr, error
        // State 10
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 11
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 12
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 13
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 14
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 15
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
    ];
    pub fn parse_Expr<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        scale: i32,
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
                (_, Tok::Times, _) if true => 2,
                (_, Tok::Plus, _) if true => 3,
                (_, Tok::Minus, _) if true => 4,
                (_, Tok::Div, _) if true => 5,
                (_, Tok::Num(_), _) if true => 6,
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
                let __action = __ACTION[__state * 7 + __integer];
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
                        2 => match __lookahead.1 {
                            __tok @ Tok::Times => __Symbol::Term_22_2a_22(__tok),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Tok::Plus => __Symbol::Term_22_2b_22(__tok),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Tok::Minus => __Symbol::Term_22_2d_22(__tok),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Tok::Div => __Symbol::Term_22_2f_22(__tok),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            Tok::Num(__tok0) => __Symbol::TermNum(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    println!("--> reduce");
                    if let Some(r) = __reduce(scale, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
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
                if let Some(r) = __reduce(scale, __action, None, &mut __states, &mut __symbols) {
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
        scale: i32,
        __action: i32,
        __lookahead_start: Option<&()>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
    ) -> Option<Result<i32,__ParseError<(),Tok,()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Expr = Expr, "-", Factor => ActionFn(1);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(scale, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            2 => {
                // Expr = Expr, "+", Factor => ActionFn(2);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(scale, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            3 => {
                // Expr = Factor => ActionFn(3);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(scale, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            4 => {
                // Factor = Factor, "*", Term => ActionFn(4);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            5 => {
                // Factor = Factor, "/", Term => ActionFn(5);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            6 => {
                // Factor = Term => ActionFn(6);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(scale, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            7 => {
                // Term = Num => ActionFn(7);
                let __sym0 = __pop_TermNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(scale, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                2
            }
            8 => {
                // Term = "(", Expr, ")" => ActionFn(8);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                2
            }
            9 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(scale, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 4 + __nonterminal] - 1;
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
    fn __pop_Term_22_2a_22<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Tok, ()) {
        println!("pop_Term_22_2a_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Tok, ()) {
        println!("pop_Term_22_2b_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Tok, ()) {
        println!("pop_Term_22_2d_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Tok, ()) {
        println!("pop_Term_22_2f_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermNum<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), i32, ()) {
        println!("pop_TermNum");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), i32, ()) {
        println!("pop_NtExpr");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), i32, ()) {
        println!("pop_NtFactor");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), i32, ()) {
        println!("pop_NtTerm");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), i32, ()) {
        println!("pop_Nt____Expr");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Expr::parse_Expr;

pub fn __action0<
>(
    scale: i32,
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

pub fn __action1<
>(
    scale: i32,
    (_, l, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    (_, r, _): ((), i32, ()),
) -> i32
{
    l - r
}

pub fn __action2<
>(
    scale: i32,
    (_, l, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    (_, r, _): ((), i32, ()),
) -> i32
{
    l + r
}

pub fn __action3<
>(
    scale: i32,
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

pub fn __action4<
>(
    scale: i32,
    (_, l, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    (_, r, _): ((), i32, ()),
) -> i32
{
    l * r
}

pub fn __action5<
>(
    scale: i32,
    (_, l, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    (_, r, _): ((), i32, ()),
) -> i32
{
    l / r
}

pub fn __action6<
>(
    scale: i32,
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

pub fn __action7<
>(
    scale: i32,
    (_, n, _): ((), i32, ()),
) -> i32
{
    n * scale
}

pub fn __action8<
>(
    scale: i32,
    (_, _, _): ((), Tok, ()),
    (_, __0, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
) -> i32
{
    (__0)
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
