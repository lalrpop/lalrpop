#![allow(unused_imports)]
#![allow(unused_variables)]
use expr_arena_ast::{Arena, Node, Op};
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use expr_arena_ast::{Arena, Node, Op};
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'ast> {
        Term_22_28_22(Tok),
        Term_22_29_22(Tok),
        Term_22_2a_22(Tok),
        Term_22_2b_22(Tok),
        Term_22_2c_22(Tok),
        Term_22_2d_22(Tok),
        Term_22_2f_22(Tok),
        TermNum(i32),
        Nt_28_3cExpr_3e_20_22_2c_22_29(&'ast Node<'ast>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'ast Node<'ast>>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'ast Node<'ast>>),
        NtComma_3cExpr_3e(Vec<&'ast Node<'ast>>),
        NtExpr(&'ast Node<'ast>),
        NtExpr_3f(::std::option::Option<&'ast Node<'ast>>),
        NtFactor(&'ast Node<'ast>),
        NtTerm(&'ast Node<'ast>),
        Nt____Expr(&'ast Node<'ast>),
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
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" [EOF]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+"]
        //     Term = (*) "(" Expr ")" ["-"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) "(" Expr ")" [EOF]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["+"]
        //     Term = (*) Num ["-"]
        //     Term = (*) Num ["/"]
        //     Term = (*) Num [EOF]
        //     __Expr = (*) Expr [EOF]
        5, // on "(", goto 4
        0, // on ")", error
        6, // on "*", goto 5
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        7, // on Num, goto 6
        // State 1
        //     Expr = Expr (*) "+" Factor ["+", "-", EOF]
        //     Expr = Expr (*) "-" Factor ["+", "-", EOF]
        //     __Expr = Expr (*) [EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        8, // on "+", goto 7
        0, // on ",", error
        9, // on "-", goto 8
        0, // on "/", error
        0, // on Num, error
        // State 2
        //     Expr = Factor (*) ["+", "-", EOF]
        //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
        //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        10, // on "*", goto 9
        -12, // on "+", reduce `Expr = Factor => ActionFn(3);`
        0, // on ",", error
        -12, // on "-", reduce `Expr = Factor => ActionFn(3);`
        11, // on "/", goto 10
        0, // on Num, error
        // State 3
        //     Factor = Term (*) ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        -18, // on "*", reduce `Factor = Term => ActionFn(7);`
        -18, // on "+", reduce `Factor = Term => ActionFn(7);`
        0, // on ",", error
        -18, // on "-", reduce `Factor = Term => ActionFn(7);`
        -18, // on "/", reduce `Factor = Term => ActionFn(7);`
        0, // on Num, error
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
        //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Term = (*) "(" Expr ")" [")"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+"]
        //     Term = (*) "(" Expr ")" ["-"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = "(" (*) Expr ")" ["*", "+", "-", "/", EOF]
        //     Term = (*) Num [")"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["+"]
        //     Term = (*) Num ["-"]
        //     Term = (*) Num ["/"]
        15, // on "(", goto 14
        0, // on ")", error
        16, // on "*", goto 15
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        17, // on Num, goto 16
        // State 5
        //     Factor = "*" (*) "(" Comma<Expr> ")" ["*", "+", "-", "/", EOF]
        18, // on "(", goto 17
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on Num, error
        // State 6
        //     Term = Num (*) ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        -19, // on "*", reduce `Term = Num => ActionFn(8);`
        -19, // on "+", reduce `Term = Num => ActionFn(8);`
        0, // on ",", error
        -19, // on "-", reduce `Term = Num => ActionFn(8);`
        -19, // on "/", reduce `Term = Num => ActionFn(8);`
        0, // on Num, error
        // State 7
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
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["+", "-", EOF]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+", "-", EOF]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["+", "-", EOF]
        //     Term = (*) Num ["/"]
        5, // on "(", goto 4
        0, // on ")", error
        6, // on "*", goto 5
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        7, // on Num, goto 6
        // State 8
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
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["+", "-", EOF]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+", "-", EOF]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["+", "-", EOF]
        //     Term = (*) Num ["/"]
        5, // on "(", goto 4
        0, // on ")", error
        6, // on "*", goto 5
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        7, // on Num, goto 6
        // State 9
        //     Factor = Factor "*" (*) Term ["*", "+", "-", "/", EOF]
        //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
        //     Term = (*) Num ["*", "+", "-", "/", EOF]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        7, // on Num, goto 6
        // State 10
        //     Factor = Factor "/" (*) Term ["*", "+", "-", "/", EOF]
        //     Term = (*) "(" Expr ")" ["*", "+", "-", "/", EOF]
        //     Term = (*) Num ["*", "+", "-", "/", EOF]
        5, // on "(", goto 4
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        7, // on Num, goto 6
        // State 11
        //     Expr = Expr (*) "+" Factor [")", "+", "-"]
        //     Expr = Expr (*) "-" Factor [")", "+", "-"]
        //     Term = "(" Expr (*) ")" ["*", "+", "-", "/", EOF]
        0, // on "(", error
        23, // on ")", goto 22
        0, // on "*", error
        24, // on "+", goto 23
        0, // on ",", error
        25, // on "-", goto 24
        0, // on "/", error
        0, // on Num, error
        // State 12
        //     Expr = Factor (*) [")", "+", "-"]
        //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
        //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
        0, // on "(", error
        -12, // on ")", reduce `Expr = Factor => ActionFn(3);`
        26, // on "*", goto 25
        -12, // on "+", reduce `Expr = Factor => ActionFn(3);`
        0, // on ",", error
        -12, // on "-", reduce `Expr = Factor => ActionFn(3);`
        27, // on "/", goto 26
        0, // on Num, error
        // State 13
        //     Factor = Term (*) [")", "*", "+", "-", "/"]
        0, // on "(", error
        -18, // on ")", reduce `Factor = Term => ActionFn(7);`
        -18, // on "*", reduce `Factor = Term => ActionFn(7);`
        -18, // on "+", reduce `Factor = Term => ActionFn(7);`
        0, // on ",", error
        -18, // on "-", reduce `Factor = Term => ActionFn(7);`
        -18, // on "/", reduce `Factor = Term => ActionFn(7);`
        0, // on Num, error
        // State 14
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
        //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Term = (*) "(" Expr ")" [")"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+"]
        //     Term = (*) "(" Expr ")" ["-"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = "(" (*) Expr ")" [")", "*", "+", "-", "/"]
        //     Term = (*) Num [")"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["+"]
        //     Term = (*) Num ["-"]
        //     Term = (*) Num ["/"]
        15, // on "(", goto 14
        0, // on ")", error
        16, // on "*", goto 15
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        17, // on Num, goto 16
        // State 15
        //     Factor = "*" (*) "(" Comma<Expr> ")" [")", "*", "+", "-", "/"]
        29, // on "(", goto 28
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on Num, error
        // State 16
        //     Term = Num (*) [")", "*", "+", "-", "/"]
        0, // on "(", error
        -19, // on ")", reduce `Term = Num => ActionFn(8);`
        -19, // on "*", reduce `Term = Num => ActionFn(8);`
        -19, // on "+", reduce `Term = Num => ActionFn(8);`
        0, // on ",", error
        -19, // on "-", reduce `Term = Num => ActionFn(8);`
        -19, // on "/", reduce `Term = Num => ActionFn(8);`
        0, // on Num, error
        // State 17
        //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," ["(", "*", Num]
        //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," [")"]
        //     (<Expr> ",")+ = (*) Expr "," ["(", "*", Num]
        //     (<Expr> ",")+ = (*) Expr "," [")"]
        //     Comma<Expr> = (*) [")"]
        //     Comma<Expr> = (*) (<Expr> ",")+ [")"]
        //     Comma<Expr> = (*) (<Expr> ",")+ Expr [")"]
        //     Comma<Expr> = (*) Expr [")"]
        //     Expr = (*) Expr "+" Factor [")"]
        //     Expr = (*) Expr "+" Factor ["+"]
        //     Expr = (*) Expr "+" Factor [","]
        //     Expr = (*) Expr "+" Factor ["-"]
        //     Expr = (*) Expr "-" Factor [")"]
        //     Expr = (*) Expr "-" Factor ["+"]
        //     Expr = (*) Expr "-" Factor [","]
        //     Expr = (*) Expr "-" Factor ["-"]
        //     Expr = (*) Factor [")"]
        //     Expr = (*) Factor ["+"]
        //     Expr = (*) Factor [","]
        //     Expr = (*) Factor ["-"]
        //     Factor = (*) Factor "*" Term [")"]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["+"]
        //     Factor = (*) Factor "*" Term [","]
        //     Factor = (*) Factor "*" Term ["-"]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term [")"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["+"]
        //     Factor = (*) Factor "/" Term [","]
        //     Factor = (*) Factor "/" Term ["-"]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term [")"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["+"]
        //     Factor = (*) Term [","]
        //     Factor = (*) Term ["-"]
        //     Factor = (*) Term ["/"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Factor = "*" "(" (*) Comma<Expr> ")" ["*", "+", "-", "/", EOF]
        //     Term = (*) "(" Expr ")" [")"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+"]
        //     Term = (*) "(" Expr ")" [","]
        //     Term = (*) "(" Expr ")" ["-"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) Num [")"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["+"]
        //     Term = (*) Num [","]
        //     Term = (*) Num ["-"]
        //     Term = (*) Num ["/"]
        35, // on "(", goto 34
        -7, // on ")", reduce `Comma<Expr> =  => ActionFn(23);`
        36, // on "*", goto 35
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        37, // on Num, goto 36
        // State 18
        //     Expr = Expr "+" Factor (*) ["+", "-", EOF]
        //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
        //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        10, // on "*", goto 9
        -11, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        0, // on ",", error
        -11, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        11, // on "/", goto 10
        0, // on Num, error
        // State 19
        //     Expr = Expr "-" Factor (*) ["+", "-", EOF]
        //     Factor = Factor (*) "*" Term ["*", "+", "-", "/", EOF]
        //     Factor = Factor (*) "/" Term ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        10, // on "*", goto 9
        -10, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        0, // on ",", error
        -10, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        11, // on "/", goto 10
        0, // on Num, error
        // State 20
        //     Factor = Factor "*" Term (*) ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        -15, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -15, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        0, // on ",", error
        -15, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -15, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        0, // on Num, error
        // State 21
        //     Factor = Factor "/" Term (*) ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        -16, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -16, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        0, // on ",", error
        -16, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -16, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        0, // on Num, error
        // State 22
        //     Term = "(" Expr ")" (*) ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        -20, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        -20, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        0, // on ",", error
        -20, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        -20, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        0, // on Num, error
        // State 23
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
        //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", "-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Term = (*) "(" Expr ")" [")", "+", "-"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) Num [")", "+", "-"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["/"]
        15, // on "(", goto 14
        0, // on ")", error
        16, // on "*", goto 15
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        17, // on Num, goto 16
        // State 24
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
        //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", "-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Term = (*) "(" Expr ")" [")", "+", "-"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) Num [")", "+", "-"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["/"]
        15, // on "(", goto 14
        0, // on ")", error
        16, // on "*", goto 15
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        17, // on Num, goto 16
        // State 25
        //     Factor = Factor "*" (*) Term [")", "*", "+", "-", "/"]
        //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
        //     Term = (*) Num [")", "*", "+", "-", "/"]
        15, // on "(", goto 14
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        17, // on Num, goto 16
        // State 26
        //     Factor = Factor "/" (*) Term [")", "*", "+", "-", "/"]
        //     Term = (*) "(" Expr ")" [")", "*", "+", "-", "/"]
        //     Term = (*) Num [")", "*", "+", "-", "/"]
        15, // on "(", goto 14
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        17, // on Num, goto 16
        // State 27
        //     Expr = Expr (*) "+" Factor [")", "+", "-"]
        //     Expr = Expr (*) "-" Factor [")", "+", "-"]
        //     Term = "(" Expr (*) ")" [")", "*", "+", "-", "/"]
        0, // on "(", error
        42, // on ")", goto 41
        0, // on "*", error
        24, // on "+", goto 23
        0, // on ",", error
        25, // on "-", goto 24
        0, // on "/", error
        0, // on Num, error
        // State 28
        //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," ["(", "*", Num]
        //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," [")"]
        //     (<Expr> ",")+ = (*) Expr "," ["(", "*", Num]
        //     (<Expr> ",")+ = (*) Expr "," [")"]
        //     Comma<Expr> = (*) [")"]
        //     Comma<Expr> = (*) (<Expr> ",")+ [")"]
        //     Comma<Expr> = (*) (<Expr> ",")+ Expr [")"]
        //     Comma<Expr> = (*) Expr [")"]
        //     Expr = (*) Expr "+" Factor [")"]
        //     Expr = (*) Expr "+" Factor ["+"]
        //     Expr = (*) Expr "+" Factor [","]
        //     Expr = (*) Expr "+" Factor ["-"]
        //     Expr = (*) Expr "-" Factor [")"]
        //     Expr = (*) Expr "-" Factor ["+"]
        //     Expr = (*) Expr "-" Factor [","]
        //     Expr = (*) Expr "-" Factor ["-"]
        //     Expr = (*) Factor [")"]
        //     Expr = (*) Factor ["+"]
        //     Expr = (*) Factor [","]
        //     Expr = (*) Factor ["-"]
        //     Factor = (*) Factor "*" Term [")"]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["+"]
        //     Factor = (*) Factor "*" Term [","]
        //     Factor = (*) Factor "*" Term ["-"]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term [")"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["+"]
        //     Factor = (*) Factor "/" Term [","]
        //     Factor = (*) Factor "/" Term ["-"]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term [")"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["+"]
        //     Factor = (*) Term [","]
        //     Factor = (*) Term ["-"]
        //     Factor = (*) Term ["/"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Factor = "*" "(" (*) Comma<Expr> ")" [")", "*", "+", "-", "/"]
        //     Term = (*) "(" Expr ")" [")"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+"]
        //     Term = (*) "(" Expr ")" [","]
        //     Term = (*) "(" Expr ")" ["-"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) Num [")"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["+"]
        //     Term = (*) Num [","]
        //     Term = (*) Num ["-"]
        //     Term = (*) Num ["/"]
        35, // on "(", goto 34
        -7, // on ")", reduce `Comma<Expr> =  => ActionFn(23);`
        36, // on "*", goto 35
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        37, // on Num, goto 36
        // State 29
        //     (<Expr> ",")+ = (<Expr> ",")+ (*) Expr "," ["(", ")", "*", Num]
        //     Comma<Expr> = (<Expr> ",")+ (*) [")"]
        //     Comma<Expr> = (<Expr> ",")+ (*) Expr [")"]
        //     Expr = (*) Expr "+" Factor [")"]
        //     Expr = (*) Expr "+" Factor ["+"]
        //     Expr = (*) Expr "+" Factor [","]
        //     Expr = (*) Expr "+" Factor ["-"]
        //     Expr = (*) Expr "-" Factor [")"]
        //     Expr = (*) Expr "-" Factor ["+"]
        //     Expr = (*) Expr "-" Factor [","]
        //     Expr = (*) Expr "-" Factor ["-"]
        //     Expr = (*) Factor [")"]
        //     Expr = (*) Factor ["+"]
        //     Expr = (*) Factor [","]
        //     Expr = (*) Factor ["-"]
        //     Factor = (*) Factor "*" Term [")"]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["+"]
        //     Factor = (*) Factor "*" Term [","]
        //     Factor = (*) Factor "*" Term ["-"]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term [")"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["+"]
        //     Factor = (*) Factor "/" Term [","]
        //     Factor = (*) Factor "/" Term ["-"]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term [")"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["+"]
        //     Factor = (*) Term [","]
        //     Factor = (*) Term ["-"]
        //     Factor = (*) Term ["/"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Term = (*) "(" Expr ")" [")"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+"]
        //     Term = (*) "(" Expr ")" [","]
        //     Term = (*) "(" Expr ")" ["-"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) Num [")"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["+"]
        //     Term = (*) Num [","]
        //     Term = (*) Num ["-"]
        //     Term = (*) Num ["/"]
        35, // on "(", goto 34
        -9, // on ")", reduce `Comma<Expr> = (<Expr> ",")+ => ActionFn(25);`
        36, // on "*", goto 35
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        37, // on Num, goto 36
        // State 30
        //     Factor = "*" "(" Comma<Expr> (*) ")" ["*", "+", "-", "/", EOF]
        0, // on "(", error
        45, // on ")", goto 44
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on Num, error
        // State 31
        //     (<Expr> ",")+ = Expr (*) "," ["(", ")", "*", Num]
        //     Comma<Expr> = Expr (*) [")"]
        //     Expr = Expr (*) "+" Factor [")", "+", ",", "-"]
        //     Expr = Expr (*) "-" Factor [")", "+", ",", "-"]
        0, // on "(", error
        -6, // on ")", reduce `Comma<Expr> = Expr => ActionFn(22);`
        0, // on "*", error
        46, // on "+", goto 45
        47, // on ",", goto 46
        48, // on "-", goto 47
        0, // on "/", error
        0, // on Num, error
        // State 32
        //     Expr = Factor (*) [")", "+", ",", "-"]
        //     Factor = Factor (*) "*" Term [")", "*", "+", ",", "-", "/"]
        //     Factor = Factor (*) "/" Term [")", "*", "+", ",", "-", "/"]
        0, // on "(", error
        -12, // on ")", reduce `Expr = Factor => ActionFn(3);`
        49, // on "*", goto 48
        -12, // on "+", reduce `Expr = Factor => ActionFn(3);`
        -12, // on ",", reduce `Expr = Factor => ActionFn(3);`
        -12, // on "-", reduce `Expr = Factor => ActionFn(3);`
        50, // on "/", goto 49
        0, // on Num, error
        // State 33
        //     Factor = Term (*) [")", "*", "+", ",", "-", "/"]
        0, // on "(", error
        -18, // on ")", reduce `Factor = Term => ActionFn(7);`
        -18, // on "*", reduce `Factor = Term => ActionFn(7);`
        -18, // on "+", reduce `Factor = Term => ActionFn(7);`
        -18, // on ",", reduce `Factor = Term => ActionFn(7);`
        -18, // on "-", reduce `Factor = Term => ActionFn(7);`
        -18, // on "/", reduce `Factor = Term => ActionFn(7);`
        0, // on Num, error
        // State 34
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
        //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Term = (*) "(" Expr ")" [")"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+"]
        //     Term = (*) "(" Expr ")" ["-"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = "(" (*) Expr ")" [")", "*", "+", ",", "-", "/"]
        //     Term = (*) Num [")"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["+"]
        //     Term = (*) Num ["-"]
        //     Term = (*) Num ["/"]
        15, // on "(", goto 14
        0, // on ")", error
        16, // on "*", goto 15
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        17, // on Num, goto 16
        // State 35
        //     Factor = "*" (*) "(" Comma<Expr> ")" [")", "*", "+", ",", "-", "/"]
        52, // on "(", goto 51
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on Num, error
        // State 36
        //     Term = Num (*) [")", "*", "+", ",", "-", "/"]
        0, // on "(", error
        -19, // on ")", reduce `Term = Num => ActionFn(8);`
        -19, // on "*", reduce `Term = Num => ActionFn(8);`
        -19, // on "+", reduce `Term = Num => ActionFn(8);`
        -19, // on ",", reduce `Term = Num => ActionFn(8);`
        -19, // on "-", reduce `Term = Num => ActionFn(8);`
        -19, // on "/", reduce `Term = Num => ActionFn(8);`
        0, // on Num, error
        // State 37
        //     Expr = Expr "+" Factor (*) [")", "+", "-"]
        //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
        //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
        0, // on "(", error
        -11, // on ")", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        26, // on "*", goto 25
        -11, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        0, // on ",", error
        -11, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        27, // on "/", goto 26
        0, // on Num, error
        // State 38
        //     Expr = Expr "-" Factor (*) [")", "+", "-"]
        //     Factor = Factor (*) "*" Term [")", "*", "+", "-", "/"]
        //     Factor = Factor (*) "/" Term [")", "*", "+", "-", "/"]
        0, // on "(", error
        -10, // on ")", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        26, // on "*", goto 25
        -10, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        0, // on ",", error
        -10, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        27, // on "/", goto 26
        0, // on Num, error
        // State 39
        //     Factor = Factor "*" Term (*) [")", "*", "+", "-", "/"]
        0, // on "(", error
        -15, // on ")", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -15, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -15, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        0, // on ",", error
        -15, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -15, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        0, // on Num, error
        // State 40
        //     Factor = Factor "/" Term (*) [")", "*", "+", "-", "/"]
        0, // on "(", error
        -16, // on ")", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -16, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -16, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        0, // on ",", error
        -16, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -16, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        0, // on Num, error
        // State 41
        //     Term = "(" Expr ")" (*) [")", "*", "+", "-", "/"]
        0, // on "(", error
        -20, // on ")", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        -20, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        -20, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        0, // on ",", error
        -20, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        -20, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        0, // on Num, error
        // State 42
        //     Factor = "*" "(" Comma<Expr> (*) ")" [")", "*", "+", "-", "/"]
        0, // on "(", error
        53, // on ")", goto 52
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on Num, error
        // State 43
        //     (<Expr> ",")+ = (<Expr> ",")+ Expr (*) "," ["(", ")", "*", Num]
        //     Comma<Expr> = (<Expr> ",")+ Expr (*) [")"]
        //     Expr = Expr (*) "+" Factor [")", "+", ",", "-"]
        //     Expr = Expr (*) "-" Factor [")", "+", ",", "-"]
        0, // on "(", error
        -8, // on ")", reduce `Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(24);`
        0, // on "*", error
        46, // on "+", goto 45
        54, // on ",", goto 53
        48, // on "-", goto 47
        0, // on "/", error
        0, // on Num, error
        // State 44
        //     Factor = "*" "(" Comma<Expr> ")" (*) ["*", "+", "-", "/", EOF]
        0, // on "(", error
        0, // on ")", error
        -17, // on "*", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        -17, // on "+", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        0, // on ",", error
        -17, // on "-", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        -17, // on "/", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        0, // on Num, error
        // State 45
        //     Expr = Expr "+" (*) Factor [")", "+", ",", "-"]
        //     Factor = (*) Factor "*" Term [")", "+", ",", "-"]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term [")", "+", ",", "-"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term [")", "+", ",", "-"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["/"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", ",", "-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Term = (*) "(" Expr ")" [")", "+", ",", "-"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) Num [")", "+", ",", "-"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["/"]
        35, // on "(", goto 34
        0, // on ")", error
        36, // on "*", goto 35
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        37, // on Num, goto 36
        // State 46
        //     (<Expr> ",")+ = Expr "," (*) ["(", ")", "*", Num]
        -4, // on "(", reduce `(<Expr> ",")+ = Expr, "," => ActionFn(18);`
        -4, // on ")", reduce `(<Expr> ",")+ = Expr, "," => ActionFn(18);`
        -4, // on "*", reduce `(<Expr> ",")+ = Expr, "," => ActionFn(18);`
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        -4, // on Num, reduce `(<Expr> ",")+ = Expr, "," => ActionFn(18);`
        // State 47
        //     Expr = Expr "-" (*) Factor [")", "+", ",", "-"]
        //     Factor = (*) Factor "*" Term [")", "+", ",", "-"]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term [")", "+", ",", "-"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term [")", "+", ",", "-"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["/"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" [")", "+", ",", "-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Term = (*) "(" Expr ")" [")", "+", ",", "-"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) Num [")", "+", ",", "-"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["/"]
        35, // on "(", goto 34
        0, // on ")", error
        36, // on "*", goto 35
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        37, // on Num, goto 36
        // State 48
        //     Factor = Factor "*" (*) Term [")", "*", "+", ",", "-", "/"]
        //     Term = (*) "(" Expr ")" [")", "*", "+", ",", "-", "/"]
        //     Term = (*) Num [")", "*", "+", ",", "-", "/"]
        35, // on "(", goto 34
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        37, // on Num, goto 36
        // State 49
        //     Factor = Factor "/" (*) Term [")", "*", "+", ",", "-", "/"]
        //     Term = (*) "(" Expr ")" [")", "*", "+", ",", "-", "/"]
        //     Term = (*) Num [")", "*", "+", ",", "-", "/"]
        35, // on "(", goto 34
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        37, // on Num, goto 36
        // State 50
        //     Expr = Expr (*) "+" Factor [")", "+", "-"]
        //     Expr = Expr (*) "-" Factor [")", "+", "-"]
        //     Term = "(" Expr (*) ")" [")", "*", "+", ",", "-", "/"]
        0, // on "(", error
        59, // on ")", goto 58
        0, // on "*", error
        24, // on "+", goto 23
        0, // on ",", error
        25, // on "-", goto 24
        0, // on "/", error
        0, // on Num, error
        // State 51
        //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," ["(", "*", Num]
        //     (<Expr> ",")+ = (*) (<Expr> ",")+ Expr "," [")"]
        //     (<Expr> ",")+ = (*) Expr "," ["(", "*", Num]
        //     (<Expr> ",")+ = (*) Expr "," [")"]
        //     Comma<Expr> = (*) [")"]
        //     Comma<Expr> = (*) (<Expr> ",")+ [")"]
        //     Comma<Expr> = (*) (<Expr> ",")+ Expr [")"]
        //     Comma<Expr> = (*) Expr [")"]
        //     Expr = (*) Expr "+" Factor [")"]
        //     Expr = (*) Expr "+" Factor ["+"]
        //     Expr = (*) Expr "+" Factor [","]
        //     Expr = (*) Expr "+" Factor ["-"]
        //     Expr = (*) Expr "-" Factor [")"]
        //     Expr = (*) Expr "-" Factor ["+"]
        //     Expr = (*) Expr "-" Factor [","]
        //     Expr = (*) Expr "-" Factor ["-"]
        //     Expr = (*) Factor [")"]
        //     Expr = (*) Factor ["+"]
        //     Expr = (*) Factor [","]
        //     Expr = (*) Factor ["-"]
        //     Factor = (*) Factor "*" Term [")"]
        //     Factor = (*) Factor "*" Term ["*"]
        //     Factor = (*) Factor "*" Term ["+"]
        //     Factor = (*) Factor "*" Term [","]
        //     Factor = (*) Factor "*" Term ["-"]
        //     Factor = (*) Factor "*" Term ["/"]
        //     Factor = (*) Factor "/" Term [")"]
        //     Factor = (*) Factor "/" Term ["*"]
        //     Factor = (*) Factor "/" Term ["+"]
        //     Factor = (*) Factor "/" Term [","]
        //     Factor = (*) Factor "/" Term ["-"]
        //     Factor = (*) Factor "/" Term ["/"]
        //     Factor = (*) Term [")"]
        //     Factor = (*) Term ["*"]
        //     Factor = (*) Term ["+"]
        //     Factor = (*) Term [","]
        //     Factor = (*) Term ["-"]
        //     Factor = (*) Term ["/"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" [")"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" [","]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
        //     Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
        //     Factor = "*" "(" (*) Comma<Expr> ")" [")", "*", "+", ",", "-", "/"]
        //     Term = (*) "(" Expr ")" [")"]
        //     Term = (*) "(" Expr ")" ["*"]
        //     Term = (*) "(" Expr ")" ["+"]
        //     Term = (*) "(" Expr ")" [","]
        //     Term = (*) "(" Expr ")" ["-"]
        //     Term = (*) "(" Expr ")" ["/"]
        //     Term = (*) Num [")"]
        //     Term = (*) Num ["*"]
        //     Term = (*) Num ["+"]
        //     Term = (*) Num [","]
        //     Term = (*) Num ["-"]
        //     Term = (*) Num ["/"]
        35, // on "(", goto 34
        -7, // on ")", reduce `Comma<Expr> =  => ActionFn(23);`
        36, // on "*", goto 35
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        37, // on Num, goto 36
        // State 52
        //     Factor = "*" "(" Comma<Expr> ")" (*) [")", "*", "+", "-", "/"]
        0, // on "(", error
        -17, // on ")", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        -17, // on "*", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        -17, // on "+", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        0, // on ",", error
        -17, // on "-", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        -17, // on "/", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        0, // on Num, error
        // State 53
        //     (<Expr> ",")+ = (<Expr> ",")+ Expr "," (*) ["(", ")", "*", Num]
        -5, // on "(", reduce `(<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(19);`
        -5, // on ")", reduce `(<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(19);`
        -5, // on "*", reduce `(<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(19);`
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        -5, // on Num, reduce `(<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(19);`
        // State 54
        //     Expr = Expr "+" Factor (*) [")", "+", ",", "-"]
        //     Factor = Factor (*) "*" Term [")", "*", "+", ",", "-", "/"]
        //     Factor = Factor (*) "/" Term [")", "*", "+", ",", "-", "/"]
        0, // on "(", error
        -11, // on ")", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        49, // on "*", goto 48
        -11, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        -11, // on ",", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        -11, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        50, // on "/", goto 49
        0, // on Num, error
        // State 55
        //     Expr = Expr "-" Factor (*) [")", "+", ",", "-"]
        //     Factor = Factor (*) "*" Term [")", "*", "+", ",", "-", "/"]
        //     Factor = Factor (*) "/" Term [")", "*", "+", ",", "-", "/"]
        0, // on "(", error
        -10, // on ")", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        49, // on "*", goto 48
        -10, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        -10, // on ",", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        -10, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        50, // on "/", goto 49
        0, // on Num, error
        // State 56
        //     Factor = Factor "*" Term (*) [")", "*", "+", ",", "-", "/"]
        0, // on "(", error
        -15, // on ")", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -15, // on "*", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -15, // on "+", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -15, // on ",", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -15, // on "-", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -15, // on "/", reduce `Factor = Factor, "*", Term => ActionFn(4);`
        0, // on Num, error
        // State 57
        //     Factor = Factor "/" Term (*) [")", "*", "+", ",", "-", "/"]
        0, // on "(", error
        -16, // on ")", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -16, // on "*", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -16, // on "+", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -16, // on ",", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -16, // on "-", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -16, // on "/", reduce `Factor = Factor, "/", Term => ActionFn(5);`
        0, // on Num, error
        // State 58
        //     Term = "(" Expr ")" (*) [")", "*", "+", ",", "-", "/"]
        0, // on "(", error
        -20, // on ")", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        -20, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        -20, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        -20, // on ",", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        -20, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        -20, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(9);`
        0, // on Num, error
        // State 59
        //     Factor = "*" "(" Comma<Expr> (*) ")" [")", "*", "+", ",", "-", "/"]
        0, // on "(", error
        61, // on ")", goto 60
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on Num, error
        // State 60
        //     Factor = "*" "(" Comma<Expr> ")" (*) [")", "*", "+", ",", "-", "/"]
        0, // on "(", error
        -17, // on ")", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        -17, // on "*", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        -17, // on "+", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        -17, // on ",", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        -17, // on "-", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        -17, // on "/", reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
        0, // on Num, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -21, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
        -12, // on EOF, reduce `Expr = Factor => ActionFn(3);`
        -18, // on EOF, reduce `Factor = Term => ActionFn(7);`
        0, // on EOF, error
        0, // on EOF, error
        -19, // on EOF, reduce `Term = Num => ActionFn(8);`
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
        0, // on EOF, error
        -11, // on EOF, reduce `Expr = Expr, "+", Factor => ActionFn(2);`
        -10, // on EOF, reduce `Expr = Expr, "-", Factor => ActionFn(1);`
        -15, // on EOF, reduce `Factor = Factor, "*", Term => ActionFn(4);`
        -16, // on EOF, reduce `Factor = Factor, "/", Term => ActionFn(5);`
        -20, // on EOF, reduce `Term = "(", Expr, ")" => ActionFn(9);`
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
        0, // on EOF, error
        -17, // on EOF, reduce `Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);`
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
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        2, // on Expr, goto 1
        0, // on Expr?, error
        3, // on Factor, goto 2
        4, // on Term, goto 3
        0, // on __Expr, error
        // State 1
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 2
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 3
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 4
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        12, // on Expr, goto 11
        0, // on Expr?, error
        13, // on Factor, goto 12
        14, // on Term, goto 13
        0, // on __Expr, error
        // State 5
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 6
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 7
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        19, // on Factor, goto 18
        4, // on Term, goto 3
        0, // on __Expr, error
        // State 8
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        20, // on Factor, goto 19
        4, // on Term, goto 3
        0, // on __Expr, error
        // State 9
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        21, // on Term, goto 20
        0, // on __Expr, error
        // State 10
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        22, // on Term, goto 21
        0, // on __Expr, error
        // State 11
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 12
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 13
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 14
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        28, // on Expr, goto 27
        0, // on Expr?, error
        13, // on Factor, goto 12
        14, // on Term, goto 13
        0, // on __Expr, error
        // State 15
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 16
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 17
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        30, // on (<Expr> ",")+, goto 29
        31, // on Comma<Expr>, goto 30
        32, // on Expr, goto 31
        0, // on Expr?, error
        33, // on Factor, goto 32
        34, // on Term, goto 33
        0, // on __Expr, error
        // State 18
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 19
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 20
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 21
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 22
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 23
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        38, // on Factor, goto 37
        14, // on Term, goto 13
        0, // on __Expr, error
        // State 24
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        39, // on Factor, goto 38
        14, // on Term, goto 13
        0, // on __Expr, error
        // State 25
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        40, // on Term, goto 39
        0, // on __Expr, error
        // State 26
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        41, // on Term, goto 40
        0, // on __Expr, error
        // State 27
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 28
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        30, // on (<Expr> ",")+, goto 29
        43, // on Comma<Expr>, goto 42
        32, // on Expr, goto 31
        0, // on Expr?, error
        33, // on Factor, goto 32
        34, // on Term, goto 33
        0, // on __Expr, error
        // State 29
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        44, // on Expr, goto 43
        0, // on Expr?, error
        33, // on Factor, goto 32
        34, // on Term, goto 33
        0, // on __Expr, error
        // State 30
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 31
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 32
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 33
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 34
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        51, // on Expr, goto 50
        0, // on Expr?, error
        13, // on Factor, goto 12
        14, // on Term, goto 13
        0, // on __Expr, error
        // State 35
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 36
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 37
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 38
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 39
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 40
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 41
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 42
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 43
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 44
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 45
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        55, // on Factor, goto 54
        34, // on Term, goto 33
        0, // on __Expr, error
        // State 46
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 47
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        56, // on Factor, goto 55
        34, // on Term, goto 33
        0, // on __Expr, error
        // State 48
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        57, // on Term, goto 56
        0, // on __Expr, error
        // State 49
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        58, // on Term, goto 57
        0, // on __Expr, error
        // State 50
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 51
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        30, // on (<Expr> ",")+, goto 29
        60, // on Comma<Expr>, goto 59
        32, // on Expr, goto 31
        0, // on Expr?, error
        33, // on Factor, goto 32
        34, // on Term, goto 33
        0, // on __Expr, error
        // State 52
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 53
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 54
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 55
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 56
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 57
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 58
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 59
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
        // State 60
        0, // on (<Expr> ","), error
        0, // on (<Expr> ",")*, error
        0, // on (<Expr> ",")+, error
        0, // on Comma<Expr>, error
        0, // on Expr, error
        0, // on Expr?, error
        0, // on Factor, error
        0, // on Term, error
        0, // on __Expr, error
    ];
    pub fn parse_Expr<
        'ast,
        __TOKEN: __ToTriple<'ast, Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        arena: &'ast Arena<'ast>,
        __tokens: __TOKENS,
    ) -> Result<&'ast Node<'ast>, __ParseError<usize,Tok,()>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            println!("outer loop");
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__ParseError::User { error: e }),
            };
            let __integer = match __lookahead {
                (_, Tok::LParen, _) if true => 0,
                (_, Tok::RParen, _) if true => 1,
                (_, Tok::Times, _) if true => 2,
                (_, Tok::Plus, _) if true => 3,
                (_, Tok::Comma, _) if true => 4,
                (_, Tok::Minus, _) if true => 5,
                (_, Tok::Div, _) if true => 6,
                (_, Tok::Num(_), _) if true => 7,
                _ => {
                    return Err(__ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                println!("inner loop");
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 8 + __integer];
                println!("state: {} lookahead: {} action: {} stack-depth: {}", __state, __integer, __action, __symbols.len());
                if __action > 0 {
                    println!("--> shift");
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Tok::LParen => __Symbol::Term_22_28_22(__tok),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Tok::RParen => __Symbol::Term_22_29_22(__tok),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Tok::Times => __Symbol::Term_22_2a_22(__tok),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Tok::Plus => __Symbol::Term_22_2b_22(__tok),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Tok::Comma => __Symbol::Term_22_2c_22(__tok),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Tok::Minus => __Symbol::Term_22_2d_22(__tok),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Tok::Div => __Symbol::Term_22_2f_22(__tok),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Tok::Num(__tok0) => __Symbol::TermNum(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    println!("--> reduce");
                    if let Some(r) = __reduce(arena, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
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
            println!("EOF loop state: {}", __state);
            let __action = __EOF_ACTION[__state];
            println!("EOF in state {} takes action {}", __state, __action);
            if __action < 0 {
                if let Some(r) = __reduce(arena, __action, None, &mut __states, &mut __symbols) {
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
        'ast,
    >(
        arena: &'ast Arena<'ast>,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>,
    ) -> Option<Result<&'ast Node<'ast>,__ParseError<usize,Tok,()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(15);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15(arena, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(13);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action13(arena, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(14);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(arena, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(18);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18(arena, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(19);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19(arena, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // Comma<Expr> = Expr => ActionFn(22);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(arena, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                3
            }
            7 => {
                // Comma<Expr> =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23(arena, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                3
            }
            8 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(24);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24(arena, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                3
            }
            9 => {
                // Comma<Expr> = (<Expr> ",")+ => ActionFn(25);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(arena, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                3
            }
            10 => {
                // Expr = Expr, "-", Factor => ActionFn(1);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            11 => {
                // Expr = Expr, "+", Factor => ActionFn(2);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            12 => {
                // Expr = Factor => ActionFn(3);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(arena, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            13 => {
                // Expr? = Expr => ActionFn(11);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(arena, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                5
            }
            14 => {
                // Expr? =  => ActionFn(12);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action12(arena, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                5
            }
            15 => {
                // Factor = Factor, "*", Term => ActionFn(4);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                6
            }
            16 => {
                // Factor = Factor, "/", Term => ActionFn(5);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                6
            }
            17 => {
                // Factor = "*", "(", Comma<Expr>, ")" => ActionFn(6);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtComma_3cExpr_3e(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                6
            }
            18 => {
                // Factor = Term => ActionFn(7);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(arena, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                6
            }
            19 => {
                // Term = Num => ActionFn(8);
                let __sym0 = __pop_TermNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(arena, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                7
            }
            20 => {
                // Term = "(", Expr, ")" => ActionFn(9);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                7
            }
            21 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(arena, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 9 + __nonterminal] - 1;
        println!("goto state {} from {} due to nonterminal {}", __next_state, __state, __nonterminal);
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, Tok, usize) {
        println!("pop_Term_22_28_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, Tok, usize) {
        println!("pop_Term_22_29_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, Tok, usize) {
        println!("pop_Term_22_2a_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, Tok, usize) {
        println!("pop_Term_22_2b_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, Tok, usize) {
        println!("pop_Term_22_2c_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, Tok, usize) {
        println!("pop_Term_22_2d_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, Tok, usize) {
        println!("pop_Term_22_2f_22");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermNum<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, i32, usize) {
        println!("pop_TermNum");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, &'ast Node<'ast>, usize) {
        println!("pop_Nt_28_3cExpr_3e_20_22_2c_22_29");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize) {
        println!("pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize) {
        println!("pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, Vec<&'ast Node<'ast>>, usize) {
        println!("pop_NtComma_3cExpr_3e");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, &'ast Node<'ast>, usize) {
        println!("pop_NtExpr");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_3f<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, ::std::option::Option<&'ast Node<'ast>>, usize) {
        println!("pop_NtExpr_3f");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, &'ast Node<'ast>, usize) {
        println!("pop_NtFactor");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, &'ast Node<'ast>, usize) {
        println!("pop_NtTerm");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'ast,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'ast>,usize)>
    ) -> (usize, &'ast Node<'ast>, usize) {
        println!("pop_Nt____Expr");
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Expr::parse_Expr;

pub fn __action0<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action1<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, l, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
    (_, r, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Sub, l, r))
}

pub fn __action2<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, l, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
    (_, r, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Add, l, r))
}

pub fn __action3<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action4<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, l, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
    (_, r, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Mul, l, r))
}

pub fn __action5<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, l, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
    (_, r, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Div, l, r))
}

pub fn __action6<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, _, _): (usize, Tok, usize),
    (_, _, _): (usize, Tok, usize),
    (_, __0, _): (usize, Vec<&'ast Node<'ast>>, usize),
    (_, _, _): (usize, Tok, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Reduce(Op::Mul, __0))
}

pub fn __action7<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action8<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, n, _): (usize, i32, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Value(n))
}

pub fn __action9<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, _, _): (usize, Tok, usize),
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Paren(__0))
}

pub fn __action10<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, h, _): (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
    (_, t, _): (usize, ::std::option::Option<&'ast Node<'ast>>, usize),
) -> Vec<&'ast Node<'ast>>
{
    h.into_iter().chain(t).collect()
}

pub fn __action11<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
) -> ::std::option::Option<&'ast Node<'ast>>
{
    Some(__0)
}

pub fn __action12<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<&'ast Node<'ast>>
{
    None
}

pub fn __action13<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    vec![]
}

pub fn __action14<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, v, _): (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    v
}

pub fn __action15<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
    (_, _, _): (usize, Tok, usize),
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action16<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, __0, _): (usize, &'ast Node<'ast>, usize),
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    vec![__0]
}

pub fn __action17<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    (_, v, _): (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
    (_, e, _): (usize, &'ast Node<'ast>, usize),
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action18<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, &'ast Node<'ast>, usize),
    __1: (usize, Tok, usize),
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action15(
        arena,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        arena,
        __temp0,
    )
}

pub fn __action19<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
    __1: (usize, &'ast Node<'ast>, usize),
    __2: (usize, Tok, usize),
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action15(
        arena,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        arena,
        __0,
        __temp0,
    )
}

pub fn __action20<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, ::std::option::Option<&'ast Node<'ast>>, usize),
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action13(
        arena,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        arena,
        __temp0,
        __0,
    )
}

pub fn __action21<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
    __1: (usize, ::std::option::Option<&'ast Node<'ast>>, usize),
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action14(
        arena,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        arena,
        __temp0,
        __1,
    )
}

pub fn __action22<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, &'ast Node<'ast>, usize),
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action11(
        arena,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        arena,
        __temp0,
    )
}

pub fn __action23<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action12(
        arena,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        arena,
        __temp0,
    )
}

pub fn __action24<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
    __1: (usize, &'ast Node<'ast>, usize),
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action11(
        arena,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        arena,
        __0,
        __temp0,
    )
}

pub fn __action25<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: (usize, ::std::vec::Vec<&'ast Node<'ast>>, usize),
) -> Vec<&'ast Node<'ast>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action12(
        arena,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        arena,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'ast, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),Self::Error>;
}

impl<'ast, > __ToTriple<'ast, > for (usize, Tok, usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),()> {
        Ok(value)
    }
}
impl<'ast, > __ToTriple<'ast, > for Result<(usize, Tok, usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,Tok,usize),()> {
        value
    }
}
