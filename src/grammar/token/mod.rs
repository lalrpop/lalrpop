use std::collections::{HashMap};
use regex::Regex;

use grammar::TypeName;

#[cfg(test)]
mod test;

pub struct TokenDefinition {
    // if the enum type is `foo::bar::baz<X,Y>` then:
    enum_type: TypeName,

    // map from a custom string, like `"("` to a variant name like LPAREN
    token_map: HashMap<String, String>,

    // regex for a Rust identifier
    identifier: Regex
}

impl TokenDefinition {
    pub fn new(enum_type: TypeName,
               token_map: Vec<(String, String)>)
               -> TokenDefinition
    {
        TokenDefinition {
            enum_type: enum_type,
            token_map: token_map.into_iter().collect(),
            identifier: Regex::new("[a-zA-Z_][:word:]*").unwrap(),
        }
    }

    pub fn enum_type(&self) -> &TypeName {
        &self.enum_type
    }

    pub fn valid_token_name(&self, name: &str) -> bool {
        self.token_map.contains_key(name) || self.identifier.is_match(name)
    }

    pub fn match_pattern(&self, name: &str) -> String {
        assert!(self.valid_token_name(name));

        let variant_name = match self.token_map.get(name) {
            Some(v) => &v[..],
            None => name,
        };

        format!("{}::{}(..)", self.enum_type.path(), variant_name)
    }
}
