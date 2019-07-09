#[derive(Debug)]
pub enum Stmt {
    Push(Int),
    Dup,
    Swap,
    Discard,

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Store,
    Load,

    Mark(String),
    Call(String),
    Jump(String), // Unconditional jump
    Jz(String),   // Jump if zero
    Js(String),   // Jump if negative

    Return,
    Exit,

    PrintChar,
    PrintNum,
    ReadChar,
    ReadNum,
}

pub fn label(digits: Vec<u8>) -> String {
    digits
        .into_iter()
        .map(|c| match c {
            0 => 's',
            1 => 't',
            _ => unreachable!(),
        })
        .collect()
}

pub fn number(digits: Vec<u8>) -> Int {
    assert!(digits.len() <= 64);

    let mut value = 0;
    for digit in digits {
        value <<= 1;
        value |= digit as i64;
    }
    value
}

pub type Int = i64;

pub struct Program {
    statements: Vec<Stmt>,
}

impl Program {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Program { statements }
    }

    pub fn dump(&self) {
        for stmt in self.statements.iter() {
            println!("{:?}", stmt);
        }
    }

    pub fn interpret(&self) {
        Interpreter::new(self)
            .execute()
            .unwrap_or_else(|err| println!("{}", err));
    }
}

struct Interpreter<'program> {
    source: &'program Vec<Stmt>,
    stack: Vec<Int>,
}

impl<'program> Interpreter<'program> {
    fn new(program: &'program Program) -> Self {
        Interpreter {
            source: &program.statements,
            stack: vec![],
        }
    }

    fn execute(&mut self) -> Result<(), String> {
        let mut pc = 0;
        while let Some(stmt) = self.source.get(pc) {
            match stmt {
                &Stmt::Push(n) => self.stack.push(n),

                &Stmt::PrintChar => {
                    let top = self.pop()?;
                    let c = num_to_char(top)?;
                    print!("{}", c);
                }

                &Stmt::Exit => return Ok(()),

                other => return Err({ format!("Unimplemented instruction: {:?}", other) }),
            }

            pc += 1;
        }

        Err(format!("Out of instructions! Program counter: {}", pc))
    }

    fn pop(&mut self) -> Result<Int, String> {
        match self.stack.pop() {
            Some(n) => Ok(n),
            None => Err(format!("Stack underflow")),
        }
    }
}

fn num_to_char(n: Int) -> Result<char, String> {
    use std::char::from_u32;

    if n < 0 {
        Err(format!("Can't cast negative int to char: {}", n))
    } else if n > u32::max_value() as i64 {
        Err(format!("Int is too huge to be a char: {}", n))
    } else {
        match from_u32(n as u32) {
            Some(c) => Ok(c),
            None => Err(format!("Invalid char: {}", n)),
        }
    }
}
