use intern::{intern, InternedString};
use grammar::*;
use rusty_peg::Symbol;

#[cfg(test)]
mod test;

rusty_peg! {
    parser Parser<'input> {
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
    }
}

pub fn parse_type_name(text: &str) -> TypeName {
    let mut parser = Parser::new(());
    TYPE_NAME.parse_complete(&mut parser, text).unwrap()
}
