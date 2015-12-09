use super::*;
use test_util::*;

#[test]
fn parse_range_and_star() {
    let r = parse_regex("[a-c][a-c0-3]*").unwrap();
    expect_debug(r, "(a|b|c)(a|b|c|0|1|2|3)*")
}

#[test]
fn parse_neg() {
    let r1 = parse_regex(r"[^a-c]").unwrap();
    expect_debug(r1, r"\!(a|b|c)");

    //let r2 = parse_regex(r"\!(a|b|c)").unwrap();
    //compare(r1, r2);

}
#[test]
fn parse_unclosed_group() {
    parse_regex(r"(123").unwrap_err();
}
#[test]
fn alt_oom() {
    parse_regex(r"(%%|[^%])+").unwrap();
}
