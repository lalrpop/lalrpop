use std::collections::{HashMap, HashSet};
use regex::Regex;

#[cfg(test)]
mod test;

pub struct TokenDefinition {
    // if the enum type is `foo::bar::baz<X,Y>` then:
    enum_mod: String, // == "::foo::bar", note leading `::`
    enum_type_name: String, // == "baz"
    enum_parameters: Vec<String>, // == ["X", "Y"]

    // map from a custom string, like `"("` to a variant name like LPAREN
    token_map: HashMap<String, String>,

    // regex for a Rust identifier
    identifier: Regex
}

impl TokenDefinition {
    pub fn new(enum_mod: String,
               enum_type_name: String,
               enum_parameters: Vec<String>,
               token_map: Vec<(String, String)>)
               -> TokenDefinition
    {
        TokenDefinition {
            enum_mod: enum_mod,
            enum_type_name: enum_type_name,
            enum_parameters: enum_parameters,
            token_map: token_map.into_iter().collect(),
            identifier: Regex::new("[a-zA-Z_][:word:]*").unwrap(),
        }
    }

    pub fn enum_absolute_path(&self) -> String {
        format!("{}::{}<{}>",
                self.enum_mod,
                self.enum_type_name,
                self.enum_parameters.connect(", "))
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

        format!("{}::{}::{}(..)", self.enum_mod, self.enum_type_name, variant_name)
    }
}
