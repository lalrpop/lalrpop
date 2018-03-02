use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node<'ast> {
    Value(i32),
    Binary {
        op: Op,
        l: &'ast Node<'ast>,
        r: &'ast Node<'ast>,
    },
    Reduce(Op, Vec<&'ast Node<'ast>>),
    Paren(&'ast Node<'ast>),
}

pub struct Arena<'ast> {
    data: RefCell<Vec<Box<Node<'ast>>>>,
}

impl<'ast> Arena<'ast> {
    pub fn new() -> Arena<'ast> {
        Arena {
            data: RefCell::new(vec![]),
        }
    }

    pub fn alloc(&'ast self, n: Node<'ast>) -> &'ast Node<'ast> {
        let b = Box::new(n);
        let p: *const Node<'ast> = &*b;
        self.data.borrow_mut().push(b);
        unsafe { &*p }
    }
}
