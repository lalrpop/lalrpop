=======
use util::tok::Tok;
>>>>>>> More progress
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;
mod __parse__Expr {
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;

    struct ReducedProduction {
        nonterminal: u32,
        symbol_count: u32,
    }

    const productions: [ReducedProduction; 9] = [
            ReducedProduction { nonterminal: 0, symbol_count: 3 },
            ReducedProduction { nonterminal: 0, symbol_count: 3 },
            ReducedProduction { nonterminal: 0, symbol_count: 1 },
            ReducedProduction { nonterminal: 1, symbol_count: 3 },
            ReducedProduction { nonterminal: 1, symbol_count: 3 },
            ReducedProduction { nonterminal: 1, symbol_count: 1 },
            ReducedProduction { nonterminal: 2, symbol_count: 1 },
            ReducedProduction { nonterminal: 2, symbol_count: 3 },
            ReducedProduction { nonterminal: 3, symbol_count: 1 },
    ];
    const action_row_0: &'static [i32] = &[5, 0, 0, 0, 0, 0, 6, 0];
    const action_row_1: &'static [i32] = &[0, 0, 0, 7, 8, 0, 0, -8];
    const action_row_2: &'static [i32] = &[0, 0, 9, -2, -2, 10, 0, -2];
    const action_row_3: &'static [i32] = &[0, 0, -5, -5, -5, -5, 0, -5];
    const action_row_4: &'static [i32] = &[14, 0, 0, 0, 0, 0, 15, 0];
    const action_row_5: &'static [i32] = &[0, 0, -6, -6, -6, -6, 0, -6];
    const action_row_6: &'static [i32] = &[5, 0, 0, 0, 0, 0, 6, 0];
    const action_row_7: &'static [i32] = &[5, 0, 0, 0, 0, 0, 6, 0];
    const action_row_8: &'static [i32] = &[5, 0, 0, 0, 0, 0, 6, 0];
    const action_row_9: &'static [i32] = &[5, 0, 0, 0, 0, 0, 6, 0];
    const action_row_10: &'static [i32] = &[0, 20, 0, 21, 22, 0, 0, 0];
    const action_row_11: &'static [i32] = &[0, -2, 23, -2, -2, 24, 0, 0];
    const action_row_12: &'static [i32] = &[0, -5, -5, -5, -5, -5, 0, 0];
    const action_row_13: &'static [i32] = &[14, 0, 0, 0, 0, 0, 15, 0];
    const action_row_14: &'static [i32] = &[0, -6, -6, -6, -6, -6, 0, 0];
    const action_row_15: &'static [i32] = &[0, 0, 9, -1, -1, 10, 0, -1];
    const action_row_16: &'static [i32] = &[0, 0, 9, -0, -0, 10, 0, -0];
    const action_row_17: &'static [i32] = &[0, 0, -3, -3, -3, -3, 0, -3];
    const action_row_18: &'static [i32] = &[0, 0, -4, -4, -4, -4, 0, -4];
    const action_row_19: &'static [i32] = &[0, 0, -7, -7, -7, -7, 0, -7];
    const action_row_20: &'static [i32] = &[14, 0, 0, 0, 0, 0, 15, 0];
    const action_row_21: &'static [i32] = &[14, 0, 0, 0, 0, 0, 15, 0];
    const action_row_22: &'static [i32] = &[14, 0, 0, 0, 0, 0, 15, 0];
    const action_row_23: &'static [i32] = &[14, 0, 0, 0, 0, 0, 15, 0];
    const action_row_24: &'static [i32] = &[0, 30, 0, 21, 22, 0, 0, 0];
    const action_row_25: &'static [i32] = &[0, -1, 23, -1, -1, 24, 0, 0];
    const action_row_26: &'static [i32] = &[0, -0, 23, -0, -0, 24, 0, 0];
    const action_row_27: &'static [i32] = &[0, -3, -3, -3, -3, -3, 0, 0];
    const action_row_28: &'static [i32] = &[0, -4, -4, -4, -4, -4, 0, 0];
    const action_row_29: &'static [i32] = &[0, -7, -7, -7, -7, -7, 0, 0];
    const actions: [&'static [i32]; 30] = [action_row_0, action_row_1, action_row_2, action_row_3, action_row_4, action_row_5, action_row_6, action_row_7, action_row_8, action_row_9, action_row_10, action_row_11, action_row_12, action_row_13, action_row_14, action_row_15, action_row_16, action_row_17, action_row_18, action_row_19, action_row_20, action_row_21, action_row_22, action_row_23, action_row_24, action_row_25, action_row_26, action_row_27, action_row_28, action_row_29];

    const goto_row_0: &'static [u32] = &[1, 2, 3, 0];
    const goto_row_1: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_2: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_3: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_4: &'static [u32] = &[10, 11, 12, 0];
    const goto_row_5: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_6: &'static [u32] = &[0, 15, 3, 0];
    const goto_row_7: &'static [u32] = &[0, 16, 3, 0];
    const goto_row_8: &'static [u32] = &[0, 0, 17, 0];
    const goto_row_9: &'static [u32] = &[0, 0, 18, 0];
    const goto_row_10: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_11: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_12: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_13: &'static [u32] = &[24, 11, 12, 0];
    const goto_row_14: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_15: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_16: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_17: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_18: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_19: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_20: &'static [u32] = &[0, 25, 12, 0];
    const goto_row_21: &'static [u32] = &[0, 26, 12, 0];
    const goto_row_22: &'static [u32] = &[0, 0, 27, 0];
    const goto_row_23: &'static [u32] = &[0, 0, 28, 0];
    const goto_row_24: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_25: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_26: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_27: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_28: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_29: &'static [u32] = &[0, 0, 0, 0];
    const gotos: [&'static [u32]; 30] = [
goto_row_0, goto_row_1, goto_row_2, goto_row_3, goto_row_4, goto_row_5, goto_row_6, goto_row_7, goto_row_8, goto_row_9, goto_row_10, goto_row_11, goto_row_12, goto_row_13, goto_row_14, goto_row_15, goto_row_16, goto_row_17, goto_row_18, goto_row_19, goto_row_20, goto_row_21, goto_row_22, goto_row_23, goto_row_24, goto_row_25, goto_row_26, goto_row_27, goto_row_28, goto_row_29];

    fn terminal_to_index<
    >(
        token: &Tok,
    ) -> usize
    {
        match *token {
            Tok::LParen => 0,
            Tok::RParen => 1,
            Tok::Times => 2,
            Tok::Plus => 3,
            Tok::Minus => 4,
            Tok::Div => 5,
            Tok::Num(__tok0) => 6,
            _ => panic!("unuspported token"),
        }
    }
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
<<<<<<< 80265c63a967adf0d43a709fc83192e57465b51b
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
            }
            0
=======
=======
<<<<<<< 045f2bbc1f5b2d428fd580aa4bb6cc6303850c61
>>>>>>> More progress
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match try!(__state0(scale, &mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____Expr((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        Expr(((), i32, ())),
        Factor(((), i32, ())),
        Term(((), i32, ())),
        ____Expr(((), i32, ())),
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
    //     Expr = (*) Expr "+" Factor [EOF]
    //     Expr = (*) Expr "+" Factor ["+"]
    //     Expr = (*) Expr "+" Factor ["-"]
    //     Expr = (*) Expr "-" Factor [EOF]
    //     Expr = (*) Expr "-" Factor ["+"]
    //     Expr = (*) Expr "-" Factor ["-"]
    //     Expr = (*) Factor [EOF]
    //     Expr = (*) Factor ["+"]
    //     Expr = (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [EOF]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [EOF]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [EOF]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) Num [EOF]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     __Expr = (*) Expr [EOF]
    //
    //     "(" -> Shift(S4)
    //     Num -> Shift(S5)
    //
    //     Expr -> S1
    //     Factor -> S2
    //     Term -> S3
    pub fn __state0<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym0 = (__loc1, (__tok), __loc2);
                __result = try!(__state4(scale, __tokens, __sym0));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym0 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(scale, __tokens, __sym0));
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
                __Nonterminal::Expr(__sym0) => {
                    __result = try!(__state1(scale, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Factor(__sym0) => {
                    __result = try!(__state2(scale, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__sym0) => {
                    __result = try!(__custom0(scale, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //     Kind = None
    //     AllInputs = [Expr]
    //     OptionalInputs = []
    //     FixedInputs = [Expr]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr (*) "+" Factor [EOF]
    //     Expr = Expr (*) "+" Factor ["+"]
    //     Expr = Expr (*) "+" Factor ["-"]
    //     Expr = Expr (*) "-" Factor [EOF]
    //     Expr = Expr (*) "-" Factor ["+"]
    //     Expr = Expr (*) "-" Factor ["-"]
    //     __Expr = Expr (*) [EOF]
    //
    //     EOF -> Reduce(__Expr = Expr => ActionFn(0);)
    //     "+" -> Shift(S6)
    //     "-" -> Shift(S7)
    //
    pub fn __state1<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state6(scale, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state7(scale, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(scale, __sym0);
                let __nt = __Nonterminal::____Expr((
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
    //     Kind = None
    //     AllInputs = [Factor]
    //     OptionalInputs = []
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Factor (*) [EOF]
    //     Expr = Factor (*) ["+"]
    //     Expr = Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [EOF]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [EOF]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     EOF -> Reduce(Expr = Factor => ActionFn(3);)
    //     "*" -> Shift(S8)
    //     "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "/" -> Shift(S9)
    //
    pub fn __state2<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state8(scale, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state9(scale, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(scale, __sym0);
                let __nt = __Nonterminal::Expr((
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
    //     WillPush = [Expr, ")"]
    //     WillProduce = Some(Term)
    //
    //     Expr = (*) Expr "+" Factor [")"]
    //     Expr = (*) Expr "+" Factor ["+"]
    //     Expr = (*) Expr "+" Factor ["-"]
    //     Expr = (*) Expr "-" Factor [")"]
    //     Expr = (*) Expr "-" Factor ["+"]
    //     Expr = (*) Expr "-" Factor ["-"]
    //     Expr = (*) Factor [")"]
    //     Expr = (*) Factor ["+"]
    //     Expr = (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [")"]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [")"]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [")"]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = "(" (*) Expr ")" [EOF]
    //     Term = "(" (*) Expr ")" ["*"]
    //     Term = "(" (*) Expr ")" ["+"]
    //     Term = "(" (*) Expr ")" ["-"]
    //     Term = "(" (*) Expr ")" ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //
    //     "(" -> Shift(S13)
    //     Num -> Shift(S14)
    //
    //     Expr -> S10
    //     Factor -> S11
    //     Term -> S12
    pub fn __state4<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
                __result = try!(__state13(scale, __tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(scale, __tokens, __sym1));
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
                __Nonterminal::Expr(__sym1) => {
                    __result = try!(__state10(scale, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__sym1) => {
                    __result = try!(__state11(scale, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__sym1) => {
                    __result = try!(__custom0(scale, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 6
    //     Kind = None
    //     AllInputs = [Expr, "+"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "+"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "+" (*) Factor [EOF]
    //     Expr = Expr "+" (*) Factor ["+"]
    //     Expr = Expr "+" (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [EOF]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [EOF]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [EOF]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) Num [EOF]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //
    //     "(" -> Shift(S4)
    //     Num -> Shift(S5)
    //
    //     Factor -> S15
    //     Term -> S3
    pub fn __state6<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state4(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(scale, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state15(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom0(scale, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 7
    //     Kind = None
    //     AllInputs = [Expr, "-"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "-"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "-" (*) Factor [EOF]
    //     Expr = Expr "-" (*) Factor ["+"]
    //     Expr = Expr "-" (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [EOF]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [EOF]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [EOF]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) Num [EOF]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //
    //     "(" -> Shift(S4)
    //     Num -> Shift(S5)
    //
    //     Factor -> S16
    //     Term -> S3
    pub fn __state7<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state4(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(scale, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state16(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom0(scale, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 8
    //     Kind = None
    //     AllInputs = [Factor, "*"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "*"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "*" (*) Term [EOF]
    //     Factor = Factor "*" (*) Term ["*"]
    //     Factor = Factor "*" (*) Term ["+"]
    //     Factor = Factor "*" (*) Term ["-"]
    //     Factor = Factor "*" (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) Num [EOF]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //
    //     "(" -> Shift(S4)
    //     Num -> Shift(S5)
    //
    //     Term -> S17
    pub fn __state8<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
                __result = try!(__state4(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(scale, __tokens, __sym2));
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
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom2(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 9
    //     Kind = None
    //     AllInputs = [Factor, "/"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "/"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "/" (*) Term [EOF]
    //     Factor = Factor "/" (*) Term ["*"]
    //     Factor = Factor "/" (*) Term ["+"]
    //     Factor = Factor "/" (*) Term ["-"]
    //     Factor = Factor "/" (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) Num [EOF]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //
    //     "(" -> Shift(S4)
    //     Num -> Shift(S5)
    //
    //     Term -> S18
    pub fn __state9<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
                __result = try!(__state4(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(scale, __tokens, __sym2));
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
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom3(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 10
    //     Kind = None
    //     AllInputs = ["(", Expr]
    //     OptionalInputs = ["("]
    //     FixedInputs = [Expr]
    //     WillPushLen = 1
    //     WillPush = [")"]
    //     WillProduce = None
    //
    //     Expr = Expr (*) "+" Factor [")"]
    //     Expr = Expr (*) "+" Factor ["+"]
    //     Expr = Expr (*) "+" Factor ["-"]
    //     Expr = Expr (*) "-" Factor [")"]
    //     Expr = Expr (*) "-" Factor ["+"]
    //     Expr = Expr (*) "-" Factor ["-"]
    //     Term = "(" Expr (*) ")" [EOF]
    //     Term = "(" Expr (*) ")" ["*"]
    //     Term = "(" Expr (*) ")" ["+"]
    //     Term = "(" Expr (*) ")" ["-"]
    //     Term = "(" Expr (*) ")" ["/"]
    //
    //     ")" -> Shift(S19)
    //     "+" -> Shift(S20)
    //     "-" -> Shift(S21)
    //
    pub fn __state10<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
                __result = try!(__custom4(scale, __tokens, __sym0, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state20(scale, __tokens, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state21(scale, __tokens, __sym1, __sym2));
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

    // State 11
    //     Kind = None
    //     AllInputs = [Factor]
    //     OptionalInputs = []
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Factor (*) [")"]
    //     Expr = Factor (*) ["+"]
    //     Expr = Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [")"]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [")"]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     ")" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "*" -> Shift(S22)
    //     "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "/" -> Shift(S23)
    //
    pub fn __state11<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state22(scale, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__state23(scale, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(scale, __sym0);
                let __nt = __Nonterminal::Expr((
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

    // State 13
    //     Kind = None
    //     AllInputs = ["("]
    //     OptionalInputs = []
    //     FixedInputs = ["("]
    //     WillPushLen = 2
    //     WillPush = [Expr, ")"]
    //     WillProduce = Some(Term)
    //
    //     Expr = (*) Expr "+" Factor [")"]
    //     Expr = (*) Expr "+" Factor ["+"]
    //     Expr = (*) Expr "+" Factor ["-"]
    //     Expr = (*) Expr "-" Factor [")"]
    //     Expr = (*) Expr "-" Factor ["+"]
    //     Expr = (*) Expr "-" Factor ["-"]
    //     Expr = (*) Factor [")"]
    //     Expr = (*) Factor ["+"]
    //     Expr = (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [")"]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [")"]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [")"]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = "(" (*) Expr ")" [")"]
    //     Term = "(" (*) Expr ")" ["*"]
    //     Term = "(" (*) Expr ")" ["+"]
    //     Term = "(" (*) Expr ")" ["-"]
    //     Term = "(" (*) Expr ")" ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //
    //     "(" -> Shift(S13)
    //     Num -> Shift(S14)
    //
    //     Expr -> S24
    //     Factor -> S11
    //     Term -> S12
    pub fn __state13<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
                __result = try!(__state13(scale, __tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(scale, __tokens, __sym1));
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
                __Nonterminal::Expr(__sym1) => {
                    __result = try!(__state24(scale, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__sym1) => {
                    __result = try!(__state11(scale, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__sym1) => {
                    __result = try!(__custom0(scale, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 15
    //     Kind = None
    //     AllInputs = [Expr, "+", Factor]
    //     OptionalInputs = [Expr, "+"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "+" Factor (*) [EOF]
    //     Expr = Expr "+" Factor (*) ["+"]
    //     Expr = Expr "+" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [EOF]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [EOF]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     EOF -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "*" -> Shift(S8)
    //     "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "/" -> Shift(S9)
    //
    pub fn __state15<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
        __sym1: &mut Option<((), Tok, ())>,
        __sym2: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state8(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state9(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(scale, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
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

    // State 16
    //     Kind = None
    //     AllInputs = [Expr, "-", Factor]
    //     OptionalInputs = [Expr, "-"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "-" Factor (*) [EOF]
    //     Expr = Expr "-" Factor (*) ["+"]
    //     Expr = Expr "-" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [EOF]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [EOF]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     EOF -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "*" -> Shift(S8)
    //     "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "/" -> Shift(S9)
    //
    pub fn __state16<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
        __sym1: &mut Option<((), Tok, ())>,
        __sym2: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state8(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state9(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(scale, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
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

    // State 20
    //     Kind = None
    //     AllInputs = [Expr, "+"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "+"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "+" (*) Factor [")"]
    //     Expr = Expr "+" (*) Factor ["+"]
    //     Expr = Expr "+" (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [")"]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [")"]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [")"]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //
    //     "(" -> Shift(S13)
    //     Num -> Shift(S14)
    //
    //     Factor -> S25
    //     Term -> S12
    pub fn __state20<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state13(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(scale, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state25(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom0(scale, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 21
    //     Kind = None
    //     AllInputs = [Expr, "-"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "-"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "-" (*) Factor [")"]
    //     Expr = Expr "-" (*) Factor ["+"]
    //     Expr = Expr "-" (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [")"]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [")"]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [")"]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //
    //     "(" -> Shift(S13)
    //     Num -> Shift(S14)
    //
    //     Factor -> S26
    //     Term -> S12
    pub fn __state21<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state13(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(scale, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state26(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom0(scale, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 22
    //     Kind = None
    //     AllInputs = [Factor, "*"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "*"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "*" (*) Term [")"]
    //     Factor = Factor "*" (*) Term ["*"]
    //     Factor = Factor "*" (*) Term ["+"]
    //     Factor = Factor "*" (*) Term ["-"]
    //     Factor = Factor "*" (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //
    //     "(" -> Shift(S13)
    //     Num -> Shift(S14)
    //
    //     Term -> S27
    pub fn __state22<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
                __result = try!(__state13(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(scale, __tokens, __sym2));
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
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom2(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 23
    //     Kind = None
    //     AllInputs = [Factor, "/"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "/"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "/" (*) Term [")"]
    //     Factor = Factor "/" (*) Term ["*"]
    //     Factor = Factor "/" (*) Term ["+"]
    //     Factor = Factor "/" (*) Term ["-"]
    //     Factor = Factor "/" (*) Term ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //
    //     "(" -> Shift(S13)
    //     Num -> Shift(S14)
    //
    //     Term -> S28
    pub fn __state23<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
                __result = try!(__state13(scale, __tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(scale, __tokens, __sym2));
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
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom3(scale, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 24
    //     Kind = None
    //     AllInputs = ["(", Expr]
    //     OptionalInputs = ["("]
    //     FixedInputs = [Expr]
    //     WillPushLen = 1
    //     WillPush = [")"]
    //     WillProduce = None
    //
    //     Expr = Expr (*) "+" Factor [")"]
    //     Expr = Expr (*) "+" Factor ["+"]
    //     Expr = Expr (*) "+" Factor ["-"]
    //     Expr = Expr (*) "-" Factor [")"]
    //     Expr = Expr (*) "-" Factor ["+"]
    //     Expr = Expr (*) "-" Factor ["-"]
    //     Term = "(" Expr (*) ")" [")"]
    //     Term = "(" Expr (*) ")" ["*"]
    //     Term = "(" Expr (*) ")" ["+"]
    //     Term = "(" Expr (*) ")" ["-"]
    //     Term = "(" Expr (*) ")" ["/"]
    //
    //     ")" -> Shift(S29)
    //     "+" -> Shift(S20)
    //     "-" -> Shift(S21)
    //
    pub fn __state24<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
                __result = try!(__custom4(scale, __tokens, __sym0, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state20(scale, __tokens, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __sym2 = (__loc1, (__tok), __loc2);
                __result = try!(__state21(scale, __tokens, __sym1, __sym2));
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

    // State 25
    //     Kind = None
    //     AllInputs = [Expr, "+", Factor]
    //     OptionalInputs = [Expr, "+"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "+" Factor (*) [")"]
    //     Expr = Expr "+" Factor (*) ["+"]
    //     Expr = Expr "+" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [")"]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [")"]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     ")" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "*" -> Shift(S22)
    //     "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "/" -> Shift(S23)
    //
    pub fn __state25<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
        __sym1: &mut Option<((), Tok, ())>,
        __sym2: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state22(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state23(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(scale, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
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

    // State 26
    //     Kind = None
    //     AllInputs = [Expr, "-", Factor]
    //     OptionalInputs = [Expr, "-"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "-" Factor (*) [")"]
    //     Expr = Expr "-" Factor (*) ["+"]
    //     Expr = Expr "-" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [")"]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [")"]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     ")" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "*" -> Shift(S22)
    //     "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "/" -> Shift(S23)
    //
    pub fn __state26<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<((), i32, ())>,
        __sym1: &mut Option<((), Tok, ())>,
        __sym2: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state22(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let __sym3 = (__loc1, (__tok), __loc2);
                __result = try!(__state23(scale, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(scale, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
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

    // Custom 0
    //    Reduce Factor = Term => ActionFn(6);
    pub fn __custom0<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: ((), i32, ()),
    ) -> Result<(Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<((), Tok, ())>, __Nonterminal<>);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6(scale, __sym0);
        let __nt = __Nonterminal::Factor((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 1
    //    Reduce Term = Num => ActionFn(7);
    pub fn __custom1<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
        let __nt = super::__action7(scale, __sym0);
        let __nt = __Nonterminal::Term((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 2
    //    Reduce Factor = Factor, "*", Term => ActionFn(4);
    pub fn __custom2<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
        let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
        let __nt = __Nonterminal::Factor((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 3
    //    Reduce Factor = Factor, "/", Term => ActionFn(5);
    pub fn __custom3<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
        let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
        let __nt = __Nonterminal::Factor((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 4
    //    Reduce Term = "(", Expr, ")" => ActionFn(8);
    pub fn __custom4<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
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
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Term((
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
            }
>>>>>>> Port lalrpop-test to use new `Configuration` value
        }
=======
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
        let __nt = __Nonterminal::Term((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
<<<<<<< 80265c63a967adf0d43a709fc83192e57465b51b
>>>>>>> update test output
=======
=======
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
            }
            0
        }
>>>>>>> More progress
>>>>>>> More progress
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
