use intern::{intern, InternedString};
use grammar::ty::TypeName;
use grammar::parse_tree::*;
use rusty_peg;

#[cfg(test)]
mod test;

rusty_peg! {
    parser Parser<'input> {
        // Grammar
        GRAMMAR: Grammar =
            ("grammar", <t:TYPE_NAME>, "{", <i:{GRAMMAR_ITEM}>, "}") => {
                Grammar { type_name: t, items: i }
            };

        GRAMMAR_ITEM: GrammarItem =
            (TOKEN_TYPE / NONTERMINAL);

        TOKEN_TYPE: GrammarItem =
            ("token", <t:TYPE_NAME>, "where", "{", <c:{CONVERSION}>, "}") => {
                GrammarItem::TokenType(TokenTypeData {type_name: t, conversions: c })
            };

        CONVERSION: (InternedString, InternedString) =
            (<from:LITERAL>, "=>", <to:LITERAL>, ";") => (from, to);

        NONTERMINAL: GrammarItem =
            (<n:ID>, <t:[NONTERMINAL_TYPE]>, "=", <a:ALTERNATIVES>) => {
                GrammarItem::Nonterminal(NonterminalData { name: n,
                                                           type_decl: t,
                                                           alternatives: a })
            };

        NONTERMINAL_TYPE: String =
            (":", <s:NOT_EQ>) => s.to_string();

        // FIXME this isn't really right; we should be gobbling up token
        // trees here until we find an "="
        NOT_EQ: &'input str =
            regex("[^=]+");

        ALTERNATIVES: Vec<Alternative> =
            (ALTERNATIVES1 / ALTERNATIVESN);

        ALTERNATIVES1: Vec<Alternative> =
            (<a:ALTERNATIVE>) => vec![a];

        ALTERNATIVESN: Vec<Alternative> =
            ("{", <a:{ALTERNATIVE}>, "}", ";") => a;

        ALTERNATIVE: Alternative =
            (<s:{SYMBOL_ARG}>, <a:[ACTION]>, ";") => Alternative {
                expr: SymbolExpr { args: s },
                action: a
            };

        ACTION: String =
            ("=>", <c:CODE_BLOCK>) => c;

        // Symbol items

        SYMBOL_ARG: SymbolArg =
            (NAMED_SYMBOL / ANON_SYMBOL / NONE_SYMBOL);

        NAMED_SYMBOL: SymbolArg =
            ("<", <id:ID>, ":", <sym:SYMBOL>, ">") => {
                SymbolArg { symbol: sym, highlight: SymbolHighlight::Named(id) }
            };

        ANON_SYMBOL: SymbolArg =
            ("<", <sym:SYMBOL>, ">") => {
                SymbolArg { symbol: sym, highlight: SymbolHighlight::Anon }
            };

        NONE_SYMBOL: SymbolArg =
            (<sym:SYMBOL>) => {
                SymbolArg { symbol: sym, highlight: SymbolHighlight::None }
            };

        SYMBOL_ARG0: SymbolArg =
            (NAMED_SYMBOL / ANON_SYMBOL / NONE_SYMBOL0);

        NONE_SYMBOL0: SymbolArg =
            (<sym:SYMBOL0>) => {
                SymbolArg { symbol: sym, highlight: SymbolHighlight::None }
            };

        // Symbols

        SYMBOL: Symbol =
            (PLUS_SYMBOL0 / PLUS_SYMBOLN /
             QUESTION_SYMBOL0 / QUESTION_SYMBOLN /
             STAR_SYMBOL0 / STAR_SYMBOLN /
             SYMBOL0);

        SYMBOL0: Symbol =
            (TERMINAL_SYMBOL / NT_SYMBOL);

        TERMINAL_SYMBOL: Symbol =
            (<l:LITERAL>) => Symbol::Terminal(l);

        NT_SYMBOL: Symbol =
            (<l:ID>) => Symbol::Nonterminal(l);

        PLUS_SYMBOL0: Symbol =
            (<v:SYMBOL_ARG0>, "+") => Symbol::Plus(SymbolExpr { args: vec![v] });

        PLUS_SYMBOLN: Symbol =
            (<v:SYMBOL_EXPRN>, "+") => Symbol::Plus(v);

        QUESTION_SYMBOL0: Symbol =
            (<v:SYMBOL_ARG0>, "?") => Symbol::Question(SymbolExpr { args: vec![v] });

        QUESTION_SYMBOLN: Symbol =
            (<v:SYMBOL_EXPRN>, "?") => Symbol::Question(v);

        STAR_SYMBOL0: Symbol =
            (<v:SYMBOL_ARG0>, "*") => Symbol::Star(SymbolExpr { args: vec![v] });

        STAR_SYMBOLN: Symbol =
            (<v:SYMBOL_EXPRN>, "*") => Symbol::Question(v);

        SYMBOL_EXPRN: SymbolExpr =
            ("(", <s:{SYMBOL_ARG}>, ")") => SymbolExpr { args: s };

        // TypeName

        TYPE_NAME: TypeName =
            (<prefix:{PATH_COMPONENT}>, <name:ID>, <suffix:PATH_SUFFIX>) => {
                TypeName::new(prefix, name, suffix)
            };

        PATH_COMPONENT: InternedString =
            (<i:ID>, "::") => i;

        PATH_SUFFIX: Vec<InternedString> =
            (<p:[PATH_SUFFIX_1]>) => p.unwrap_or(Vec::new());

        PATH_SUFFIX_1: Vec<InternedString> =
            ("<", <p:PATH_PARAMETERS>, ">") => p;

        PATH_PARAMETERS: Vec<InternedString> =
            fold(<p:PATH_PARAMETER0>,
                 (",", <q:PATH_PARAMETER>) => { let mut p = p; p.push(q); p });

        PATH_PARAMETER0: Vec<InternedString> =
            (<p:PATH_PARAMETER>) => vec![p];

        PATH_PARAMETER: InternedString =
            (PATH_PARAMETER_TYPE / PATH_PARAMETER_LIFETIME);

        PATH_PARAMETER_TYPE: InternedString =
            ID;

        PATH_PARAMETER_LIFETIME: InternedString =
            LIFETIME;

        // IDENTIFIERS, LIFETIMES

        ID: InternedString =
            (<i:ID_RE>) => intern(i);

        ID_RE: &'input str =
            regex(r"[a-zA-Z_][a-zA-Z0-9_]*");

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

struct CODE_BLOCK;

impl<'input> rusty_peg::Symbol<'input,Parser<'input>> for CODE_BLOCK {
    type Output = String;

    fn pretty_print(&self) -> String {
        format!("ACTION")
    }

    fn parse(&self, _: &mut Parser<'input>, input: rusty_peg::Input<'input>)
             -> rusty_peg::ParseResult<'input,String>
    {
        let bytes = input.text.as_bytes();
        let mut offset = input.offset;

        if offset >= input.text.len() || bytes[offset] != ('{' as u8) {
            return Err(rusty_peg::Error { expected: "'{' character",
                                          offset: input.offset });
        }

        let mut balance = 1;
        while balance != 0 {
            offset += 1;

            if offset >= input.text.len() {
                return Err(rusty_peg::Error { expected: "matching '}' character",
                                              offset: offset });
            }

            if bytes[offset] == ('{' as u8) {
                balance += 1;
            } else if bytes[offset] == ('}' as u8) {
                balance -= 1;
            }
        }

        offset += 1; // consume final `}`

        let regex_str = &input.text[input.offset + 1 .. offset - 1];
        let output = rusty_peg::Input { text: input.text, offset: offset };
        return Ok((output, regex_str.to_string()));
    }
}

pub fn parse_type_name(text: &str) -> TypeName {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&TYPE_NAME, &mut parser, text).unwrap()
}

pub fn parse_grammar(text: &str) -> Result<Grammar,rusty_peg::Error> {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&GRAMMAR, &mut parser, text)
}
