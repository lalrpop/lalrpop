use std::fmt::Debug;

pub trait ParseCallbacks: Eq + Debug {
    type Num: Eq + Debug + Into<Self::Term>;
    type Term: Eq + Debug;

    fn number(&mut self, i32) -> Self::Num;
}

#[derive(Eq, PartialEq, Debug)]
pub struct TestParseCallbacks;

#[derive(Eq, PartialEq, Debug)]
pub struct TestNum(pub i32);

#[derive(Eq, PartialEq, Debug)]
pub struct TestTerm(pub TestNum);

impl From<TestNum> for TestTerm {
    fn from(n: TestNum) -> Self {
        TestTerm(n)
    }
}

impl ParseCallbacks for TestParseCallbacks {
    type Num = TestNum;
    type Term = TestTerm;

    fn number(&mut self, n: i32) -> Self::Num {
        TestNum(n)
    }
}
