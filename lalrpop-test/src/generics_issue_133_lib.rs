pub enum Token<'input> {
    Ident(&'input str)
}

pub trait Id {}

pub enum Expr<T: Id> {
    Ident(T),
}

pub trait Interner<T: Id> {
    fn intern(&mut self, src: &str) -> T;
}
