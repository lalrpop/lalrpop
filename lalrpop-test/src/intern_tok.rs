extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;
mod __parse__Items {
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;

    struct ReducedProduction {
        nonterminal: u32,
        symbol_count: u32,
    }

<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
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
=======
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
    //   EOF -> Reduce(Items =  => ActionFn(9);)
    //   "+" -> Reduce(Items =  => ActionFn(9);)
    //   "-" -> Reduce(Items =  => ActionFn(9);)
    //
    //   Items -> S1
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
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) => {
                let __start: usize = ::std::default::Default::default();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action9(input, &__start, &__end);
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
                __Nonterminal::Items(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Items = Items (*) Spanned<"+"> [EOF]
    //   Items = Items (*) Spanned<"+"> ["+"]
    //   Items = Items (*) Spanned<"+"> ["-"]
    //   Items = Items (*) "-" [EOF]
    //   Items = Items (*) "-" ["+"]
    //   Items = Items (*) "-" ["-"]
    //   Spanned<"+"> = (*) "+" [EOF]
    //   Spanned<"+"> = (*) "+" ["+"]
    //   Spanned<"+"> = (*) "+" ["-"]
    //   __Items = Items (*) [EOF]
    //
    //   EOF -> Reduce(__Items = Items => ActionFn(0);)
    //   "+" -> Shift(S3)
    //   "-" -> Shift(S4)
    //
    //   Spanned<"+"> -> S2
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<(usize, usize)>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state3(input, __tokens, __sym1));
            }
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state4(input, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __nt = __Nonterminal::____Items((
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
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Spanned_3c_22_2b_22_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 2
    //   Items = Items Spanned<"+"> (*) [EOF]
    //   Items = Items Spanned<"+"> (*) ["+"]
    //   Items = Items Spanned<"+"> (*) ["-"]
    //
    //   EOF -> Reduce(Items = Items, Spanned<"+"> => ActionFn(2);)
    //   "+" -> Reduce(Items = Items, Spanned<"+"> => ActionFn(2);)
    //   "-" -> Reduce(Items = Items, Spanned<"+"> => ActionFn(2);)
    //
    pub fn __state2<
>>>>>>> Port lalrpop-test to use new `Configuration` value
        'input,
    >(
        token: &(usize, &'input str),
    ) -> usize
    {
        match *token {
            (0, __tok0) => 0,
            (1, __tok0) => 1,
            _ => panic!("unuspported token"),
        }
    }
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
    pub fn parse_Items<
=======

    // State 3
    //   Spanned<"+"> = "+" (*) [EOF]
    //   Spanned<"+"> = "+" (*) ["+"]
    //   Spanned<"+"> = "+" (*) ["-"]
    //
    //   EOF -> Reduce(Spanned<"+"> = "+" => ActionFn(10);)
    //   "+" -> Reduce(Spanned<"+"> = "+" => ActionFn(10);)
    //   "-" -> Reduce(Spanned<"+"> = "+" => ActionFn(10);)
    //
    pub fn __state3<
>>>>>>> Port lalrpop-test to use new `Configuration` value
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<(usize, usize)>, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __machine = Machine::new();
        __machine.execute_partial(&mut __tokens);
        Err(__ParseError::ExtraToken { token: __tokens.next().expect("no more tokens").unwrap() })
    }
    enum StackData<'input> {
        Empty,
        Terminal((usize, (usize, &'input str), usize)),
        Nt0(usize),
        Nt1(usize),
        Nt2(Vec<(usize, usize)>),
        Nt3((usize, usize)),
        Nt4(Vec<(usize, usize)>),
    }

<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
    struct Machine<'input> {
        state_stack: Vec<u32>,
        data_stack: Vec<StackData<'input>>
    }
    impl<'input> Machine<'input> {
        fn new() -> Machine<'input> {
            Machine { state_stack: Vec::new(), data_stack: Vec::new() }
        }
        fn top_state(&self) -> usize {
            *self.state_stack.last().expect("state stack is empty!") as usize
        }
        fn dispatch_action(&self, nonterminal: u32, args: Vec<StackData<'input>>) -> StackData<'input> {
            StackData::Empty
        }
        fn reduce(&mut self, production: &ReducedProduction) {
            let mut args = Vec::new();
            for _ in 0 .. production.symbol_count {
                args.push(self.data_stack.pop().expect("popped data stack"));
                self.state_stack.pop();
=======
    // State 4
    //   Items = Items "-" (*) [EOF]
    //   Items = Items "-" (*) ["+"]
    //   Items = Items "-" (*) ["-"]
    //
    //   EOF -> Reduce(Items = Items, "-" => ActionFn(3);)
    //   "+" -> Reduce(Items = Items, "-" => ActionFn(3);)
    //   "-" -> Reduce(Items = Items, "-" => ActionFn(3);)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Vec<(usize, usize)>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
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
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action3(input, __sym0, __sym1);
                let __nt = __Nonterminal::Items((
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
                    match __ch as u32 {
                        43 => /* '+' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        45 => /* '-' */ {
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
    (_, __0, _): (usize, Vec<(usize, usize)>, usize),
) -> Vec<(usize, usize)>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, usize, usize),
) -> Vec<(usize, usize)>
{
    vec![(__0, __1)]
}

pub fn __action2<
    'input,
>(
    input: &'input str,
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
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, Vec<(usize, usize)>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<(usize, usize)>
{
    v
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, usize, usize),
) -> (usize, usize)
{
    (__0, __1)
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: (usize, usize, usize),
) -> Vec<(usize, usize)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action6(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
        __0,
    )
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> (usize, usize)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action6(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __temp0,
        __0,
        __1,
    )
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(usize, usize)>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action5(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __temp0,
    )
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> (usize, usize)
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action5(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __0,
        __temp0,
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
