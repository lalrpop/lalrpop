use std::fs;
use criterion::{criterion_group, Criterion};

use crate::src::parsers::json;

pub fn json_parse(c: &mut Criterion) {
    let parser = json::ValueParser::new();
    let json_file = fs::read_to_string("benches/data/512KB.json").unwrap();
    c.bench_function("Parse JSON", |b| {
        b.iter(|| {
            parser.parse(&json_file).unwrap();
        })
    });
}

criterion_group!(parser, json_parse);

