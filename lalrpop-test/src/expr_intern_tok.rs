use std::str::FromStr;
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;
mod __parse__Expr {
    use std::str::FromStr;
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;

    struct ReducedProduction {
        nonterminal: u32,
        symbol_count: u32,
    }

    const productions: [ReducedProduction; 10] = [
            ReducedProduction { nonterminal: 0, symbol_count: 3 },
            ReducedProduction { nonterminal: 0, symbol_count: 3 },
            ReducedProduction { nonterminal: 0, symbol_count: 1 },
            ReducedProduction { nonterminal: 1, symbol_count: 3 },
            ReducedProduction { nonterminal: 1, symbol_count: 3 },
            ReducedProduction { nonterminal: 1, symbol_count: 1 },
            ReducedProduction { nonterminal: 2, symbol_count: 1 },
            ReducedProduction { nonterminal: 3, symbol_count: 1 },
            ReducedProduction { nonterminal: 3, symbol_count: 3 },
            ReducedProduction { nonterminal: 4, symbol_count: 1 },
    ];
    const action_row_0: &'static [i32] = &[6, 0, 0, 0, 0, 0, 7, 0];
    const action_row_1: &'static [i32] = &[0, 0, 0, 8, 9, 0, 0, -9];
    const action_row_2: &'static [i32] = &[0, 0, 10, -2, -2, 11, 0, -2];
    const action_row_3: &'static [i32] = &[0, 0, -7, -7, -7, -7, 0, -7];
    const action_row_4: &'static [i32] = &[0, 0, -5, -5, -5, -5, 0, -5];
    const action_row_5: &'static [i32] = &[16, 0, 0, 0, 0, 0, 17, 0];
    const action_row_6: &'static [i32] = &[0, 0, -6, -6, -6, -6, 0, -6];
    const action_row_7: &'static [i32] = &[6, 0, 0, 0, 0, 0, 7, 0];
    const action_row_8: &'static [i32] = &[6, 0, 0, 0, 0, 0, 7, 0];
    const action_row_9: &'static [i32] = &[6, 0, 0, 0, 0, 0, 7, 0];
    const action_row_10: &'static [i32] = &[6, 0, 0, 0, 0, 0, 7, 0];
    const action_row_11: &'static [i32] = &[0, 22, 0, 23, 24, 0, 0, 0];
    const action_row_12: &'static [i32] = &[0, -2, 25, -2, -2, 26, 0, 0];
    const action_row_13: &'static [i32] = &[0, -7, -7, -7, -7, -7, 0, 0];
    const action_row_14: &'static [i32] = &[0, -5, -5, -5, -5, -5, 0, 0];
    const action_row_15: &'static [i32] = &[16, 0, 0, 0, 0, 0, 17, 0];
    const action_row_16: &'static [i32] = &[0, -6, -6, -6, -6, -6, 0, 0];
    const action_row_17: &'static [i32] = &[0, 0, 10, -1, -1, 11, 0, -1];
    const action_row_18: &'static [i32] = &[0, 0, 10, -0, -0, 11, 0, -0];
    const action_row_19: &'static [i32] = &[0, 0, -3, -3, -3, -3, 0, -3];
    const action_row_20: &'static [i32] = &[0, 0, -4, -4, -4, -4, 0, -4];
    const action_row_21: &'static [i32] = &[0, 0, -8, -8, -8, -8, 0, -8];
    const action_row_22: &'static [i32] = &[16, 0, 0, 0, 0, 0, 17, 0];
    const action_row_23: &'static [i32] = &[16, 0, 0, 0, 0, 0, 17, 0];
    const action_row_24: &'static [i32] = &[16, 0, 0, 0, 0, 0, 17, 0];
    const action_row_25: &'static [i32] = &[16, 0, 0, 0, 0, 0, 17, 0];
    const action_row_26: &'static [i32] = &[0, 32, 0, 23, 24, 0, 0, 0];
    const action_row_27: &'static [i32] = &[0, -1, 25, -1, -1, 26, 0, 0];
    const action_row_28: &'static [i32] = &[0, -0, 25, -0, -0, 26, 0, 0];
    const action_row_29: &'static [i32] = &[0, -3, -3, -3, -3, -3, 0, 0];
    const action_row_30: &'static [i32] = &[0, -4, -4, -4, -4, -4, 0, 0];
    const action_row_31: &'static [i32] = &[0, -8, -8, -8, -8, -8, 0, 0];
    const actions: [&'static [i32]; 32] = [action_row_0, action_row_1, action_row_2, action_row_3, action_row_4, action_row_5, action_row_6, action_row_7, action_row_8, action_row_9, action_row_10, action_row_11, action_row_12, action_row_13, action_row_14, action_row_15, action_row_16, action_row_17, action_row_18, action_row_19, action_row_20, action_row_21, action_row_22, action_row_23, action_row_24, action_row_25, action_row_26, action_row_27, action_row_28, action_row_29, action_row_30, action_row_31];

    const goto_row_0: &'static [u32] = &[1, 2, 3, 4, 0];
    const goto_row_1: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_2: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_3: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_4: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_5: &'static [u32] = &[11, 12, 13, 14, 0];
    const goto_row_6: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_7: &'static [u32] = &[0, 17, 3, 4, 0];
    const goto_row_8: &'static [u32] = &[0, 18, 3, 4, 0];
    const goto_row_9: &'static [u32] = &[0, 0, 3, 19, 0];
    const goto_row_10: &'static [u32] = &[0, 0, 3, 20, 0];
    const goto_row_11: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_12: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_13: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_14: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_15: &'static [u32] = &[26, 12, 13, 14, 0];
    const goto_row_16: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_17: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_18: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_19: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_20: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_21: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_22: &'static [u32] = &[0, 27, 13, 14, 0];
    const goto_row_23: &'static [u32] = &[0, 28, 13, 14, 0];
    const goto_row_24: &'static [u32] = &[0, 0, 13, 29, 0];
    const goto_row_25: &'static [u32] = &[0, 0, 13, 30, 0];
    const goto_row_26: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_27: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_28: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_29: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_30: &'static [u32] = &[0, 0, 0, 0, 0];
    const goto_row_31: &'static [u32] = &[0, 0, 0, 0, 0];
    const gotos: [&'static [u32]; 32] = [
goto_row_0, goto_row_1, goto_row_2, goto_row_3, goto_row_4, goto_row_5, goto_row_6, goto_row_7, goto_row_8, goto_row_9, goto_row_10, goto_row_11, goto_row_12, goto_row_13, goto_row_14, goto_row_15, goto_row_16, goto_row_17, goto_row_18, goto_row_19, goto_row_20, goto_row_21, goto_row_22, goto_row_23, goto_row_24, goto_row_25, goto_row_26, goto_row_27, goto_row_28, goto_row_29, goto_row_30, goto_row_31];

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
            (6, __tok0) => 6,
            _ => panic!("unuspported token"),
        }
    }
    pub fn parse_Expr<
        'input,
    >(
        scale: i32,
        input: &'input str,
    ) -> Result<i32, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __machine = Machine::new();
        __machine.execute_partial(&mut __tokens);
        Err(__ParseError::ExtraToken { token: __tokens.next().expect("no more tokens").unwrap() })
    }
    enum StackData<'input> {
        Empty,
        Terminal((usize, (usize, &'input str), usize)),
        Nt0(i32),
        Nt1(i32),
        Nt2(i32),
        Nt3(i32),
        Nt4(i32),
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
pub use self::__parse__Expr::parse_Expr;
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
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
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
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
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
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
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
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    (__0)
}

pub fn __action1<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l - r
}

pub fn __action2<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l + r
}

pub fn __action3<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    (__0)
}

pub fn __action4<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l * r
}

pub fn __action5<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l / r
}

pub fn __action6<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    (__0)
}

pub fn __action7<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, n, _): (usize, i32, usize),
) -> i32
{
    n * scale
}

pub fn __action8<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
) -> i32
{
    (__0)
}

pub fn __action9<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
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
