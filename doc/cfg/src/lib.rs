use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    // No parser should have been generated so nothing should be unused
    #[deny(dead_code)]
    cfg
);

pub fn use_cfg_created_parser() {
    cfg::AlwaysCreatedParser::new();
    #[cfg(feature = "test-not-set")]
    cfg::NotCreatedParser::new();
    #[cfg(feature = "test-set")]
    cfg::CreatedParser::new();
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
    #[cfg(all(feature = "test-set", feature = "test-not-set"))]
    cfg::NotCreatedWithAllParser::new();
    #[allow(clippy::non_minimal_cfg)]
    #[cfg(any(feature = "test-set", all(not(feature = "test-not-set"))))]
    cfg::CreatedAnyWithAllNotNotParser::new();
}
