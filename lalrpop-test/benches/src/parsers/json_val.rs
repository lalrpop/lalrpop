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

/// For benchmarking purposes, this is `Value` except it contains references to
/// the input string to avoid unnecessary allocations.
#[derive(Debug, PartialEq, Clone)]
pub enum ValueRef<'a> {
    Null,
    Boolean(bool),
    Str(&'a str),
    Num(f64),
    Array(Vec<ValueRef<'a>>),
    Object(HashMap<&'a str, ValueRef<'a>>),
}
