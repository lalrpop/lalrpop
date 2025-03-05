#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod api;

pub use crate::api::process_root;
#[allow(deprecated)]
pub use crate::api::process_root_unconditionally;
pub use crate::api::process_src;
pub use crate::api::Configuration;
