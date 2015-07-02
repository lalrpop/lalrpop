use intern::{intern, InternedString};
use grammar::parse_tree::*;
use rusty_peg;

#[cfg(test)]
mod test;

fn make_list<T>(head: Vec<(T, &'static str)>,
                tail: Option<(T, Option<&'static str>)>)
                -> Vec<T> {
    head.into_iter().map(|(a,_)| a)
                    .chain(tail.map(|(a, _)| a).into_iter())
                    .collect()
}

rusty_peg! {
    parser Parser<'input> {
        // Grammar
        GRAMMAR: Grammar =
            (<lo:POSL> "grammar" <hi:POSR>
             <tps:[GRAMMAR_TPS]>
             <parameters:[GRAMMAR_PARAMS]>
             <where_clauses:[WHERE_CLAUSES]>
             "{" <i:{GRAMMAR_ITEM}> "}") => {
                Grammar { span: Span(lo, hi),
                          type_parameters: tps.unwrap_or(vec![]),
                          parameters: parameters.unwrap_or(vec![]),
                          where_clauses: where_clauses.unwrap_or(vec![]),
                          items: i }
            };

        GRAMMAR_TPS: Vec<TypeParameter> =
            ("<" <h:{TYPE_PARAMETER ","}> <t:[TYPE_PARAMETER [","]]> ">") => make_list(h, t);

        TYPE_PARAMETER: TypeParameter =
            (LIFETIME_TYPE_PARAMETER / ID_TYPE_PARAMETER);

        LIFETIME_TYPE_PARAMETER: TypeParameter =
            (<l:LIFETIME>) => TypeParameter::Lifetime(l);

        ID_TYPE_PARAMETER: TypeParameter =
            (<l:ID>) => TypeParameter::Id(l);

        GRAMMAR_PARAMS: Vec<Parameter> =
            ("(" <h:{GRAMMAR_PARAM ","}> <t:[GRAMMAR_PARAM [","]]> ")") => make_list(h, t);

        GRAMMAR_PARAM: Parameter =
            (<id:ID> ":" <t:TYPE_REF>) => Parameter { name: id, ty: t };

        WHERE_CLAUSES: Vec<String> =
            ("where" <h:{TYPE ","}> <t:[TYPE [","]]>) => make_list(h, t);

        GRAMMAR_ITEM: GrammarItem =
            (TOKEN_TYPE / NONTERMINAL / USE);

        TOKEN_TYPE: GrammarItem =
            ("token" <t:TYPE_REF> "where" "{" <c:{CONVERSION}> "}" ";") => {
                GrammarItem::TokenType(TokenTypeData {type_name: t, conversions: c })
            };

        CONVERSION: (TerminalString, TerminalString) =
            (<from:TERMINAL> "=>" <to:TERMINAL> ";") => (from, to);

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
            (<a:NONTERMINAL_ID> "<" <b:{NONTERMINAL_ID ","}> <c:[NONTERMINAL_ID [","]]> ">") => {
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

        ACTION: String = ("=>" <b:CODE>) => b;

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
            (<lo:POSL> <l:ID> ":" <s:SYMBOL0> <hi:POSR>) => {
                Symbol::new(Span(lo, hi), SymbolKind::Name(l, Box::new(s)))
            };

        CHOSEN_SYMBOL: Symbol =
            (<lo:POSL> "~" <s:SYMBOL0> <hi:POSR>) => {
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
            (MACRO_SYMBOL / TERMINAL_SYMBOL / NT_SYMBOL / ESCAPE_SYMBOL / PAREN_SYMBOL);

        MACRO_SYMBOL: Symbol =
            (<lo:POSL> <l:NONTERMINAL_ID> "<"
             <m:{SYMBOL ","}> <n:[SYMBOL [","]]> ">" <hi:POSR>) => {
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

        EXPR_SYMBOL: ExprSymbol =
            (<s:{SYMBOL}>) => ExprSymbol { symbols: s };

        // TypeRef

        TYPE_REF: TypeRef =
            (TUPLE_TYPE_REF / LIFETIME_TYPE_REF / NOMINAL_TYPE_REF / ESCAPE_TYPE_REF);

        TUPLE_TYPE_REF: TypeRef =
            ("(" <l:TYPE_REF_LIST> ")") => TypeRef::Tuple(l);

        LIFETIME_TYPE_REF: TypeRef =
            (<l:LIFETIME>) => TypeRef::Lifetime(l);

        ESCAPE_TYPE_REF: TypeRef =
            ("`" <s:SYMBOL> "`") => TypeRef::OfSymbol(s.kind);

        NOMINAL_TYPE_REF: TypeRef =
            (<p:PATH> <a:[NOMINAL_TYPE_REF_ARGS]>) => {
                if p.len() == 1 && a.is_none() {
                    // detect something like `Foo` and treat it specially,
                    // so that macro expansion can pattern match here
                    TypeRef::Id(p.into_iter().next().unwrap())
                } else {
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

        PATH: Vec<InternedString> =
            (<b:{PATH_BASE}> <c:ID>) => {
                let mut b = b;
                b.push(c);
                b
            };

        PATH_BASE: InternedString =
            (<i:ID> "::") => i;

        // IDENTIFIERS, LIFETIMES

        NONTERMINAL_ID: NonterminalString =
            (<i:ID>) => NonterminalString(i);

        ID: InternedString =
            (<i:ID_RE>) => intern(i);

        ID_RE: &'input str =
            regex(r"[a-zA-Z_][a-zA-Z0-9_]*") - [
                "if", "use", "where", "token", "grammar", "pub", "struct"
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
