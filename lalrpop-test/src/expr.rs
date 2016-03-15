use util::tok::Tok;
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
