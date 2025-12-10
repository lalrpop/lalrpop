use criterion::{Criterion, criterion_group};
use std::env;

use lalrpop::*;

pub fn lalrpop_compile(c: &mut Criterion) {
    c.bench_function("Build lalrpop grammar", |b| {
        b.iter(|| {
            Configuration::new()
                .force_build(true)
                .log_quiet()
                .set_out_dir(env::var("OUT_DIR").unwrap())
                .process_file("../lalrpop/src/parser/lrgrammar.lalrpop")
                .unwrap()
        });
    });
}

criterion_group!(compile, lalrpop_compile);
