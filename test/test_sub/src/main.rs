extern crate util;

mod sub;

#[cfg(not(test))]
fn main() {
    util::main(sub::parse_S);
}

#[test]
fn test1() {
    util::test(sub::parse_S, "22 - 3", 22 - 3);
}

#[test]
fn test2() {
    util::test(sub::parse_S, "22 - (3 - 5)", 22 - (3 - 5));
}

#[test]
fn test3() {
    util::test(sub::parse_S, "22 - (3 - 5) - 13", 22 - (3 - 5) - 13);
}
