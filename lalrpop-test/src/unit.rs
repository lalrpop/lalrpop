#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_5c_5cd_2b_22_23(&'input str),
        NtExpr(()),
        NtFactor(()),
        NtTerm(()),
        Nt____Expr(()),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        //     Expr = (*) Expr "+" Factor ["+"]
        //     Expr = (*) Expr "+" Factor ["-"]
        //     Expr = (*) Expr "+" Factor [EOF]
        //     Expr = (*) Expr "-" Factor ["+"]
        //     Expr = (*) Expr "-" Factor ["-"]
        //     Expr = (*) Expr "-" Factor [EOF]
        //     Expr = (*) Factor ["+"]
        //     Expr = (*) Factor ["-"]
        //     Expr = (*) Factor [EOF]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["+"]
        //     Factor = (*) Factor "*" Term ["-"]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "*" Term [EOF]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["+"]
        //     Factor = (*) Factor "/" Term ["-"]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Factor "/" Term [EOF]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["+"]
        //     Factor = (*) Term ["-"]
        //     Factor = (*) Term ["/"]
        //     Factor = (*) Term [EOF]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+"]
        //     Term = (*) "(" Expr ")" ["-"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) "(" Expr ")" [EOF]
        //     Term = (*) r#"\\d+"# ["*"]
        //     Term = (*) r#"\\d+"# ["+"]
        //     Term = (*) r#"\\d+"# ["-"]
        //     Term = (*) r#"\\d+"# ["/"]
        //     Term = (*) r#"\\d+"# [EOF]
        //     __Expr = (*) Expr [EOF]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        6, // on r#"\\d+"#, goto 5
        // State 1
        //     Expr = Expr (*) "+" Factor ["+", "-", EOF]
        //     Expr = Expr (*) "-" Factor ["+", "-", EOF]
        //     __Expr = Expr (*) [EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        7, // on "+", goto 6
        8, // on "-", goto 7
        0, // on "/", error
        0, // on r#"\\d+"#, error
        // State 2
        //     Expr = Factor (*) ["+", "-", EOF]
        //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
        //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        9, // on "*", goto 8
        -3, // on "+", reduce `Expr = Factor => ActionFn(3);`
        -3, // on "-", reduce `Expr = Factor => ActionFn(3);`
        10, // on "/", goto 9
        0, // on r#"\\d+"#, error
        // State 3
        //     Factor = Term (*) ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        -6, // on "*", reduce `Factor = Term => ActionFn(6);`
        -6, // on "+", reduce `Factor = Term => ActionFn(6);`
        -6, // on "-", reduce `Factor = Term => ActionFn(6);`
        -6, // on "/", reduce `Factor = Term => ActionFn(6);`
        0, // on r#"\\d+"#, error
        // State 4
        //     Expr = (*) Expr "+" Factor [")"]
        //     Expr = (*) Expr "+" Factor ["+"]
        //     Expr = (*) Expr "+" Factor ["-"]
        //     Expr = (*) Expr "-" Factor [")"]
        //     Expr = (*) Expr "-" Factor ["+"]
        //     Expr = (*) Expr "-" Factor ["-"]
        //     Expr = (*) Factor [")"]
        //     Expr = (*) Factor ["+"]
        //     Expr = (*) Factor ["-"]
        //     Factor = (*) Factor "*" Term [")"]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["+"]
        //     Factor = (*) Factor "*" Term ["-"]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term [")"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["+"]
        //     Factor = (*) Factor "/" Term ["-"]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term [")"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["+"]
        //     Factor = (*) Term ["-"]
        //     Factor = (*) Term ["/"]
        //     Term = (*) "(" Expr ")" [")"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+"]
        //     Term = (*) "(" Expr ")" ["-"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = "(" (*) Expr ")" ["*", "+", "-", "/", EOF]
        //     Term = (*) r#"\\d+"# [")"]
        //     Term = (*) r#"\\d+"# ["*"]
        //     Term = (*) r#"\\d+"# ["+"]
        //     Term = (*) r#"\\d+"# ["-"]
        //     Term = (*) r#"\\d+"# ["/"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        15, // on r#"\\d+"#, goto 14
        // State 5
        //     Term = r#"\\d+"# (*) ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        -7, // on "*", reduce `Term = r#"\\d+"# => ActionFn(7);`
        -7, // on "+", reduce `Term = r#"\\d+"# => ActionFn(7);`
        -7, // on "-", reduce `Term = r#"\\d+"# => ActionFn(7);`
        -7, // on "/", reduce `Term = r#"\\d+"# => ActionFn(7);`
        0, // on r#"\\d+"#, error
        // State 6
        //     Expr = Expr "+" (*) Factor ["+", "-", EOF]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["+", "-", EOF]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["+", "-", EOF]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["+", "-", EOF]
        //     Factor = (*) Term ["/"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+", "-", EOF]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) r#"\\d+"# ["*"]
        //     Term = (*) r#"\\d+"# ["+", "-", EOF]
        //     Term = (*) r#"\\d+"# ["/"]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        6, // on r#"\\d+"#, goto 5
        // State 7
        //     Expr = Expr "-" (*) Factor ["+", "-", EOF]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["+", "-", EOF]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["+", "-", EOF]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["+", "-", EOF]
        //     Factor = (*) Term ["/"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+", "-", EOF]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) r#"\\d+"# ["*"]
        //     Term = (*) r#"\\d+"# ["+", "-", EOF]
        //     Term = (*) r#"\\d+"# ["/"]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        6, // on r#"\\d+"#, goto 5
        // State 8
        //     Factor = Factor "*" (*) Term ["*", "+", "-", "/", EOF]
        //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
        //     Term = (*) r#"\\d+"# ["*", "+", "-", "/", EOF]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        6, // on r#"\\d+"#, goto 5
        // State 9
        //     Factor = Factor "/" (*) Term ["*", "+", "-", "/", EOF]
        //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
        //     Term = (*) r#"\\d+"# ["*", "+", "-", "/", EOF]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        6, // on r#"\\d+"#, goto 5
        // State 10
        //     Expr = Expr (*) "+" Factor [")", "+", "-"]
        //     Expr = Expr (*) "-" Factor [")", "+", "-"]
        //     Term = "(" Expr (*) ")" ["*", "+", "-", "/", EOF]
        0, // on "(", error
        20, // on ")", goto 19
        0, // on "*", error
        21, // on "+", goto 20
        22, // on "-", goto 21
        0, // on "/", error
        0, // on r#"\\d+"#, error
        // State 11
        //     Expr = Factor (*) [")", "+", "-"]
        //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
        //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
        0, // on "(", error
        -3, // on ")", reduce `Expr = Factor => ActionFn(3);`
        23, // on "*", goto 22
        -3, // on "+", reduce `Expr = Factor => ActionFn(3);`
        -3, // on "-", reduce `Expr = Factor => ActionFn(3);`
        24, // on "/", goto 23
        0, // on r#"\\d+"#, error
        // State 12
        //     Factor = Term (*) [")", "*", "+", "-", "/"]
        0, // on "(", error
        -6, // on ")", reduce `Factor = Term => ActionFn(6);`
        -6, // on "*", reduce `Factor = Term => ActionFn(6);`
        -6, // on "+", reduce `Factor = Term => ActionFn(6);`
        -6, // on "-", reduce `Factor = Term => ActionFn(6);`
        -6, // on "/", reduce `Factor = Term => ActionFn(6);`
        0, // on r#"\\d+"#, error
        // State 13
        //     Expr = (*) Expr "+" Factor [")"]
        //     Expr = (*) Expr "+" Factor ["+"]
        //     Expr = (*) Expr "+" Factor ["-"]
        //     Expr = (*) Expr "-" Factor [")"]
        //     Expr = (*) Expr "-" Factor ["+"]
        //     Expr = (*) Expr "-" Factor ["-"]
        //     Expr = (*) Factor [")"]
        //     Expr = (*) Factor ["+"]
        //     Expr = (*) Factor ["-"]
        //     Factor = (*) Factor "*" Term [")"]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["+"]
        //     Factor = (*) Factor "*" Term ["-"]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term [")"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["+"]
        //     Factor = (*) Factor "/" Term ["-"]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term [")"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["+"]
        //     Factor = (*) Term ["-"]
        //     Factor = (*) Term ["/"]
        //     Term = (*) "(" Expr ")" [")"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+"]
        //     Term = (*) "(" Expr ")" ["-"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = "(" (*) Expr ")" [")", "*", "+", "-", "/"]
        //     Term = (*) r#"\\d+"# [")"]
        //     Term = (*) r#"\\d+"# ["*"]
        //     Term = (*) r#"\\d+"# ["+"]
        //     Term = (*) r#"\\d+"# ["-"]
        //     Term = (*) r#"\\d+"# ["/"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        15, // on r#"\\d+"#, goto 14
        // State 14
        //     Term = r#"\\d+"# (*) [")", "*", "+", "-", "/"]
        0, // on "(", error
        -7, // on ")", reduce `Term = r#"\\d+"# => ActionFn(7);`
        -7, // on "*", reduce `Term = r#"\\d+"# => ActionFn(7);`
        -7, // on "+", reduce `Term = r#"\\d+"# => ActionFn(7);`
        -7, // on "-", reduce `Term = r#"\\d+"# => ActionFn(7);`
        -7, // on "/", reduce `Term = r#"\\d+"# => ActionFn(7);`
        0, // on r#"\\d+"#, error
        // State 15
        //     Expr = Expr "+" Factor (*) ["+", "-", EOF]
        //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
        //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        9, // on "*", goto 8
        -2, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        -2, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        10, // on "/", goto 9
        0, // on r#"\\d+"#, error
        // State 16
        //     Expr = Expr "-" Factor (*) ["+", "-", EOF]
        //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
        //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        9, // on "*", goto 8
        -1, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        -1, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        10, // on "/", goto 9
        0, // on r#"\\d+"#, error
        // State 17
        //     Factor = Factor "*" Term (*) ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        -4, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -4, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -4, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -4, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        0, // on r#"\\d+"#, error
        // State 18
        //     Factor = Factor "/" Term (*) ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        -5, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -5, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -5, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -5, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        0, // on r#"\\d+"#, error
        // State 19
        //     Term = "(" Expr ")" (*) ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        -8, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        -8, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        -8, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        -8, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        0, // on r#"\\d+"#, error
        // State 20
        //     Expr = Expr "+" (*) Factor [")", "+", "-"]
        //     Factor = (*) Factor "*" Term [")", "+", "-"]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term [")", "+", "-"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term [")", "+", "-"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["/"]
        //     Term = (*) "(" Expr ")" [")", "+", "-"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) r#"\\d+"# [")", "+", "-"]
        //     Term = (*) r#"\\d+"# ["*"]
        //     Term = (*) r#"\\d+"# ["/"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        15, // on r#"\\d+"#, goto 14
        // State 21
        //     Expr = Expr "-" (*) Factor [")", "+", "-"]
        //     Factor = (*) Factor "*" Term [")", "+", "-"]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term [")", "+", "-"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term [")", "+", "-"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["/"]
        //     Term = (*) "(" Expr ")" [")", "+", "-"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) r#"\\d+"# [")", "+", "-"]
        //     Term = (*) r#"\\d+"# ["*"]
        //     Term = (*) r#"\\d+"# ["/"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        15, // on r#"\\d+"#, goto 14
        // State 22
        //     Factor = Factor "*" (*) Term [")", "*", "+", "-", "/"]
        //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
        //     Term = (*) r#"\\d+"# [")", "*", "+", "-", "/"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        15, // on r#"\\d+"#, goto 14
        // State 23
        //     Factor = Factor "/" (*) Term [")", "*", "+", "-", "/"]
        //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
        //     Term = (*) r#"\\d+"# [")", "*", "+", "-", "/"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        15, // on r#"\\d+"#, goto 14
        // State 24
        //     Expr = Expr (*) "+" Factor [")", "+", "-"]
        //     Expr = Expr (*) "-" Factor [")", "+", "-"]
        //     Term = "(" Expr (*) ")" [")", "*", "+", "-", "/"]
        0, // on "(", error
        30, // on ")", goto 29
        0, // on "*", error
        21, // on "+", goto 20
        22, // on "-", goto 21
        0, // on "/", error
        0, // on r#"\\d+"#, error
        // State 25
        //     Expr = Expr "+" Factor (*) [")", "+", "-"]
        //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
        //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
        0, // on "(", error
        -2, // on ")", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        23, // on "*", goto 22
        -2, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        -2, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        24, // on "/", goto 23
        0, // on r#"\\d+"#, error
        // State 26
        //     Expr = Expr "-" Factor (*) [")", "+", "-"]
        //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
        //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
        0, // on "(", error
        -1, // on ")", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        23, // on "*", goto 22
        -1, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        -1, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        24, // on "/", goto 23
        0, // on r#"\\d+"#, error
        // State 27
        //     Factor = Factor "*" Term (*) [")", "*", "+", "-", "/"]
        0, // on "(", error
        -4, // on ")", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -4, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -4, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -4, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -4, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        0, // on r#"\\d+"#, error
        // State 28
        //     Factor = Factor "/" Term (*) [")", "*", "+", "-", "/"]
        0, // on "(", error
        -5, // on ")", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -5, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -5, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -5, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -5, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        0, // on r#"\\d+"#, error
        // State 29
        //     Term = "(" Expr ")" (*) [")", "*", "+", "-", "/"]
        0, // on "(", error
        -8, // on ")", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        -8, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        -8, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        -8, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        -8, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(8);`
        0, // on r#"\\d+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -9, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
        -3, // on EOF, reduce `Expr = Factor => ActionFn(3);`
        -6, // on EOF, reduce `Factor = Term => ActionFn(6);`
        0, // on EOF, error
        -7, // on EOF, reduce `Term = r#"\\d+"# => ActionFn(7);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -2, // on EOF, reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        -1, // on EOF, reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        -4, // on EOF, reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -5, // on EOF, reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -8, // on EOF, reduce `Term = "(", Expr, ")" => ActionFn(8);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Expr, goto 1
        3, // on Factor, goto 2
        4, // on Term, goto 3
        0, // on __Expr, error
        // State 1
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 2
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 3
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 4
        11, // on Expr, goto 10
        12, // on Factor, goto 11
        13, // on Term, goto 12
        0, // on __Expr, error
        // State 5
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 6
        0, // on Expr, error
        16, // on Factor, goto 15
        4, // on Term, goto 3
        0, // on __Expr, error
        // State 7
        0, // on Expr, error
        17, // on Factor, goto 16
        4, // on Term, goto 3
        0, // on __Expr, error
        // State 8
        0, // on Expr, error
        0, // on Factor, error
        18, // on Term, goto 17
        0, // on __Expr, error
        // State 9
        0, // on Expr, error
        0, // on Factor, error
        19, // on Term, goto 18
        0, // on __Expr, error
        // State 10
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 11
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 12
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 13
        25, // on Expr, goto 24
        12, // on Factor, goto 11
        13, // on Term, goto 12
        0, // on __Expr, error
        // State 14
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 15
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 16
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 17
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 18
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 19
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 20
        0, // on Expr, error
        26, // on Factor, goto 25
        13, // on Term, goto 12
        0, // on __Expr, error
        // State 21
        0, // on Expr, error
        27, // on Factor, goto 26
        13, // on Term, goto 12
        0, // on __Expr, error
        // State 22
        0, // on Expr, error
        0, // on Factor, error
        28, // on Term, goto 27
        0, // on __Expr, error
        // State 23
        0, // on Expr, error
        0, // on Factor, error
        29, // on Term, goto 28
        0, // on __Expr, error
        // State 24
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 25
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 26
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 27
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 28
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 29
        0, // on Expr, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
    ];
    pub fn parse_Expr<
        'input,
    >(
        input: &'input str,
    ) -> Result<(), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                _ => {
                    return Err(__ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Termr_23_22_5c_5cd_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<(),__ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Expr = Expr, "-", Factor => ActionFn(1);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            2 => {
                // Expr = Expr, "+", Factor => ActionFn(2);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            3 => {
                // Expr = Factor => ActionFn(3);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            4 => {
                // Factor = Factor, "*", Term => ActionFn(4);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            5 => {
                // Factor = Factor, "/", Term => ActionFn(5);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            6 => {
                // Factor = Term => ActionFn(6);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            7 => {
                // Term = r#"\\d+"# => ActionFn(7);
                let __sym0 = __pop_Termr_23_22_5c_5cd_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                2
            }
            8 => {
                // Term = "(", Expr, ")" => ActionFn(8);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                2
            }
            9 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 4 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cd_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cd_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Expr::parse_Expr;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, (), usize),
) -> ()
{
    ()
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, (), usize),
) -> ()
{
    ()
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, (), usize),
) -> ()
{
    ()
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, (), usize),
) -> ()
{
    ()
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, (), usize),
    (_, __2, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
