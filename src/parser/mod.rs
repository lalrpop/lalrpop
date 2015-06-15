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
            ("grammar" <t:TYPE_NAME> "{" <i:{GRAMMAR_ITEM}> "}") => {
                Grammar { type_name: t, items: i }
            };

        GRAMMAR_ITEM: GrammarItem =
            (TOKEN_TYPE / NONTERMINAL);

        TOKEN_TYPE: GrammarItem =
            ("token" <t:TYPE_NAME> "where" "{" <c:{CONVERSION}> "}" ";") => {
                GrammarItem::TokenType(TokenTypeData {type_name: t, conversions: c })
            };

        CONVERSION: (InternedString, InternedString) =
            (<from:LITERAL> "=>" <to:LITERAL> ";") => (from, to);

        NONTERMINAL: GrammarItem =
            (<n:ID> <t:[NONTERMINAL_TYPE]> "=" <a:ALTERNATIVES>) => {
                GrammarItem::Nonterminal(NonterminalData { name: n,
                                                           type_decl: t,
                                                           alternatives: a })
            };

        NONTERMINAL_TYPE: String =
            (":" <s:NOT_EQ>) => s.to_string();

        // FIXME this isn't really right; we should be gobbling up token
        // trees here until we find an "="
        NOT_EQ: &'input str =
            regex("[^=]+");

        ALTERNATIVES: Vec<Alternative> =
            (ALTERNATIVES1 / ALTERNATIVESN);

        ALTERNATIVES1: Vec<Alternative> =
            (<a:ALTERNATIVE>) => vec![a];

        ALTERNATIVESN: Vec<Alternative> =
            ("{" <a:{ALTERNATIVE}> "}" ";") => a;

        ALTERNATIVE: Alternative =
            (<s:{SYMBOL}> <a:[ACTION]> ";") => Alternative {
                expr: s,
                action: a
            };

        ACTION: String = ("=>" <b:CODE>) => b;

        // Symbols

        SYMBOL: Symbol =
            fold(<lhs:SYMBOL0>,
                 "+" => Symbol::Plus(Box::new(lhs)),
                 "*" => Symbol::Star(Box::new(lhs)),
                 "?" => Symbol::Question(Box::new(lhs)));

        SYMBOL0: Symbol =
            (TERMINAL_SYMBOL / NT_SYMBOL / EXPR_SYMBOL / NAMED_SYMBOL / CHOSEN_SYMBOL);

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

        // TypeName

        TYPE_NAME: TypeName =
            (<prefix:{PATH_COMPONENT}> <name:ID> <suffix:PATH_SUFFIX>) => {
                TypeName::new(prefix, name, suffix)
            };

        PATH_COMPONENT: InternedString =
            (<i:ID> "::") => i;

        PATH_SUFFIX: Vec<InternedString> =
            (<p:[PATH_SUFFIX_1]>) => p.unwrap_or(Vec::new());

        PATH_SUFFIX_1: Vec<InternedString> =
            ("<" <p:PATH_PARAMETERS> ">") => p;

        PATH_PARAMETERS: Vec<InternedString> =
            fold(<p:PATH_PARAMETER0>,
                 ("," <q:PATH_PARAMETER>) => { let mut p = p; p.push(q); p });

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

pub fn parse_type_name(text: &str) -> TypeName {
    let mut parser = Parser::new(());
    rusty_peg::Symbol::parse_complete(&TYPE_NAME, &mut parser, text).unwrap()
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
