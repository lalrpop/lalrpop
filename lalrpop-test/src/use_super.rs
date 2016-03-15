=======
use super::util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
>>>>>>> More progress
use self::__lalrpop_util::ParseError as __ParseError;
mod __parse__S {
    use super::super::util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;

    struct ReducedProduction {
        nonterminal: u32,
        symbol_count: u32,
    }

    const productions: [ReducedProduction; 2] = [
            ReducedProduction { nonterminal: 0, symbol_count: 2 },
            ReducedProduction { nonterminal: 1, symbol_count: 1 },
    ];
    const action_row_0: &'static [i32] = &[3, 0, 0];
    const action_row_1: &'static [i32] = &[0, 0, -1];
    const action_row_2: &'static [i32] = &[0, 4, 0];
    const action_row_3: &'static [i32] = &[0, 0, -0];
    const actions: [&'static [i32]; 4] = [action_row_0, action_row_1, action_row_2, action_row_3];

    const goto_row_0: &'static [u32] = &[1, 0];
    const goto_row_1: &'static [u32] = &[0, 0];
    const goto_row_2: &'static [u32] = &[0, 0];
    const goto_row_3: &'static [u32] = &[0, 0];
    const gotos: [&'static [u32]; 4] = [
goto_row_0, goto_row_1, goto_row_2, goto_row_3];

    fn terminal_to_index<
    >(
        token: &Tok,
    ) -> usize
    {
        match *token {
            Tok::LParen => 0,
            Tok::RParen => 1,
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
        let mut __machine = Machine::new();
        __machine.execute_partial(&mut __tokens);
        Err(__ParseError::ExtraToken { token: __tokens.next().expect("no more tokens").unwrap() })
    }
    enum StackData {
        Empty,
        Terminal(((), Tok, ())),
        Nt0(i32),
        Nt1(i32),
    }

<<<<<<< 80265c63a967adf0d43a709fc83192e57465b51b
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
    struct Machine {
        state_stack: Vec<u32>,
        data_stack: Vec<StackData>
    }
    impl Machine {
        fn new() -> Machine {
            Machine { state_stack: Vec::new(), data_stack: Vec::new() }
=======
=======
<<<<<<< 045f2bbc1f5b2d428fd580aa4bb6cc6303850c61
>>>>>>> More progress
    // State 0
    //     Kind = None
    //     AllInputs = []
    //     OptionalInputs = []
    //     FixedInputs = []
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     S = (*) "(" ")" [EOF]
    //     __S = (*) S [EOF]
    //
    //     "(" -> Shift(S2)
    //
    //     S -> S1
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
                __result = try!(__state2(__tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
>>>>>>> Port lalrpop-test to use new `Configuration` value
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6
        fn top_state(&self) -> usize {
            *self.state_stack.last().expect("state stack is empty!") as usize
        }
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
        fn dispatch_action(&self, nonterminal: u32, args: Vec<StackData>) -> StackData {
            StackData::Empty
=======
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::S(__sym0) => {
                    __result = try!(__custom0(__tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
>>>>>>> update test output
        }
        fn reduce(&mut self, production: &ReducedProduction) {
            let mut args = Vec::new();
            for _ in 0 .. production.symbol_count {
                args.push(self.data_stack.pop().expect("popped data stack"));
                self.state_stack.pop();
=======
    }

    // State 2
    //     Kind = None
    //     AllInputs = ["("]
    //     OptionalInputs = []
    //     FixedInputs = ["("]
    //     WillPushLen = 1
    //     WillPush = [")"]
    //     WillProduce = Some(S)
    //
    //     S = "(" (*) ")" [EOF]
    //
    //     ")" -> Shift(S3)
    //
    pub fn __state2<
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
        match __lookahead {
            Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__custom1(__tokens, __sym0, __sym1));
                return Ok(__result);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
>>>>>>> Port lalrpop-test to use new `Configuration` value
            }
            let top_state = self.top_state();
            self.state_stack.push(gotos[top_state][production.nonterminal as usize]);
            let res = self.dispatch_action(production.nonterminal, args);
            self.data_stack.push(res);
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
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
=======
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
>>>>>>> update test output
        return Ok(__result);
    }

    // Custom 1
    //    Reduce S = "(", ")" => ActionFn(1);
    pub fn __custom1<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: ((), Tok, ()),
        __sym1: ((), Tok, ()),
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
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action1(__sym0, __sym1);
                let __nt = __Nonterminal::S((
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
        let __end = __sym1.2.clone();
        let __nt = super::__action1(__sym0, __sym1);
        let __nt = __Nonterminal::S((
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
