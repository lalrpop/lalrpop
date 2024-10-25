use criterion::criterion_main;

mod src {
    pub mod compile_benches;
    pub mod parser_benches;
    // This can just use lalrpop_mod!() once #979 merges
    #[rustfmt::skip]
    pub mod parsers;
}

use crate::src::compile_benches::compile;
use crate::src::parser_benches::parser;

criterion_main!(compile, parser);
