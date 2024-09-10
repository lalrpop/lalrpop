use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Str(String),
    Num(f64),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}
