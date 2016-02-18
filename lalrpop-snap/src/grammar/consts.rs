/// Recognized associated type for the token location
pub const LOCATION: &'static str = "Location";

/// Recognized associated type for custom errors
pub const ERROR: &'static str = "Error";

/// The lifetime parameter injected when we do not have an external token enum
pub const INPUT_LIFETIME: &'static str = "'input";

/// The parameter injected when we do not have an external token enum
pub const INPUT_PARAMETER: &'static str = "input";

/// The annotation to request inlining.
pub const INLINE: &'static str = "inline";

/// The annotation to set precedence.
pub const PRECEDENCE: &'static str = "precedence";
