use util::tok::Tok;
use lalrpop_util::ParseError;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;
mod __parse__Items {
    use util::tok::Tok;
    use lalrpop_util::ParseError;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;

    struct ReducedProduction {
        nonterminal: u32,
        symbol_count: u32,
    }

    const productions: [ReducedProduction; 4] = [
            ReducedProduction { nonterminal: 0, symbol_count: 0 },
            ReducedProduction { nonterminal: 0, symbol_count: 2 },
            ReducedProduction { nonterminal: 0, symbol_count: 2 },
            ReducedProduction { nonterminal: 1, symbol_count: 1 },
    ];
    const action_row_0: &'static [i32] = &[-0, -0, -0];
    const action_row_1: &'static [i32] = &[3, 4, -3];
    const action_row_2: &'static [i32] = &[-1, -1, -1];
    const action_row_3: &'static [i32] = &[-2, -2, -2];
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
            Tok::Plus => 0,
            Tok::Minus => 1,
            _ => panic!("unuspported token"),
        }
    }
    pub fn parse_Items<
        __TOKEN: __ToTriple<Error=char>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<Vec<(usize, usize)>, __ParseError<usize,Tok,char>>
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
        Nt0(Vec<(usize, usize)>),
        Nt1(Vec<(usize, usize)>),
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
            __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
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

pub fn __action0<
>(
    (_, __0, _): (usize, Vec<(usize, usize)>, usize),
) -> Vec<(usize, usize)>
{
    (__0)
}

pub fn __action1<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(usize, usize)>
{
    vec![]
}

pub fn __action2<
>(
    (_, __0, _): (usize, Vec<(usize, usize)>, usize),
    (_, __1, _): (usize, Tok, usize),
) -> Result<Vec<(usize, usize)>,__ParseError<usize,Tok,char>>
{
    Err(ParseError::User { error: '+' })
}

pub fn __action3<
>(
    (_, v, _): (usize, Vec<(usize, usize)>, usize),
    (_, _, _): (usize, Tok, usize),
) -> Result<Vec<(usize, usize)>,__ParseError<usize,Tok,char>>
{
    Ok(v)
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),Self::Error>;
}

impl<> __ToTriple<> for (usize, Tok, usize) {
    type Error = char;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),char> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, Tok, usize),char> {
    type Error = char;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),char> {
        value
    }
}
