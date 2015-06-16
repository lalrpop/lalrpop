use regex::Regex;
use std::fmt::Debug;

thread_local! {
    static SPAN: Regex =
        Regex::new(r"Span\([0-9 ,]*\)").unwrap()
}

pub fn compare<D:Debug>(actual: D, expected: D) {
    println!("Actual:");
    println!("{:#?}", actual);
    println!("Expected:");
    println!("{:#?}", expected);

    let actual = format!("{:?}", actual);
    let expected = format!("{:?}", expected);

    SPAN.with(|span| {
        let actual = span.replace_all(&actual, "Span(..)");
        let expected = span.replace_all(&expected, "Span(..)");
        assert_eq!(actual, expected);
    });
}

