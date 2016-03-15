extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;
mod __parse__E {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;

    struct ReducedProduction {
        nonterminal: u32,
        symbol_count: u32,
    }

    const productions: [ReducedProduction; 11] = [
            ReducedProduction { nonterminal: 0, symbol_count: 3 },
            ReducedProduction { nonterminal: 1, symbol_count: 0 },
            ReducedProduction { nonterminal: 1, symbol_count: 1 },
            ReducedProduction { nonterminal: 2, symbol_count: 1 },
            ReducedProduction { nonterminal: 2, symbol_count: 2 },
            ReducedProduction { nonterminal: 3, symbol_count: 3 },
            ReducedProduction { nonterminal: 3, symbol_count: 4 },
            ReducedProduction { nonterminal: 3, symbol_count: 4 },
            ReducedProduction { nonterminal: 3, symbol_count: 5 },
            ReducedProduction { nonterminal: 4, symbol_count: 4 },
            ReducedProduction { nonterminal: 5, symbol_count: 1 },
    ];
    const action_row_0: &'static [i32] = &[0, 0, 0, 3, 0, 0, 0];
    const action_row_1: &'static [i32] = &[0, 0, 0, 0, 0, 0, -10];
    const action_row_2: &'static [i32] = &[0, 7, 8, 0, 0, 0, 0];
    const action_row_3: &'static [i32] = &[0, -3, -3, 0, 0, 0, 0];
    const action_row_4: &'static [i32] = &[0, 7, 8, 0, 0, 0, 0];
    const action_row_5: &'static [i32] = &[0, 0, 13, 0, 14, 0, 0];
    const action_row_6: &'static [i32] = &[0, 0, 0, 0, 0, 15, 0];
    const action_row_7: &'static [i32] = &[0, 0, 0, 0, 0, 16, 0];
    const action_row_8: &'static [i32] = &[0, -4, -4, 0, 0, 0, 0];
    const action_row_9: &'static [i32] = &[0, 0, 13, 0, 18, 0, 0];
    const action_row_10: &'static [i32] = &[0, 0, -3, 0, -3, 0, 0];
    const action_row_11: &'static [i32] = &[0, 0, 13, 0, 20, 0, 0];
    const action_row_12: &'static [i32] = &[0, 0, 0, 0, 0, 21, 0];
    const action_row_13: &'static [i32] = &[0, 0, 0, 0, 0, 0, -5];
    const action_row_14: &'static [i32] = &[0, 0, 0, 22, 0, 0, 0];
    const action_row_15: &'static [i32] = &[23, 0, 0, 0, 0, 0, 0];
    const action_row_16: &'static [i32] = &[0, 0, 13, 0, 24, 0, 0];
    const action_row_17: &'static [i32] = &[0, 0, 0, 0, 0, 0, -7];
    const action_row_18: &'static [i32] = &[0, 0, -4, 0, -4, 0, 0];
    const action_row_19: &'static [i32] = &[0, 0, 0, 0, 0, 0, -6];
    const action_row_20: &'static [i32] = &[25, 0, 0, 0, 0, 0, 0];
    const action_row_21: &'static [i32] = &[0, 0, 0, 0, 26, 0, 0];
    const action_row_22: &'static [i32] = &[0, -0, -0, 0, 0, 0, 0];
    const action_row_23: &'static [i32] = &[0, 0, 0, 0, 0, 0, -8];
    const action_row_24: &'static [i32] = &[0, 0, -0, 0, -0, 0, 0];
    const action_row_25: &'static [i32] = &[0, 0, -9, 0, -9, 0, 0];
    const actions: [&'static [i32]; 26] = [action_row_0, action_row_1, action_row_2, action_row_3, action_row_4, action_row_5, action_row_6, action_row_7, action_row_8, action_row_9, action_row_10, action_row_11, action_row_12, action_row_13, action_row_14, action_row_15, action_row_16, action_row_17, action_row_18, action_row_19, action_row_20, action_row_21, action_row_22, action_row_23, action_row_24, action_row_25];

    const goto_row_0: &'static [u32] = &[0, 0, 0, 1, 0, 0];
    const goto_row_1: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_2: &'static [u32] = &[3, 0, 4, 0, 5, 0];
    const goto_row_3: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_4: &'static [u32] = &[8, 0, 0, 0, 9, 0];
    const goto_row_5: &'static [u32] = &[10, 0, 11, 0, 0, 0];
    const goto_row_6: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_7: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_8: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_9: &'static [u32] = &[10, 0, 16, 0, 0, 0];
    const goto_row_10: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_11: &'static [u32] = &[18, 0, 0, 0, 0, 0];
    const goto_row_12: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_13: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_14: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_15: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_16: &'static [u32] = &[18, 0, 0, 0, 0, 0];
    const goto_row_17: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_18: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_19: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_20: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_21: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_22: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_23: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_24: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const goto_row_25: &'static [u32] = &[0, 0, 0, 0, 0, 0];
    const gotos: [&'static [u32]; 26] = [
goto_row_0, goto_row_1, goto_row_2, goto_row_3, goto_row_4, goto_row_5, goto_row_6, goto_row_7, goto_row_8, goto_row_9, goto_row_10, goto_row_11, goto_row_12, goto_row_13, goto_row_14, goto_row_15, goto_row_16, goto_row_17, goto_row_18, goto_row_19, goto_row_20, goto_row_21, goto_row_22, goto_row_23, goto_row_24, goto_row_25];

    fn terminal_to_index<
        'input,
    >(
        token: &(usize, &'input str),
    ) -> usize
    {
        match *token {
            (0, __tok0) => 0,
            (1, __tok0) => 1,
            (2, __tok0) => 2,
            (3, __tok0) => 3,
            (4, __tok0) => 4,
            (5, __tok0) => 5,
            _ => panic!("unuspported token"),
        }
    }
    pub fn parse_E<
        'input,
    >(
        input: &'input str,
    ) -> Result<(Vec<&'input str>, &'input str, Vec<&'input str>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __machine = Machine::new();
        __machine.execute_partial(&mut __tokens);
        Err(__ParseError::ExtraToken { token: __tokens.next().expect("no more tokens").unwrap() })
    }
    enum StackData<'input> {
        Empty,
        Terminal((usize, (usize, &'input str), usize)),
        Nt0(&'input str),
        Nt1(::std::vec::Vec<&'input str>),
        Nt2(::std::vec::Vec<&'input str>),
        Nt3((Vec<&'input str>, &'input str, Vec<&'input str>)),
        Nt4(&'input str),
        Nt5((Vec<&'input str>, &'input str, Vec<&'input str>)),
    }

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
                        59 => /* ';' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        102 ... 115 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        123 => /* '{' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        125 => /* '}' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 6;
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
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 120 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        122 => /* 'z' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 111 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        113 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 108 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        110 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
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
    (_, __0, _): (usize, (Vec<&'input str>, &'input str, Vec<&'input str>), usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    (__0, __1, __2)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<&'input str>
{
    vec![]
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> ::std::vec::Vec<&'input str>
{
    v
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, e, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __start1 = __1.2.clone();
    let __end1 = __2.0.clone();
    let __temp0 = __action4(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action4(
        input,
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action1(
        input,
        __0,
        __temp0,
        __1,
        __temp1,
        __2,
    )
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ::std::vec::Vec<&'input str>, usize),
    __3: (usize, &'input str, usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __start1 = __2.0.clone();
    let __end1 = __2.2.clone();
    let __temp0 = __action4(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action5(
        input,
        __2,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action1(
        input,
        __0,
        __temp0,
        __1,
        __temp1,
        __3,
    )
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<&'input str>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __start1 = __2.2.clone();
    let __end1 = __3.0.clone();
    let __temp0 = __action5(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action4(
        input,
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action1(
        input,
        __0,
        __temp0,
        __2,
        __temp1,
        __3,
    )
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<&'input str>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<&'input str>, usize),
    __4: (usize, &'input str, usize),
) -> (Vec<&'input str>, &'input str, Vec<&'input str>)
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __start1 = __3.0.clone();
    let __end1 = __3.2.clone();
    let __temp0 = __action5(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action5(
        input,
        __3,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action1(
        input,
        __0,
        __temp0,
        __2,
        __temp1,
        __4,
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
