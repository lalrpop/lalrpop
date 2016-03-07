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

<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
    struct Machine {
        state_stack: Vec<u32>,
        data_stack: Vec<StackData>
    }
    impl Machine {
        fn new() -> Machine {
            Machine { state_stack: Vec::new(), data_stack: Vec::new() }
=======
    // State 0
    //   Items = (*) [EOF]
    //   Items = (*) ["+"]
    //   Items = (*) ["-"]
    //   Items = (*) Items "+" [EOF]
    //   Items = (*) Items "+" ["+"]
    //   Items = (*) Items "+" ["-"]
    //   Items = (*) Items "-" [EOF]
    //   Items = (*) Items "-" ["+"]
    //   Items = (*) Items "-" ["-"]
    //   __Items = (*) Items [EOF]
    //
    //   EOF -> Reduce(Items =  => ActionFn(1);)
    //   "+" -> Reduce(Items =  => ActionFn(1);)
    //   "-" -> Reduce(Items =  => ActionFn(1);)
    //
    //   Items -> S1
    pub fn __state0<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __start: usize = ::std::default::Default::default();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action1(&__start, &__end);
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
>>>>>>> Port lalrpop-test to use new `Configuration` value
        }
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
    }

    // State 1
    //   Items = Items (*) "+" [EOF]
    //   Items = Items (*) "+" ["+"]
    //   Items = Items (*) "+" ["-"]
    //   Items = Items (*) "-" [EOF]
    //   Items = Items (*) "-" ["+"]
    //   Items = Items (*) "-" ["-"]
    //   __Items = Items (*) [EOF]
    //
    //   EOF -> Reduce(__Items = Items => ActionFn(0);)
    //   "+" -> Shift(S2)
    //   "-" -> Shift(S3)
    //
    pub fn __state1<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok, usize)>,
        __sym0: &mut Option<(usize, Vec<(usize, usize)>, usize)>,
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state2(__tokens, __sym0, __sym1));
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state3(__tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(__sym0);
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
        return Ok(__result);
    }

    // State 2
    //   Items = Items "+" (*) [EOF]
    //   Items = Items "+" (*) ["+"]
    //   Items = Items "+" (*) ["-"]
    //
    //   EOF -> Reduce(Items = Items, "+" => ActionFn(2);)
    //   "+" -> Reduce(Items = Items, "+" => ActionFn(2);)
    //   "-" -> Reduce(Items = Items, "+" => ActionFn(2);)
    //
    pub fn __state2<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Vec<(usize, usize)>, usize)>,
        __sym1: &mut Option<(usize, Tok, usize)>,
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = try!(super::__action2(__sym0, __sym1));
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
            let top_state = self.top_state();
            self.state_stack.push(gotos[top_state][production.nonterminal as usize]);
            let res = self.dispatch_action(production.nonterminal, args);
            self.data_stack.push(res);
        }
<<<<<<< 0f2545c366e7d96b9d69553c96cba06c64fc5ee3
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
=======
    }

    // State 3
    //   Items = Items "-" (*) [EOF]
    //   Items = Items "-" (*) ["+"]
    //   Items = Items "-" (*) ["-"]
    //
    //   EOF -> Reduce(Items = Items, "-" => ActionFn(3);)
    //   "+" -> Reduce(Items = Items, "-" => ActionFn(3);)
    //   "-" -> Reduce(Items = Items, "-" => ActionFn(3);)
    //
    pub fn __state3<
        __TOKENS: Iterator<Item=Result<(usize, Tok, usize),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Vec<(usize, usize)>, usize)>,
        __sym1: &mut Option<(usize, Tok, usize)>,
    ) -> Result<(Option<(usize, Tok, usize)>, __Nonterminal<>), __ParseError<usize,Tok,char>>
    {
        let mut __result: (Option<(usize, Tok, usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = try!(super::__action3(__sym0, __sym1));
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
