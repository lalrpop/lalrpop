use crate::arg_pattern::FooParser;
use crate::util::expect_debug;

#[test]
fn arg_multiple_pattern() {
    let result = FooParser::new().parse("A B C");
    println!("{result:#?}");
    expect_debug(
        result,
        r#"
Ok(
    "ABC"
)
"#
        .trim(),
    );
}
