/// Recognized associated type for the token location
pub const LOCATION: &str = "Location";

/// Recognized associated type for custom errors
pub const ERROR: &str = "Error";

/// The lifetime parameter injected when we do not have an external token enum
pub const INPUT_LIFETIME: &str = "'input";

/// The parameter injected when we do not have an external token enum
pub const INPUT_PARAMETER: &str = "input";

/// The attribute to request inlining.
pub const INLINE: &str = "inline";

/// The attribute to request conditional compilation.
pub const CFG: &str = "cfg";

/// The attribute to request recursive-ascent-style code generation.
pub const TABLE_DRIVEN: &str = "table_driven";

/// The attribute to request recursive-ascent-style code generation.
pub const RECURSIVE_ASCENT: &str = "recursive_ascent";

/// The attribute to request test-all-style code generation.
pub const TEST_ALL: &str = "test_all";
