use criterion::{criterion_group, Criterion};
use std::fs;

use crate::src::parsers::json;
use crate::src::parsers::json_ref;

pub fn json_parse(c: &mut Criterion) {
    let parser = json::ValueParser::new();
    let json_file = fs::read_to_string("benches/data/512KB.json").unwrap();
    c.bench_function("Parse JSON", |b| {
        b.iter(|| {
            parser.parse(&json_file).unwrap();
        })
    });
}

pub fn json_ref_parse(c: &mut Criterion) {
    let parser = json_ref::ValueRefParser::new();
    let json_file = fs::read_to_string("benches/data/512KB.json").unwrap();
    c.bench_function("Parse JSON with Referenced Input", |b| {
        b.iter(|| {
            parser.parse(&json_file).unwrap();
        })
    });
}

criterion_group!(parser, json_parse, json_ref_parse);
