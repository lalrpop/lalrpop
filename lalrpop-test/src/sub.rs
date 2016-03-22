use self::__lalrpop_util::ParseError as __ParseError;
mod __parse__S {
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;

    struct ReducedProduction {
        nonterminal: u32,
        symbol_count: u32,
    }

    const productions: [ReducedProduction; 6] = [
            ReducedProduction { nonterminal: 0, symbol_count: 3 },
            ReducedProduction { nonterminal: 0, symbol_count: 1 },
            ReducedProduction { nonterminal: 1, symbol_count: 1 },
            ReducedProduction { nonterminal: 2, symbol_count: 1 },
            ReducedProduction { nonterminal: 2, symbol_count: 3 },
            ReducedProduction { nonterminal: 3, symbol_count: 1 },
    ];
    const action_row_0: &'static [i32] = &[5, 0, 0, 6, 0];
    const action_row_1: &'static [i32] = &[0, 0, 7, 0, -2];
    const action_row_2: &'static [i32] = &[0, 0, 0, 0, -5];
    const action_row_3: &'static [i32] = &[0, 0, -1, 0, -1];
    const action_row_4: &'static [i32] = &[10, 0, 0, 11, 0];
    const action_row_5: &'static [i32] = &[0, 0, -3, 0, -3];
    const action_row_6: &'static [i32] = &[5, 0, 0, 6, 0];
    const action_row_7: &'static [i32] = &[0, 13, 14, 0, 0];
    const action_row_8: &'static [i32] = &[0, -1, -1, 0, 0];
    const action_row_9: &'static [i32] = &[10, 0, 0, 11, 0];
    const action_row_10: &'static [i32] = &[0, -3, -3, 0, 0];
    const action_row_11: &'static [i32] = &[0, 0, -0, 0, -0];
    const action_row_12: &'static [i32] = &[0, 0, -4, 0, -4];
    const action_row_13: &'static [i32] = &[10, 0, 0, 11, 0];
    const action_row_14: &'static [i32] = &[0, 17, 14, 0, 0];
    const action_row_15: &'static [i32] = &[0, -0, -0, 0, 0];
    const action_row_16: &'static [i32] = &[0, -4, -4, 0, 0];
    const actions: [&'static [i32]; 17] = [action_row_0, action_row_1, action_row_2, action_row_3, action_row_4, action_row_5, action_row_6, action_row_7, action_row_8, action_row_9, action_row_10, action_row_11, action_row_12, action_row_13, action_row_14, action_row_15, action_row_16];

    const goto_row_0: &'static [u32] = &[1, 2, 3, 0];
    const goto_row_1: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_2: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_3: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_4: &'static [u32] = &[7, 0, 8, 0];
    const goto_row_5: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_6: &'static [u32] = &[0, 0, 11, 0];
    const goto_row_7: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_8: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_9: &'static [u32] = &[14, 0, 8, 0];
    const goto_row_10: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_11: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_12: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_13: &'static [u32] = &[0, 0, 15, 0];
    const goto_row_14: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_15: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_16: &'static [u32] = &[0, 0, 0, 0];
    const gotos: [&'static [u32]; 17] = [
goto_row_0, goto_row_1, goto_row_2, goto_row_3, goto_row_4, goto_row_5, goto_row_6, goto_row_7, goto_row_8, goto_row_9, goto_row_10, goto_row_11, goto_row_12, goto_row_13, goto_row_14, goto_row_15, goto_row_16];

    fn terminal_to_index<
    >(
        token: &Tok,
    ) -> usize
    {
        match *token {
            Tok::LParen => 0,
            Tok::RParen => 1,
            Tok::Minus => 2,
            Tok::Num(__tok0) => 3,
            _ => panic!("unuspported token"),
        }
    }
    pub fn parse_S<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<i32, __ParseError<(),Tok,()>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
        let mut __machine = Machine::new();
        __machine.execute_partial(&mut __tokens);
        Err(__ParseError::ExtraToken { token: __tokens.next().expect("no more tokens").unwrap() })
    }
    enum StackData {
        Empty,
        Terminal(((), Tok, ())),
        Nt0(i32),
        Nt1(i32),
        Nt2(i32),
        Nt3(i32),
    }

    struct Machine {
        state_stack: Vec<u32>,
        data_stack: Vec<StackData>
    }
    impl Machine {
        fn new() -> Machine {
            Machine { state_stack: Vec::new(), data_stack: Vec::new() }
        }
        fn top_state(&self) -> usize {
            *self.state_stack.last().expect("state stack is empty!") as usize
        }
        fn dispatch_action(&self, nonterminal: u32, args: Vec<StackData>) -> StackData {
            StackData::Empty
        }
        fn reduce(&mut self, production: &ReducedProduction) {
            let mut args = Vec::new();
            for _ in 0 .. production.symbol_count {
                args.push(self.data_stack.pop().expect("popped data stack"));
                self.state_stack.pop();
            }
            let top_state = self.top_state();
            self.state_stack.push(gotos[top_state][production.nonterminal as usize]);
            let res = self.dispatch_action(production.nonterminal, args);
            self.data_stack.push(res);
        }
        fn execute_partial<
            __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
        >(
            &mut self,
            __tokens: &mut __TOKENS,
        ) -> usize
        {
            self.state_stack.push(0);
            let mut __token = __tokens.next();
            while let Some(Ok((l, terminal, r))) = __token {
                let terminal_index = terminal_to_index(&terminal);
                let state = self.top_state();
                let action = actions[state][terminal_index];
                if action > 0 {
                    self.state_stack.push((action-1) as u32);
                    self.data_stack.push(StackData::Terminal((l, terminal, r)));
                    __token = __tokens.next();
                } else if action < 0 {
                    self.reduce(&productions[(action*-1) as usize]);
                    __token = Some(Ok((l, terminal, r)));
                } else {
                    __token = None;
                    // error
                }
=======
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match try!(__state0(&mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____S((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        E(((), i32, ())),
        S(((), i32, ())),
        T(((), i32, ())),
        ____S(((), i32, ())),
    }

    // State 0
    //     Kind = None
    //     AllInputs = []
    //     OptionalInputs = []
    //     FixedInputs = []
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     E = (*) E "-" T [EOF]
    //     E = (*) E "-" T ["-"]
    //     E = (*) T [EOF]
    //     E = (*) T ["-"]
    //     S = (*) E [EOF]
    //     T = (*) "(" E ")" [EOF]
    //     T = (*) "(" E ")" ["-"]
    //     T = (*) Num [EOF]
    //     T = (*) Num ["-"]
    //     __S = (*) S [EOF]
    //
    //     "(" -> Shift(S4)
    //     Num -> Shift(S5)
    //
    //     E -> S1
    //     S -> S2
    //     T -> S3
    pub fn __state0<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym0 = (__loc1, (__tok), __loc2);
                __result = try!(__state4(__tokens, __sym0));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym0 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(__tokens, __sym0));
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
                __Nonterminal::E(__sym0) => {
                    __result = try!(__state1(__tokens, __lookahead, __sym0));
                }
                __Nonterminal::S(__sym0) => {
                    __result = try!(__custom0(__tokens, __lookahead, __sym0));
                }
                __Nonterminal::T(__sym0) => {
                    __result = try!(__custom1(__tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //     Kind = None
    //     AllInputs = [E]
    //     OptionalInputs = []
    //     FixedInputs = [E]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     E = E (*) "-" T [EOF]
    //     E = E (*) "-" T ["-"]
    //     S = E (*) [EOF]
    //
    //     EOF -> Reduce(S = E => ActionFn(1);)
    //     "-" -> Shift(S6)
    //
    pub fn __state1<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state6(__tokens, __sym0, __sym1));
                return Ok(__result);
            }
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(__sym0);
                let __nt = __Nonterminal::S((
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

    // State 4
    //     Kind = None
    //     AllInputs = ["("]
    //     OptionalInputs = []
    //     FixedInputs = ["("]
    //     WillPushLen = 2
    //     WillPush = [E, ")"]
    //     WillProduce = Some(T)
    //
    //     E = (*) E "-" T [")"]
    //     E = (*) E "-" T ["-"]
    //     E = (*) T [")"]
    //     E = (*) T ["-"]
    //     T = (*) "(" E ")" [")"]
    //     T = (*) "(" E ")" ["-"]
    //     T = "(" (*) E ")" [EOF]
    //     T = "(" (*) E ")" ["-"]
    //     T = (*) Num [")"]
    //     T = (*) Num ["-"]
    //
    //     "(" -> Shift(S9)
    //     Num -> Shift(S10)
    //
    //     E -> S7
    //     T -> S8
    pub fn __state4<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: ((), Tok, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        let __sym0 = &mut Some(__sym0);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state9(__tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(__tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym0.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E(__sym1) => {
                    __result = try!(__state7(__tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T(__sym1) => {
                    __result = try!(__custom1(__tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 6
    //     Kind = None
    //     AllInputs = [E, "-"]
    //     OptionalInputs = []
    //     FixedInputs = [E, "-"]
    //     WillPushLen = 1
    //     WillPush = [T]
    //     WillProduce = Some(E)
    //
    //     E = E "-" (*) T [EOF]
    //     E = E "-" (*) T ["-"]
    //     T = (*) "(" E ")" [EOF]
    //     T = (*) "(" E ")" ["-"]
    //     T = (*) Num [EOF]
    //     T = (*) Num ["-"]
    //
    //     "(" -> Shift(S4)
    //     Num -> Shift(S5)
    //
    //     T -> S11
    pub fn __state6<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: ((), i32, ()),
        __sym1: ((), Tok, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state4(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(__tokens, __sym2));
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
                __Nonterminal::T(__sym2) => {
                    __result = try!(__custom3(__tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 7
    //     Kind = None
    //     AllInputs = ["(", E]
    //     OptionalInputs = ["("]
    //     FixedInputs = [E]
    //     WillPushLen = 1
    //     WillPush = [")"]
    //     WillProduce = None
    //
    //     E = E (*) "-" T [")"]
    //     E = E (*) "-" T ["-"]
    //     T = "(" E (*) ")" [EOF]
    //     T = "(" E (*) ")" ["-"]
    //
    //     ")" -> Shift(S12)
    //     "-" -> Shift(S13)
    //
    pub fn __state7<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), Tok, ())>,
        __sym1: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                let __sym0 = __sym0.take().unwrap();
                __result = try!(__custom4(__tokens, __sym0, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state13(__tokens, __sym1, __sym2));
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

    // State 9
    //     Kind = None
    //     AllInputs = ["("]
    //     OptionalInputs = []
    //     FixedInputs = ["("]
    //     WillPushLen = 2
    //     WillPush = [E, ")"]
    //     WillProduce = Some(T)
    //
    //     E = (*) E "-" T [")"]
    //     E = (*) E "-" T ["-"]
    //     E = (*) T [")"]
    //     E = (*) T ["-"]
    //     T = (*) "(" E ")" [")"]
    //     T = (*) "(" E ")" ["-"]
    //     T = "(" (*) E ")" [")"]
    //     T = "(" (*) E ")" ["-"]
    //     T = (*) Num [")"]
    //     T = (*) Num ["-"]
    //
    //     "(" -> Shift(S9)
    //     Num -> Shift(S10)
    //
    //     E -> S14
    //     T -> S8
    pub fn __state9<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: ((), Tok, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        let __sym0 = &mut Some(__sym0);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state9(__tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(__tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym0.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E(__sym1) => {
                    __result = try!(__state14(__tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::T(__sym1) => {
                    __result = try!(__custom1(__tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 13
    //     Kind = None
    //     AllInputs = [E, "-"]
    //     OptionalInputs = []
    //     FixedInputs = [E, "-"]
    //     WillPushLen = 1
    //     WillPush = [T]
    //     WillProduce = Some(E)
    //
    //     E = E "-" (*) T [")"]
    //     E = E "-" (*) T ["-"]
    //     T = (*) "(" E ")" [")"]
    //     T = (*) "(" E ")" ["-"]
    //     T = (*) Num [")"]
    //     T = (*) Num ["-"]
    //
    //     "(" -> Shift(S9)
    //     Num -> Shift(S10)
    //
    //     T -> S15
    pub fn __state13<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: ((), i32, ()),
        __sym1: ((), Tok, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state9(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(__tokens, __sym2));
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
                __Nonterminal::T(__sym2) => {
                    __result = try!(__custom3(__tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 14
    //     Kind = None
    //     AllInputs = ["(", E]
    //     OptionalInputs = ["("]
    //     FixedInputs = [E]
    //     WillPushLen = 1
    //     WillPush = [")"]
    //     WillProduce = None
    //
    //     E = E (*) "-" T [")"]
    //     E = E (*) "-" T ["-"]
    //     T = "(" E (*) ")" [")"]
    //     T = "(" E (*) ")" ["-"]
    //
    //     ")" -> Shift(S16)
    //     "-" -> Shift(S13)
    //
    pub fn __state14<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), Tok, ())>,
        __sym1: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                let __sym0 = __sym0.take().unwrap();
                __result = try!(__custom4(__tokens, __sym0, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state13(__tokens, __sym1, __sym2));
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

    // Custom 0
    //    Reduce __S = S => ActionFn(0);
    pub fn __custom0<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0(__sym0);
        let __nt = __Nonterminal::____S((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 1
    //    Reduce E = T => ActionFn(3);
    pub fn __custom1<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3(__sym0);
        let __nt = __Nonterminal::E((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 2
    //    Reduce T = Num => ActionFn(4);
    pub fn __custom2<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4(__sym0);
        let __nt = __Nonterminal::T((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 3
    //    Reduce E = E, "-", T => ActionFn(2);
    pub fn __custom3<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
        __sym1: ((), Tok, ()),
        __sym2: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action2(__sym0, __sym1, __sym2);
        let __nt = __Nonterminal::E((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 4
    //    Reduce T = "(", E, ")" => ActionFn(5);
    pub fn __custom4<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: ((), Tok, ()),
        __sym1: ((), i32, ()),
        __sym2: ((), Tok, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::T((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
>>>>>>> Port lalrpop-test to use new `Configuration` value
            }
            0
        }
=======
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5(__sym0, __sym1, __sym2);
        let __nt = __Nonterminal::T((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
>>>>>>> update test output
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
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

pub fn __action2<
>(
    (_, l, _): ((), i32, ()),
    (_, _, _): ((), Tok, ()),
    (_, r, _): ((), i32, ()),
) -> i32
{
    l - r
}

pub fn __action3<
>(
    (_, t, _): ((), i32, ()),
) -> i32
{
    t - super::ZERO
}

pub fn __action4<
>(
    (_, __0, _): ((), i32, ()),
) -> i32
{
    (__0)
}

pub fn __action5<
>(
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
