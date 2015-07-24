use intern::{intern, InternedString};
use grammar::parse_tree::*;
use grammar::pattern::*;
use rusty_peg;

#[cfg(test)]
mod test;

fn make_list<T>(head: Vec<(T, &'static str)>,
                tail: Option<T>)
                -> Vec<T> {
    head.into_iter().map(|(a,_)| a)
                    .chain(tail)
                    .collect()
}

rusty_peg! {
    parser Parser<'input> {
        // Grammar
        GRAMMAR: Grammar =
            (<uses:{USE}>
             <lo:POSL> "grammar" <hi:POSR>
             <tps:[GRAMMAR_TPS]>
             <parameters:[GRAMMAR_PARAMS]>
             <where_clauses:[WHERE_CLAUSES]>
             ";" <i:{GRAMMAR_ITEM}>) => {
                Grammar { span: Span(lo, hi),
                          type_parameters: tps.unwrap_or(vec![]),
                          parameters: parameters.unwrap_or(vec![]),
                          where_clauses: where_clauses.unwrap_or(vec![]),
                          items: uses.into_iter().chain(i).collect() }
            };

        GRAMMAR_TPS: Vec<TypeParameter> =
            ("<" <h:{TYPE_PARAMETER ","}> <t:[TYPE_PARAMETER]> ">") => make_list(h, t);

        TYPE_PARAMETER: TypeParameter =
            (LIFETIME_TYPE_PARAMETER / ID_TYPE_PARAMETER);

        LIFETIME_TYPE_PARAMETER: TypeParameter =
            (<l:LIFETIME>) => TypeParameter::Lifetime(l);

        ID_TYPE_PARAMETER: TypeParameter =
            (<l:ID>) => TypeParameter::Id(l);

        GRAMMAR_PARAMS: Vec<Parameter> =
            ("(" <h:{GRAMMAR_PARAM ","}> <t:[GRAMMAR_PARAM]> ")") => make_list(h, t);

        GRAMMAR_PARAM: Parameter =
            (<id:ID> ":" <t:TYPE_REF>) => Parameter { name: id, ty: t };

        WHERE_CLAUSES: Vec<String> =
            ("where" <h:{TYPE ","}> <t:[TYPE]>) => make_list(h, t);

        GRAMMAR_ITEM: GrammarItem =
            (EXTERN_TOKEN / NONTERMINAL / USE);

        USE: GrammarItem =
            ("use" <c:CODE> ";") => GrammarItem::Use(c);

        NONTERMINAL: GrammarItem =
            (<p:[NONTERMINAL_PUB]>
             <lo:POSL> <n:NONTERMINAL_NAME> <hi:POSR>
             <t:[NONTERMINAL_TYPE]> "=" <a:ALTERNATIVES>) => {
                GrammarItem::Nonterminal(NonterminalData { public: p.is_some(),
                                                           span: Span(lo, hi),
                                                           name: n.0,
                                                           args: n.1,
                                                           type_decl: t,
                                                           alternatives: a })
            };

        NONTERMINAL_PUB: () =
            "pub" => ();

        NONTERMINAL_NAME: (NonterminalString, Vec<NonterminalString>) =
            (NONTERMINAL_NAME_MACRO / NONTERMINAL_NAME_SIMPLE / NONTERMINAL_NAME_ESCAPE);

        NONTERMINAL_NAME_SIMPLE: (NonterminalString, Vec<NonterminalString>) =
            (<a:NONTERMINAL_ID>) => (a, vec![]);

        NONTERMINAL_NAME_ESCAPE: (NonterminalString, Vec<NonterminalString>) =
            (<a:ESCAPE>) => (NonterminalString(a), vec![]);

        NONTERMINAL_NAME_MACRO: (NonterminalString, Vec<NonterminalString>) =
            (<a:NONTERMINAL_ID> "<" <b:{NONTERMINAL_ID ","}> <c:[NONTERMINAL_ID]> ">") => {
                (a, make_list(b, c))
            };

        NONTERMINAL_TYPE: TypeRef =
            (":" <s:TYPE_REF>) => s;

        ALTERNATIVES: Vec<Alternative> =
            (ALTERNATIVES1 / ALTERNATIVESN);

        ALTERNATIVES1: Vec<Alternative> =
            (<a:ALTERNATIVE>) => vec![a];

        ALTERNATIVESN: Vec<Alternative> =
            ("{" <a:{ALTERNATIVE}> "}" ";") => a;

        ALTERNATIVE: Alternative =
            (<lo:POSL> <s:EXPR_SYMBOL> <c:[IF_COND]> <a:[ACTION]> ";" <hi:POSR>) => Alternative {
                span: Span(lo, hi),
                expr: s,
                condition: c,
                action: a
            };

        IF_COND: Condition =
            ("if" <c:COND>) => c;

        ACTION: ActionKind =
            (LOOKAHEAD_ACTION / LOOKBEHIND_ACTION / FALLIBLE_USER_ACTION / USER_ACTION);

        USER_ACTION: ActionKind =
            ("=>" <b:CODE>) => ActionKind::User(b);

        FALLIBLE_USER_ACTION: ActionKind =
            ("=>?" <b:CODE>) => ActionKind::Fallible(b);

        LOOKAHEAD_ACTION: ActionKind =
            ("=>@L") => ActionKind::Lookahead;

        LOOKBEHIND_ACTION: ActionKind =
            ("=>@R") => ActionKind::Lookbehind;

        // Conditions

        COND: Condition =
            (<lo:POSL> <a:NONTERMINAL_ID> <op:COND_OP> <b:LITERAL> <hi:POSR>) => {
                Condition { span:Span(lo, hi), lhs:a, rhs:b, op:op }
            };

        COND_OP: ConditionOp = (EQUALS_OP / NOT_EQUALS_OP / MATCH_OP / NOT_MATCH_OP);
        EQUALS_OP: ConditionOp = "==" => ConditionOp::Equals;
        NOT_EQUALS_OP: ConditionOp = "!=" => ConditionOp::NotEquals;
        MATCH_OP: ConditionOp = "~~" => ConditionOp::Match;
        NOT_MATCH_OP: ConditionOp = "!~" => ConditionOp::NotMatch;

        // Symbols

        SYMBOL: Symbol =
            (NAMED_SYMBOL / CHOSEN_SYMBOL / SYMBOL0);

        NAMED_SYMBOL: Symbol =
            (<lo:POSL> "<" <l:ID> ":" <s:SYMBOL0> <hi:POSR> ">") => {
                Symbol::new(Span(lo, hi), SymbolKind::Name(l, Box::new(s)))
            };

        CHOSEN_SYMBOL: Symbol =
            (<lo:POSL> "<" <s:SYMBOL0> ">" <hi:POSR>) => {
                Symbol::new(Span(lo, hi), SymbolKind::Choose(Box::new(s)))
            };

        SYMBOL0: Symbol =
            fold(<lhs:SYMBOL1>,
                 (<lo:POSL> <op:REPEAT_OP> <hi:POSR>) => {
                     Symbol::new(Span(lo, hi),
                                 SymbolKind::Repeat(Box::new(RepeatSymbol { symbol: lhs,
                                                                            op: op })))
                 });

        REPEAT_OP: RepeatOp = (REPEAT_OP_PLUS / REPEAT_OP_STAR / REPEAT_OP_QUESTION);
        REPEAT_OP_PLUS: RepeatOp = "+" => RepeatOp::Plus;
        REPEAT_OP_STAR: RepeatOp = "*" => RepeatOp::Star;
        REPEAT_OP_QUESTION: RepeatOp = "?" => RepeatOp::Question;

        SYMBOL1: Symbol =
            (MACRO_SYMBOL / TERMINAL_SYMBOL / NT_SYMBOL / ESCAPE_SYMBOL / PAREN_SYMBOL/
             LOOKAHEAD_SYMBOL / LOOKBEHIND_SYMBOL);

        MACRO_SYMBOL: Symbol =
            (<lo:POSL> <l:MACRO_ID> <m:{SYMBOL ","}> <n:[SYMBOL]> ">" <hi:POSR>) => {
                Symbol::new(Span(lo, hi),
                            SymbolKind::Macro(MacroSymbol { name: l,
                                                            args: make_list(m, n) }))
            };

        TERMINAL_SYMBOL: Symbol =
            (<lo:POSL> <l:TERMINAL> <hi:POSR>) => {
                Symbol::new(Span(lo, hi), SymbolKind::Terminal(l))
            };

        NT_SYMBOL: Symbol =
            (<lo:POSL> <l:NONTERMINAL_ID> <hi:POSR>) => {
                Symbol::new(Span(lo, hi), SymbolKind::Nonterminal(l))
            };

        ESCAPE_SYMBOL: Symbol =
            (<lo:POSL> <l:ESCAPE> <hi:POSR>) => {
                Symbol::new(Span(lo, hi), SymbolKind::Nonterminal(NonterminalString(l)))
            };

        PAREN_SYMBOL: Symbol =
            (<lo:POSL> "(" <s:EXPR_SYMBOL> ")" <hi:POSR>) => {
                Symbol::new(Span(lo, hi), SymbolKind::Expr(s))
            };

        LOOKAHEAD_SYMBOL: Symbol =
            (<lo:POSL> "@L" <hi:POSR>) => {
                Symbol::new(Span(lo, hi), SymbolKind::Lookahead)
            };

        LOOKBEHIND_SYMBOL: Symbol =
            (<lo:POSL> "@R" <hi:POSR>) => {
                Symbol::new(Span(lo, hi), SymbolKind::Lookbehind)
            };

        EXPR_SYMBOL: ExprSymbol =
            (<s:{SYMBOL}>) => ExprSymbol { symbols: s };

        // TypeRef

        TYPE_REF: TypeRef =
            (TUPLE_TYPE_REF / LIFETIME_TYPE_REF / NOMINAL_TYPE_REF /
             REF_TYPE_REF / REF_MUT_TYPE_REF / ESCAPE_TYPE_REF);

        TUPLE_TYPE_REF: TypeRef =
            ("(" <l:TYPE_REF_LIST> ")") => TypeRef::Tuple(l);

        LIFETIME_TYPE_REF: TypeRef =
            (<l:LIFETIME>) => TypeRef::Lifetime(l);

        ESCAPE_TYPE_REF: TypeRef =
            ("`" <s:SYMBOL> "`") => TypeRef::OfSymbol(s.kind);

        REF_TYPE_REF: TypeRef =
            ("&" <l:[LIFETIME]> <t:TYPE_REF>) => TypeRef::Ref { lifetime: l,
                                                                mutable: false,
                                                                referent: Box::new(t) };

        REF_MUT_TYPE_REF: TypeRef =
            ("&" <l:[LIFETIME]> "mut" <t:TYPE_REF>) => TypeRef::Ref { lifetime: l,
                                                                      mutable: true,
                                                                      referent: Box::new(t) };

        NOMINAL_TYPE_REF: TypeRef =
            (<p:PATH> <a:[NOMINAL_TYPE_REF_ARGS]>) => {
                match p.as_id() {
                    Some(id) if a.is_none() =>
                        // detect something like `Foo` and treat it specially,
                        // so that macro expansion can pattern match here
                        TypeRef::Id(id),
                    _ =>
                        // otherwise, `Vec<..>` or `Foo::Bar` etc expand to
                        // this full path
                        TypeRef::Nominal { path: p, types: a.unwrap_or(vec![]) }
                }
            };

        NOMINAL_TYPE_REF_ARGS: Vec<TypeRef> =
            ("<" <l:TYPE_REF_LIST> ">") => l;

        TYPE_REF_LIST: Vec<TypeRef> =
            (<a:{TYPE_REF_COMMA}> <t:[TYPE_REF]>) => {
                let mut a = a;
                a.extend(t.into_iter());
                a
            };

        TYPE_REF_COMMA: TypeRef =
            (<t:TYPE_REF> ",") => t;

        PATH: Path =
            (<a:["::"]> <h:{ID "::"}> <t:ID>) => {
                Path { absolute: a.is_some(),
                       ids: make_list(h, Some(t)) }
            };

        // TOKEN DEFINITIONS

        EXTERN_TOKEN: GrammarItem =
            (<lo0:POSL> "extern" "token" <hi0:POSR> "{"
               <a0:{ASSOCIATED_TYPE}>
               "enum" <lo:POSL> <t:TYPE_REF> <hi:POSR> "{"
                 <c0:{CONVERSION ","}> <c1:[CONVERSION]>
               "}"
               <a1:{ASSOCIATED_TYPE}>
             "}") => {
                GrammarItem::ExternToken(ExternToken {
                    span: Span(lo0, hi0),
                    associated_types: a0.into_iter().chain(a1).collect(),
                    enum_token: EnumToken {
                        type_name: t,
                        type_span: Span(lo, hi),
                        conversions: make_list(c0, c1)
                    }
                })
            };

        ASSOCIATED_TYPE: AssociatedType =
            ("type" <lo1:POSL> <n:ID> <hi1:POSR> "=" <t:TYPE_REF> ";") => {
                AssociatedType { type_span: Span(lo1, hi1),
                                 type_name: n,
                                 type_ref: t }
            };

        CONVERSION: Conversion =
            (<lo:POSL> <from:TERMINAL> "=>" <to:PATTERN> <hi:POSR>) => {
                Conversion { span: Span(lo, hi), from: from, to: to }
            };

        PATTERN: Pattern<TypeRef> =
            (ENUM_PATTERN / STRUCT_PATTERN0 / STRUCT_PATTERN1 / UNDERSCORE_PATTERN /
             DOTDOT_PATTERN / CHOOSE_PATTERN / TUPLE_PATTERN / PATH_PATTERN);

        ENUM_PATTERN: Pattern<TypeRef> =
            (<lo:POSL> <p:PATH> "(" <s0:{PATTERN ","}> <s1:[PATTERN]> ")" <hi:POSR>) => {
                Pattern { span: Span(lo, hi),
                          kind: PatternKind::Enum(p, make_list(s0, s1)) }
            };

        STRUCT_PATTERN0: Pattern<TypeRef> =
            (<lo:POSL> <p:PATH> "{" <s0:{FIELD_PATTERN ","}>
             <s1:[FIELD_PATTERN]> "}" <hi:POSR>) => {
                Pattern { span: Span(lo, hi),
                          kind: PatternKind::Struct(p,
                                                    make_list(s0, s1),
                                                    false) }
            };

        STRUCT_PATTERN1: Pattern<TypeRef> =
            (<lo:POSL> <p:PATH> "{" <s0:{FIELD_PATTERN ","}> ".." "}" <hi:POSR>) => {
                Pattern { span: Span(lo, hi),
                          kind: PatternKind::Struct(p,
                                                    make_list(s0, None),
                                                    true) }
            };

        FIELD_PATTERN: FieldPattern<TypeRef> =
            (<lo:POSL> <id:ID> <hi:POSR> ":" <pat:PATTERN>) => {
                FieldPattern { field_span: Span(lo, hi),
                               field_name: id,
                               pattern: pat }
            };

        UNDERSCORE_PATTERN: Pattern<TypeRef> =
            (<lo:POSL> "_" <hi:POSR>) => {
                Pattern { span: Span(lo, hi),
                          kind: PatternKind::Underscore }
            };

        DOTDOT_PATTERN: Pattern<TypeRef> =
            (<lo:POSL> ".." <hi:POSR>) => {
                Pattern { span: Span(lo, hi),
                          kind: PatternKind::DotDot }
            };

        CHOOSE_PATTERN: Pattern<TypeRef> =
            (<lo:POSL> "<" <t:TYPE_REF> ">" <hi:POSR>) => {
                Pattern { span: Span(lo, hi),
                          kind: PatternKind::Choose(t) }
            };

        TUPLE_PATTERN: Pattern<TypeRef> =
            (<lo:POSL> "(" <p0:{PATTERN ","}> <p1:[PATTERN]> ")" <hi:POSR>) => {
                Pattern { span: Span(lo, hi),
                          kind: PatternKind::Tuple(make_list(p0, p1)) }
            };

        PATH_PATTERN: Pattern<TypeRef> =
            (<lo:POSL> <p:PATH> <hi:POSR>) => {
                Pattern { span: Span(lo, hi), kind: PatternKind::Path(p) }
            };

        // IDENTIFIERS, LIFETIMES

        MACRO_ID: NonterminalString =
            (<i:MACRO_ID_RE>) => NonterminalString(intern(&i[..i.len()-1]));

        MACRO_ID_RE: &'input str =
            regex(r"[a-zA-Z_][a-zA-Z0-9_]*<");

        NONTERMINAL_ID: NonterminalString =
            (<i:ID>) => NonterminalString(i);

        ID: InternedString =
            (<i:ID_RE>) => intern(i);

        ID_RE: &'input str =
            regex(r"[a-zA-Z_][a-zA-Z0-9_]*") - [
                "if", "use", "where", "token", "grammar", "pub", "struct", "extern", "enum",
                "mut"
            ];

        ESCAPE: InternedString =
            (<i:ESCAPE_RE>) => intern(&i[1..i.len()-1]);

        ESCAPE_RE: &'input str =
            regex(r"`[^`]*`");

        LIFETIME: InternedString =
            (<i:LIFETIME_RE>) => intern(i);

        LIFETIME_RE: &'input str =
            regex(r"'[a-zA-Z_][a-zA-Z0-9_]*");

        TERMINAL: TerminalString =
            (<l:LITERAL>) => TerminalString(l);

        LITERAL: InternedString =
            (<i:LITERAL_RE>) => intern(&i[1..i.len()-1]);

        LITERAL_RE: &'input str =
            regex(r#""[^"]*""#); // TODO
    }
}

// Custom symbols.

struct CODE;
struct TYPE;

impl<'input> rusty_peg::Symbol<'input,Parser<'input>> for CODE {
    type Output = String;

    fn parse(&self, _: &mut Parser<'input>, input: rusty_peg::Input<'input>)
             -> rusty_peg::ParseResult<'input,String>
    {
        parse_code(input, false)
    }
}

impl<'input> rusty_peg::Symbol<'input,Parser<'input>> for TYPE {
    type Output = String;

    fn parse(&self, _: &mut Parser<'input>, input: rusty_peg::Input<'input>)
             -> rusty_peg::ParseResult<'input,String>
    {
        parse_code(input, true)
    }
}

fn parse_code<'input>(input: rusty_peg::Input<'input>, type_pos: bool)
                      -> rusty_peg::ParseResult<'input,String> {
    let bytes = input.text.as_bytes();
    let mut offset = input.offset;
    let mut balance: u32 = 0;
    let mut in_string: bool = false;

    while offset < input.text.len() {
        let cur_byte = bytes[offset];

        // FIXME -- really we need a more sophisticated tokenization
        // scheme here to accommodate `r#` and so forth!

        if in_string {
            // inside of a string, allow anything and look for '"'
            if cur_byte == '\\' as u8 && offset < input.text.len() - 1 {
                offset += 1; // skip over escape sequences like \"
            } else if cur_byte == '"' as u8 {
                in_string = false;
            }
        } else {
            if type_pos {
                match cur_byte as char {
                    '{' if balance == 0 => break,
                    '<' => balance += 1,
                    '>' if balance == 0 => break,
                    '>' => balance -= 1,
                    _ => { /* fallthrough to the match below */ }
                }
            }

            // otherwise, we are inside regular code, so track {}, [], or () pairs
            match cur_byte as char {
                '{' | '(' | '[' => balance += 1,
                '"' => in_string = true,
                '}' | ')' | ']' | ',' | ';' if balance == 0 => break,
                '}' | ')' | ']' => balance -= 1,
                _ => { }
            }
        }

        offset += 1; // move to next byte
    }

    let regex_str = &input.text[input.offset .. offset];
    let output = rusty_peg::Input { text: input.text, offset: offset };
    return Ok((output, regex_str.to_string()));
}

pub fn parse_grammar(text: &str) -> Result<Grammar,rusty_peg::Error> {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&GRAMMAR, &mut parser, text)
}

#[cfg(test)]
fn parse_alternative(text: &str) -> Result<Alternative,rusty_peg::Error> {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&ALTERNATIVE, &mut parser, text)
}

#[cfg(test)]
fn parse_symbol(text: &str) -> Result<Symbol,rusty_peg::Error> {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&SYMBOL, &mut parser, text)
}

#[cfg(test)]
fn parse_nonterminal(text: &str) -> Result<GrammarItem,rusty_peg::Error> {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&NONTERMINAL, &mut parser, text)
}

#[cfg(test)]
pub fn parse_type_ref(text: &str) -> Result<TypeRef,rusty_peg::Error> {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&TYPE_REF, &mut parser, text)
}
