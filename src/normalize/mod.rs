/*!
 * Normalization processes a parse tree until it is in suitable form to
 * be converted to the more canonical form. This is done as a series of
 * passes, each contained in their own module below.
 */

use grammar::parse_tree as pt;

pub type NormResult<T> = Result<T, NormError>;

#[derive(Clone, Debug)]
pub struct NormError {
    message: String,
    span: pt::Span,
}

macro_rules! return_err {
    ($span: expr, $($args:expr),+) => {
        return Err(NormError {
            message: format!($($args),+),
            span: $span
        });
    }
}

// These are executed *IN ORDER*:

// Expands macros and expressions
//
//     X = ...1 Comma<X> (X Y Z) ...2
//
// to
//
//     X = ...1 `Comma<X>` `(X Y Z)` ...2
//     `Comma_X`: Vec<<X>> = ...;
//     `(X Y Z)` = X Y Z;
//
// AFTER THIS POINT: No more macros, macro references, guarded
// alternatives, repeats, or expr symbols, though type indirections
// may occur.
mod macro_expand;

// Computes types where the user omitted them (or from macro
// byproducts).
//
// AFTER THIS POINT: there is a separate `repr::Types` table
// providing all nonterminals with an explicit type.
mod tyinfer;

// Synthesizes action code for all nonterminals.
//
// AFTER THIS POINT: All nonterminals have action code, and all
// Symbol::Choose and Symbol::Name are removed.
// mod action;

///////////////////////////////////////////////////////////////////////////
// Shared routines

#[cfg(test)]
mod test_util;

mod norm_util;

