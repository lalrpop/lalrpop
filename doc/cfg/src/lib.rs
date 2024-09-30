#![cfg(not(all(feature = "test-set", feature = "test-not-set")))]
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    // No parser should have been generated so nothing should be unused
    #[deny(dead_code)]
    cfg
);

/// ideally this test can be run twice
/// * expected OK:  `cargo doc --no-default-features --features test-set`
/// * expected OK:  `cargo doc --no-default-features --features test-not-set`
/// * expected ERR: `cargo doc --no-default-features --features test-set,test-not-set`
pub fn use_cfg_created_parser() {
    cfg::AlwaysCreatedParser::new();
    #[cfg(feature = "test-set")]
    cfg::CreatedParser::new();
    #[cfg(feature = "test-not-set")]
    cfg::NotCreatedParser::new();
    #[cfg(not(feature = "test-not-set"))]
    cfg::CreatedWithNotParser::new();
    #[cfg(not(feature = "test-set"))]
    cfg::NotCreatedWithNotParser::new();
    #[cfg(any(feature = "test-not-set", feature = "test-set"))]
    cfg::CreatedWithAnyParser::new();
    #[allow(clippy::non_minimal_cfg)]
    #[cfg(any(feature = "test-not-set"))]
    cfg::NotCreatedWithAnyParser::new();
    #[allow(clippy::non_minimal_cfg)]
    #[cfg(all(feature = "test-set"))]
    cfg::CreatedWithAllParser::new();
    #[cfg(all(feature = "test-set", feature = "test-set2"))]
    cfg::CreatedWithAll2Parser::new();
    #[allow(clippy::non_minimal_cfg)]
    #[cfg(any(
        feature = "test-set",
        all(not(feature = "test-not-set"), feature = "test-set2")
    ))]
    cfg::CreatedWithAnyAllNotParser::new();
    #[cfg(feature = "test-set")]
    cfg::CreatedWithMacroParser::new();
    #[cfg(feature = "test-not-set")]
    cfg::NotCreatedWithMacroParser::new();
    #[cfg(feature = "test-set")]
    cfg::CreatedWithInlineParser::new();
    #[cfg(feature = "test-not-set")]
    cfg::NotCreatedWithInlineParser::new();

    cfg::CreateAlternativesParser::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_cfg_created_parser() {
        use_cfg_created_parser();
    }
}
