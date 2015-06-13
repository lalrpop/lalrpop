use std::collections::{HashMap};

use intern::InternedString;
use grammar::TypeName;

#[cfg(test)]
mod test;

pub struct TokenDefinition {
    // if the enum type is `foo::bar::baz<X,Y>` then:
    enum_type: TypeName,

    // map from a custom string, like `"("` to a variant name like LPAREN
    token_map: HashMap<InternedString, InternedString>,
}

impl TokenDefinition {
    pub fn new(enum_type: TypeName,
               token_map: Vec<(InternedString, InternedString)>)
               -> TokenDefinition
    {
        TokenDefinition {
            enum_type: enum_type,
            token_map: token_map.into_iter().collect(),
        }
    }

    pub fn enum_type(&self) -> &TypeName {
        &self.enum_type
    }

    pub fn match_pattern(&self, name: InternedString) -> String {
        let variant_name = match self.token_map.get(&name) {
            Some(&v) => v,
            None => name,
        };

        format!("{}::{}(..)", self.enum_type.path(), variant_name)
    }
}
