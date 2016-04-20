//! Naive LR(1) generation algorithm.

use grammar::repr::*;
use self::tls::Lr1Tls;

pub mod ascent;

mod build;
mod build_lalr;
mod core;
mod error;
mod example;
mod first;
mod lane_table;
mod lookahead;
mod state_graph;
mod tls;
mod trace;

#[cfg(test)] mod interpret;

use self::core::LR1State;

pub use self::core::{LR1Result, LR1TableConstructionError};
pub use self::error::report_error;

pub fn build_states<'grammar>(grammar: &'grammar Grammar,
                              start: NonterminalString)
                              -> LR1Result<'grammar> {
    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());

    match grammar.algorithm {
        Algorithm::LR1 => build::build_lr1_states(grammar, start),
        Algorithm::LALR1 => build_lalr::build_lalr_states(grammar, start),
    }
}

