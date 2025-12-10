use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub json, "benches/src/parsers/json.rs");
pub mod json_val;
lalrpop_mod!(pub json_ref, "benches/src/parsers/json_ref.rs");
