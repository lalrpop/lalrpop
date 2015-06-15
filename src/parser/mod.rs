use intern::{intern, InternedString};
use grammar::parse_tree::*;
use rusty_peg;

#[cfg(test)]
mod test;

rusty_peg! {
    parser Parser<'input> {
        // Grammar
        GRAMMAR: Grammar =
            ("grammar" <t:TYPE_REF> "{" <i:{GRAMMAR_ITEM}> "}") => {
                Grammar { type_name: t, items: i }
            };

        GRAMMAR_ITEM: GrammarItem =
            (TOKEN_TYPE / NONTERMINAL);

        TOKEN_TYPE: GrammarItem =
            ("token" <t:TYPE_REF> "where" "{" <c:{CONVERSION}> "}" ";") => {
                GrammarItem::TokenType(TokenTypeData {type_name: t, conversions: c })
            };

        CONVERSION: (InternedString, InternedString) =
            (<from:LITERAL> "=>" <to:LITERAL> ";") => (from, to);

        NONTERMINAL: GrammarItem =
            (<n:NONTERMINAL_NAME> <t:[NONTERMINAL_TYPE]> "=" <a:ALTERNATIVES>) => {
                GrammarItem::Nonterminal(NonterminalData { name: n.0,
                                                           args: n.1,
                                                           type_decl: t,
                                                           alternatives: a })
            };

        NONTERMINAL_NAME: (InternedString, Vec<InternedString>) =
            (NONTERMINAL_NAME_MACRO / NONTERMINAL_NAME_SIMPLE);

        NONTERMINAL_NAME_SIMPLE: (InternedString, Vec<InternedString>) =
            (<a:ID>) => (a, vec![]);

        NONTERMINAL_NAME_MACRO: (InternedString, Vec<InternedString>) =
            (<a:ID> "<" <b:{NONTERMINAL_NAME_MACRO1}> <c:[ID]> ">") => {
                let mut args = b;
                if let Some(c) = c { args.push(c); }
                (a, args)
            };

        NONTERMINAL_NAME_MACRO1: InternedString =
            (<a:ID> ",") => a;

        NONTERMINAL_TYPE: TypeRef =
            (":" <s:TYPE_REF>) => s;

        ALTERNATIVES: Vec<Alternative> =
            (ALTERNATIVES1 / ALTERNATIVESN);

        ALTERNATIVES1: Vec<Alternative> =
            (<a:ALTERNATIVE>) => vec![a];

        ALTERNATIVESN: Vec<Alternative> =
            ("{" <a:{ALTERNATIVE}> "}" ";") => a;

        ALTERNATIVE: Alternative =
            (<s:{SYMBOL}> <c:[IF_COND]> <a:[ACTION]> ";") => Alternative {
                expr: s,
                condition: c,
                action: a
            };

        IF_COND: Condition =
            ("if" <c:COND>) => c;

        ACTION: Action = ("=>" <b:CODE>) => Action::User(b);

        // Conditions

        COND: Condition =
            (<lo:POSL> <a:ID> <op:COND_OP> <b:LITERAL> <hi:POSR>) => {
                Condition { span:Span(lo, hi), lhs:a, rhs:b, op:op }
            };

        COND_OP: ConditionOp = (EQUALS_OP / NOT_EQUALS_OP / MATCH_OP / NOT_MATCH_OP);
        EQUALS_OP: ConditionOp = "==" => ConditionOp::Equals;
        NOT_EQUALS_OP: ConditionOp = "!=" => ConditionOp::NotEquals;
        MATCH_OP: ConditionOp = "~~" => ConditionOp::Match;
        NOT_MATCH_OP: ConditionOp = "!~" => ConditionOp::NotMatch;

        // Symbols

        SYMBOL: Symbol =
            fold(<lhs:SYMBOL0>,
                 "+" => Symbol::Plus(Box::new(lhs)),
                 "*" => Symbol::Star(Box::new(lhs)),
                 "?" => Symbol::Question(Box::new(lhs)));

        SYMBOL0: Symbol =
            (MACRO_SYMBOL / TERMINAL_SYMBOL / NT_SYMBOL / EXPR_SYMBOL /
             NAMED_SYMBOL / CHOSEN_SYMBOL);

        MACRO_SYMBOL: Symbol =
            (<lo:POSL> <l:ID> "<" <m:{MACRO_ARG_START}> <n:[SYMBOL]> ">" <hi:POSR>) => {
                let mut args = m;
                if let Some(n) = n { args.push(n); }
                Symbol::Macro(MacroSymbol { name: l,
                                            args: args,
                                            span: Span(lo, hi), })
            };

        MACRO_ARG_START: Symbol = (<s:SYMBOL> ",") => s;

        TERMINAL_SYMBOL: Symbol =
            (<l:LITERAL>) => Symbol::Terminal(l);

        NT_SYMBOL: Symbol =
            (<l:ID>) => Symbol::Nonterminal(l);

        EXPR_SYMBOL: Symbol =
            ("(" <s:{SYMBOL}> ")") => Symbol::Expr(s);

        NAMED_SYMBOL: Symbol =
            ("~" <l:ID> ":" <s:SYMBOL>) => Symbol::Name(l, Box::new(s));

        CHOSEN_SYMBOL: Symbol =
            ("~" <s:SYMBOL>) => Symbol::Choose(Box::new(s));

        // TypeRef

        TYPE_REF: TypeRef =
            (TUPLE_TYPE_REF / LIFETIME_TYPE_REF / NOMINAL_TYPE_REF);

        TUPLE_TYPE_REF: TypeRef =
            ("(" <l:TYPE_REF_LIST> ")") => TypeRef::Tuple(l);

        LIFETIME_TYPE_REF: TypeRef =
            (<l:LIFETIME>) => TypeRef::Lifetime(l);

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

        ID: InternedString =
            (<i:ID_RE>) => intern(i);

        ID_RE: &'input str =
            regex(r"[a-zA-Z_][a-zA-Z0-9_]*") - ["if"];

        LIFETIME: InternedString =
            (<i:LIFETIME_RE>) => intern(i);

        LIFETIME_RE: &'input str =
            regex(r"'[a-zA-Z_][a-zA-Z0-9_]*");

        LITERAL: InternedString =
            (<i:LITERAL_RE>) => intern(i);

        LITERAL_RE: &'input str =
            regex(r#""[^"]*""#); // TODO
    }
}

// Custom symbols.

struct CODE;

impl<'input> rusty_peg::Symbol<'input,Parser<'input>> for CODE {
    type Output = String;

    fn parse(&self, _: &mut Parser<'input>, input: rusty_peg::Input<'input>)
             -> rusty_peg::ParseResult<'input,String>
    {
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

        let regex_str = &input.text[input.offset + 1 .. offset - 1];
        let output = rusty_peg::Input { text: input.text, offset: offset };
        return Ok((output, regex_str.to_string()));
    }
}

pub fn parse_grammar(text: &str) -> Result<Grammar,rusty_peg::Error> {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&GRAMMAR, &mut parser, text)
}

fn parse_alternative(text: &str) -> Result<Alternative,rusty_peg::Error> {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&ALTERNATIVE, &mut parser, text)
}

fn parse_symbol(text: &str) -> Result<Symbol,rusty_peg::Error> {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&SYMBOL, &mut parser, text)
}

fn parse_nonterminal(text: &str) -> Result<GrammarItem,rusty_peg::Error> {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&NONTERMINAL, &mut parser, text)
}

fn parse_type_ref(text: &str) -> Result<TypeRef,rusty_peg::Error> {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&TYPE_REF, &mut parser, text)
}
