use diff;
use regex::Regex;
use std::fmt::{Debug, Formatter, Error};

thread_local! {
    static SPAN: Regex =
        Regex::new(r"Span\([0-9 ,]*\)").unwrap()
}

struct ExpectedDebug<'a>(&'a str);

impl<'a> Debug for ExpectedDebug<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self.0)
    }
}

pub fn expect_debug<D:Debug>(actual: D, expected: &str) {
    compare(ExpectedDebug(&format!("{:#?}", actual)),
            ExpectedDebug(expected))
}

pub fn compare<D:Debug,E:Debug>(actual: D, expected: E) {
    let actual_s = format!("{:?}", actual);
    let expected_s = format!("{:?}", expected);

    SPAN.with(|span| {
        let actual_s = span.replace_all(&actual_s, "Span(..)");
        let expected_s = span.replace_all(&expected_s, "Span(..)");
        if actual_s != expected_s {
            let actual_s = format!("{:#?}", actual);
            let expected_s = format!("{:#?}", expected);

            for diff in diff::lines(&actual_s, &expected_s) {
                match diff {
                    diff::Result::Left(l)    => println!("a {}", l),
                    diff::Result::Both(l, _) => println!("  {}", l),
                    diff::Result::Right(r)   => println!("e {}", r)
                }
            }

            assert!(false);
        }
    });
}

