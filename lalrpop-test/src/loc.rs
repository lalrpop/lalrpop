=======
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
>>>>>>> More progress
use self::__lalrpop_util::ParseError as __ParseError;
mod __parse__Items {
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;

    struct ReducedProduction {
        nonterminal: u32,
        symbol_count: u32,
    }

    const productions: [ReducedProduction; 7] = [
            ReducedProduction { nonterminal: 0, symbol_count: 0 },
            ReducedProduction { nonterminal: 1, symbol_count: 0 },
            ReducedProduction { nonterminal: 2, symbol_count: 0 },
            ReducedProduction { nonterminal: 2, symbol_count: 2 },
            ReducedProduction { nonterminal: 2, symbol_count: 2 },
            ReducedProduction { nonterminal: 3, symbol_count: 1 },
            ReducedProduction { nonterminal: 4, symbol_count: 1 },
    ];
    const action_row_0: &'static [i32] = &[-2, -2, -2];
    const action_row_1: &'static [i32] = &[4, 5, -6];
    const action_row_2: &'static [i32] = &[-3, -3, -3];
    const action_row_3: &'static [i32] = &[-5, -5, -5];
    const action_row_4: &'static [i32] = &[-4, -4, -4];
    const actions: [&'static [i32]; 5] = [action_row_0, action_row_1, action_row_2, action_row_3, action_row_4];

    const goto_row_0: &'static [u32] = &[0, 0, 1, 0, 0];
    const goto_row_1: &'static [u32] = &[0, 0, 0, 2, 0];
    const goto_row_2: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_3: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_4: &'static [u32] = &[0, 0, 0, 0, 0];
    const gotos: [&'static [u32]; 5] = [
goto_row_0, goto_row_1, goto_row_2, goto_row_3, goto_row_4];

    fn terminal_to_index<
    >(
        token: &Tok,
    ) -> usize
    {
        match *token {
            Tok::Plus => 0,
            Tok::Minus => 1,
            _ => panic!("unuspported token"),
        }
    }
    pub fn parse_Items<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<Vec<(usize, usize)>, __ParseError<usize,Tok,()>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __machine = Machine::new();
        __machine.execute_partial(&mut __tokens);
        Err(__ParseError::ExtraToken { token: __tokens.next().expect("no more tokens").unwrap() })
    }
    enum StackData {
        Empty,
        Terminal((usize, Tok, usize)),
        Nt0(usize),
        Nt1(usize),
        Nt2(Vec<(usize, usize)>),
        Nt3((usize, usize)),
        Nt4(Vec<(usize, usize)>),
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
    //     Items = (*) [EOF]
    //     Items = (*) ["+"]
    //     Items = (*) ["-"]
    //     Items = (*) Items Spanned<"+"> [EOF]
    //     Items = (*) Items Spanned<"+"> ["+"]
    //     Items = (*) Items Spanned<"+"> ["-"]
    //     Items = (*) Items "-" [EOF]
    //     Items = (*) Items "-" ["+"]
    //     Items = (*) Items "-" ["-"]
    //     __Items = (*) Items [EOF]
    //
    //     EOF -> Reduce(Items =  => ActionFn(9);)
    //     "+" -> Reduce(Items =  => ActionFn(9);)
    //     "-" -> Reduce(Items =  => ActionFn(9);)
    //
    //     Items -> S1
    pub fn __state0<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __start: usize = ::std::default::Default::default();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action9(&__start, &__end);
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
    //     Kind = None
    //     AllInputs = [Items]
    //     OptionalInputs = []
    //     FixedInputs = [Items]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Items = Items (*) Spanned<"+"> [EOF]
    //     Items = Items (*) Spanned<"+"> ["+"]
    //     Items = Items (*) Spanned<"+"> ["-"]
    //     Items = Items (*) "-" [EOF]
    //     Items = Items (*) "-" ["+"]
    //     Items = Items (*) "-" ["-"]
    //     Spanned<"+"> = (*) "+" [EOF]
    //     Spanned<"+"> = (*) "+" ["+"]
    //     Spanned<"+"> = (*) "+" ["-"]
    //     __Items = Items (*) [EOF]
    //
    //     EOF -> Reduce(__Items = Items => ActionFn(0);)
    //     "+" -> Shift(S3)
    //     "-" -> Shift(S4)
    //
    //     Spanned<"+"> -> S2
    pub fn __state1<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: (usize, Vec<(usize, usize)>, usize),
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__custom1(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let __sym1 = (__loc1, (__tok), __loc2);
                __result = try!(__custom2(__tokens, __sym0, __sym1));
                return Ok(__result);
            }
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(__sym0);
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
>>>>>>> Port lalrpop-test to use new `Configuration` value
        }
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6
        fn top_state(&self) -> usize {
            *self.state_stack.last().expect("state stack is empty!") as usize
        }
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
        fn dispatch_action(&self, nonterminal: u32, args: Vec<StackData>) -> StackData {
            StackData::Empty
        }
        fn reduce(&mut self, production: &ReducedProduction) {
            let mut args = Vec::new();
            for _ in 0 .. production.symbol_count {
                args.push(self.data_stack.pop().expect("popped data stack"));
                self.state_stack.pop();
=======
        return Ok(__result);
=======
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Spanned_3c_22_2b_22_3e(__sym1) => {
                    __result = try!(__custom0(__tokens, __lookahead, __sym0, __sym1));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
>>>>>>> update test output
    }

    // Custom 0
    //    Reduce Items = Items, Spanned<"+"> => ActionFn(2);
    pub fn __custom0<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: (usize, Vec<(usize, usize)>, usize),
        __sym1: (usize, (usize, usize), usize),
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action2(__sym0, __sym1);
        let __nt = __Nonterminal::Items((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 1
    //    Reduce Spanned<"+"> = "+" => ActionFn(10);
    pub fn __custom1<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: (usize, Tok, usize),
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(__sym0);
                let __nt = __Nonterminal::Spanned_3c_22_2b_22_3e((
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
            let top_state = self.top_state();
            self.state_stack.push(gotos[top_state][production.nonterminal as usize]);
            let res = self.dispatch_action(production.nonterminal, args);
            self.data_stack.push(res);
        }
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
        fn execute_partial<
            __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
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
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10(__sym0);
        let __nt = __Nonterminal::Spanned_3c_22_2b_22_3e((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
>>>>>>> update test output
    }

    // Custom 2
    //    Reduce Items = Items, "-" => ActionFn(3);
    pub fn __custom2<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: (usize, Vec<(usize, usize)>, usize),
        __sym1: (usize, Tok, usize),
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,()>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
<<<<<<< 59cc89ee5a918263721461655ed742ebfeaf2fa6
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action3(__sym0, __sym1);
                let __nt = __Nonterminal::Items((
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
        let __nt = super::__action3(__sym0, __sym1);
        let __nt = __Nonterminal::Items((
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
            __TOKENS: Iterator<Item=Result<(usize, Tok, usize),()>>,
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
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, usize, usize),
) -> Vec<(usize, usize)>
{
    vec![(__0, __1)]
}

pub fn __action2<
>(
    (_, v, _): (usize, Vec<(usize, usize)>, usize),
    (_, e, _): (usize, (usize, usize), usize),
) -> Vec<(usize, usize)>
{
    {
        let mut v = v;
        v.push(e);
        v
    }
}

pub fn __action3<
>(
    (_, v, _): (usize, Vec<(usize, usize)>, usize),
    (_, _, _): (usize, Tok, usize),
) -> Vec<(usize, usize)>
{
    v
}

pub fn __action4<
>(
    (_, __0, _): (usize, usize, usize),
    (_, _, _): (usize, Tok, usize),
    (_, __1, _): (usize, usize, usize),
) -> (usize, usize)
{
    /* spanned */ (__0, __1)
}

pub fn __action5<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

pub fn __action6<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

pub fn __action7<
>(
    __0: (usize, usize, usize),
) -> Vec<(usize, usize)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action6(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
        __0,
    )
}

pub fn __action8<
>(
    __0: (usize, Tok, usize),
    __1: (usize, usize, usize),
) -> (usize, usize)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action6(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __temp0,
        __0,
        __1,
    )
}

pub fn __action9<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(usize, usize)>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action5(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        __temp0,
    )
}

pub fn __action10<
>(
    __0: (usize, Tok, usize),
) -> (usize, usize)
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action5(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        __0,
        __temp0,
    )
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),Self::Error>;
}

impl<> __ToTriple<> for (usize, Tok, usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),()> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, Tok, usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),()> {
        value
    }
}
