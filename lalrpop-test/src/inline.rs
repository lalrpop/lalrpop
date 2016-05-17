use self::__lalrpop_util::ParseError as __ParseError;
mod __parse__E {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;

    struct ReducedProduction {
        nonterminal: u32,
        symbol_count: u32,
    }
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
    const productions: [ReducedProduction; 7] = [
            ReducedProduction { nonterminal: 0, symbol_count: 0 },
            ReducedProduction { nonterminal: 1, symbol_count: 1 },
            ReducedProduction { nonterminal: 1, symbol_count: 2 },
            ReducedProduction { nonterminal: 1, symbol_count: 3 },
            ReducedProduction { nonterminal: 2, symbol_count: 0 },
            ReducedProduction { nonterminal: 2, symbol_count: 1 },
            ReducedProduction { nonterminal: 3, symbol_count: 1 },
    ];
    const action_row_0: &'static [i32] = &[3, 4, 0];
    const action_row_1: &'static [i32] = &[0, 0, -6];
    const action_row_2: &'static [i32] = &[3, 6, 0];
    const action_row_3: &'static [i32] = &[0, 0, -1];
    const action_row_4: &'static [i32] = &[0, 0, -2];
    const action_row_5: &'static [i32] = &[3, 4, -1];
    const action_row_6: &'static [i32] = &[0, 0, -3];
    const actions: [&'static [i32]; 7] = [action_row_0, action_row_1, action_row_2, action_row_3, action_row_4, action_row_5, action_row_6];

    const goto_row_0: &'static [u32] = &[0, 1, 0, 0];
    const goto_row_1: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_2: &'static [u32] = &[0, 4, 0, 0];
    const goto_row_3: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_4: &'static [u32] = &[0, 0, 0, 0];
    const goto_row_5: &'static [u32] = &[0, 6, 0, 0];
    const goto_row_6: &'static [u32] = &[0, 0, 0, 0];
    const gotos: [&'static [u32]; 7] = [
goto_row_0, goto_row_1, goto_row_2, goto_row_3, goto_row_4, goto_row_5, goto_row_6];

    fn terminal_to_index<
=======
    // State 0
    //     Kind = None
    //     AllInputs = []
    //     OptionalInputs = []
    //     FixedInputs = []
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     E = (*) "&" E [EOF]
    //     E = (*) "&" "L" E [EOF]
    //     E = (*) "L" [EOF]
    //     __E = (*) E [EOF]
    //
    //     "&" -> Shift(S2)
    //     "L" -> Shift(S3)
    //
    //     E -> S1
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym0 = (__loc1, (__tok0), __loc2);
                __result = try!(__state2(input, __tokens, __sym0));
            }
            Some((__loc1, (1, __tok0), __loc2)) => {
                let __sym0 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym0));
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
                    __result = try!(__custom0(input, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 2
    //     Kind = None
    //     AllInputs = ["&"]
    //     OptionalInputs = []
    //     FixedInputs = ["&"]
    //     WillPushLen = 1
    //     WillPush = [E]
    //     WillProduce = Some(E)
    //
    //     E = (*) "&" E [EOF]
    //     E = "&" (*) E [EOF]
    //     E = (*) "&" "L" E [EOF]
    //     E = "&" (*) "L" E [EOF]
    //     E = (*) "L" [EOF]
    //
    //     "&" -> Shift(S2)
    //     "L" -> Shift(S5)
    //
    //     E -> S4
    pub fn __state2<
>>>>>>> Port lalrpop-test to use new `Configuration` value
        'input,
    >(
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6
        token: &(usize, &'input str),
    ) -> usize
    {
        match *token {
            (0, __tok0) => 0,
            (1, __tok0) => 1,
            _ => panic!("unuspported token"),
=======
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state2(input, __tokens, __sym1));
            }
            Some((__loc1, (1, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(input, __tokens, __sym0, __sym1));
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
                    let __sym0 = __sym0.take().unwrap();
                    __result = try!(__custom2(input, __tokens, __lookahead, __sym0, __sym1));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
>>>>>>> update test output
        }
    }
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
    pub fn parse_E<
=======

    // State 5
    //     Kind = None
    //     AllInputs = ["&", "L"]
    //     OptionalInputs = ["&"]
    //     FixedInputs = ["L"]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = Some(E)
    //
    //     E = (*) "&" E [EOF]
    //     E = (*) "&" "L" E [EOF]
    //     E = "&" "L" (*) E [EOF]
    //     E = (*) "L" [EOF]
    //     E = "L" (*) [EOF]
    //
    //     EOF -> Reduce(E = "L" => ActionFn(1);)
    //     "&" -> Shift(S2)
    //     "L" -> Shift(S3)
    //
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6
    pub fn __state3<
>>>>>>> Port lalrpop-test to use new `Configuration` value
=======
    //     E -> S6
    pub fn __state5<
>>>>>>> update test output
        'input,
    >(
        input: &'input str,
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6
    ) -> Result<String, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __machine = Machine::new();
        __machine.execute_partial(&mut __tokens);
        Err(__ParseError::ExtraToken { token: __tokens.next().expect("no more tokens").unwrap() })
    }
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
    enum StackData<'input> {
        Empty,
        Terminal((usize, (usize, &'input str), usize)),
        Nt0(()),
        Nt1(String),
        Nt2(String),
        Nt3(String),
    }

    struct Machine<'input> {
        state_stack: Vec<u32>,
        data_stack: Vec<StackData<'input>>
    }
    impl<'input> Machine<'input> {
        fn new() -> Machine<'input> {
            Machine { state_stack: Vec::new(), data_stack: Vec::new() }
=======

    // State 4
    //   E = "&" E (*) [EOF]
    //
    //   EOF -> Reduce(E = "&", E => ActionFn(7);)
    //
    pub fn __state4<
=======
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state2(input, __tokens, __sym2));
            }
            Some((__loc1, (1, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom1(input, __tokens, __sym2));
            }
            None => {
                let __start = __sym1.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action1(input, __sym1);
                let __nt = __Nonterminal::E((
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
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E(__sym2) => {
                    let __sym0 = __sym0.take().unwrap();
                    __result = try!(__custom3(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // Custom 0
    //    Reduce __E = E => ActionFn(0);
    pub fn __custom0<
>>>>>>> update test output
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, String, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0(input, __sym0);
        let __nt = __Nonterminal::____E((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 1
    //    Reduce E = "L" => ActionFn(1);
    pub fn __custom1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state2(input, __tokens, __sym2));
            }
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state3(input, __tokens, __sym2));
            }
            None => {
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym1.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action1(input, __sym1);
                let __nt = __Nonterminal::E((
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
        fn top_state(&self) -> usize {
            *self.state_stack.last().expect("state stack is empty!") as usize
        }
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
        fn dispatch_action(&self, nonterminal: u32, args: Vec<StackData<'input>>) -> StackData<'input> {
            StackData::Empty
        }
        fn reduce(&mut self, production: &ReducedProduction) {
            let mut args = Vec::new();
            for _ in 0 .. production.symbol_count {
                args.push(self.data_stack.pop().expect("popped data stack"));
                self.state_stack.pop();
=======
=======
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1(input, __sym0);
        let __nt = __Nonterminal::E((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
>>>>>>> update test output
        return Ok(__result);
    }

    // Custom 2
    //    Reduce E = "&", E => ActionFn(7);
    pub fn __custom2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, &'input str, usize),
        __sym1: (usize, String, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::E((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
>>>>>>> Port lalrpop-test to use new `Configuration` value
            }
            let top_state = self.top_state();
            self.state_stack.push(gotos[top_state][production.nonterminal as usize]);
            let res = self.dispatch_action(production.nonterminal, args);
            self.data_stack.push(res);
        }
        fn execute_partial<
            __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
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
=======
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action7(input, __sym0, __sym1);
        let __nt = __Nonterminal::E((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 3
    //    Reduce E = "&", "L", E => ActionFn(8);
    pub fn __custom3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, &'input str, usize),
        __sym1: (usize, &'input str, usize),
        __sym2: (usize, String, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8(input, __sym0, __sym1, __sym2);
        let __nt = __Nonterminal::E((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
>>>>>>> update test output
    }
}
pub use self::__parse__E::parse_E;
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
                        38 => /* '&' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        76 => /* 'L' */ {
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
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    format!("L")
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
    (_, __1, _): (usize, String, usize),
) -> String
{
    format!("& {} {}", __0, __1)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> String
{
    format!("()")
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    format!("L")
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ()
{
    ()
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> String
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action5(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __temp0,
    )
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
) -> String
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action6(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __temp0,
        __1,
    )
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, String, usize),
) -> String
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action4(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __temp0,
        __2,
    )
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
