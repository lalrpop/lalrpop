use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'a> {
    Null,
    Boolean(bool),
    Str(&'a str),
    Num(f64),
    Array(Vec<Value<'a>>),
    Object(HashMap<&'a str, Value<'a>>),
}
