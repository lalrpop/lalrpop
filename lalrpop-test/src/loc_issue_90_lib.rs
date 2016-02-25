use loc_issue_90::parse_Expression2;
use util::expect_debug;

#[derive(Debug)]
pub enum Expr<'input> {
    Paren(usize, Box<Expr<'input>>, usize),
    Mul(usize, usize),
    Adjacent(usize, Box<Expr<'input>>, Box<Expr<'input>>, Box<Expr<'input>>, usize),
    Upgrade(usize, Box<Expr<'input>>, usize),
    Ident(usize, &'input str, usize),
    Maybe(usize, usize),
    Wacky(usize, usize),
    Wonky(usize, usize),
    Ref(usize, Box<Expr<'input>>, Box<Expr<'input>>, usize),
}

#[test]
fn loc_issue_90_wonky() {
    //                              0123456789abc
    let result = parse_Expression2("wonky * wonky");
    println!("{:#?}", result);
    expect_debug(result, r#"
Ok(
    Adjacent(
        0,
        Upgrade(
            0,
            Wonky(
                5,
                6
            ),
            6
        ),
        Mul(
            6,
            7
        ),
        Wonky(
            13,
            13
        ),
        13
    )
)
"#.trim());
}

#[test]
fn loc_issue_90_wacky() {
    let result = parse_Expression2("wacky");
    println!("{:#?}", result);
    expect_debug(result, r#"
Ok(
    Upgrade(
        0,
        Wacky(
            0,
            5
        ),
        5
    )
)
"#.trim());
}

#[test]
fn loc_issue_90_wacky_3() {
    //                              0123456789abc
    let result = parse_Expression2("wacky * wacky");
    println!("{:#?}", result);
    expect_debug(result, r#"
Ok(
    Adjacent(
        0,
        Upgrade(
            0,
            Wacky(
                0,
                5
            ),
            5
        ),
        Mul(
            6,
            7
        ),
        Wacky(
            8,
            13
        ),
        13
    )
)
"#.trim());
}

#[test]
fn loc_issue_90_maybe() {
    let result = parse_Expression2("& x");
    println!("{:#?}", result);
    expect_debug(result, r#"
Ok(
    Upgrade(
        0,
        Ref(
            0,
            Maybe(
                1,
                1
            ),
            Ident(
                2,
                "x",
                3
            ),
            3
        ),
        3
    )
)
"#.trim());
}

#[test]
fn loc_issue_90_test1() {
    let result = parse_Expression2("x * y");
    println!("{:#?}", result);
    expect_debug(result, r#"
Ok(
    Adjacent(
        0,
        Upgrade(
            0,
            Ident(
                0,
                "x",
                1
            ),
            1
        ),
        Mul(
            2,
            3
        ),
        Ident(
            4,
            "y",
            5
        ),
        5
    )
)
"#.trim());
}


#[test]
fn loc_issue_90_test2() {
    let result = parse_Expression2("(x*z) * y");
    println!("{:#?}", result);
    expect_debug(result, r#"
Ok(
    Adjacent(
        0,
        Upgrade(
            0,
            Paren(
                0,
                Adjacent(
                    1,
                    Upgrade(
                        1,
                        Ident(
                            1,
                            "x",
                            2
                        ),
                        2
                    ),
                    Mul(
                        2,
                        3
                    ),
                    Ident(
                        3,
                        "z",
                        4
                    ),
                    4
                ),
                5
            ),
            5
        ),
        Mul(
            6,
            7
        ),
        Ident(
            8,
            "y",
            9
        ),
        9
    )
)
"#.trim());
}

