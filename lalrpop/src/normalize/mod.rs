/*!
 * Normalization processes a parse tree until it is in suitable form to
 * be converted to the more canonical form. This is done as a series of
 * passes, each contained in their own module below.
 */

use grammar::parse_tree as pt;
use grammar::repr as r;

pub type NormResult<T> = Result<T, NormError>;

#[derive(Clone, Debug)]
pub struct NormError {
    pub message: String,
    pub span: pt::Span,
}

macro_rules! return_err {
    ($span: expr, $($args:expr),+) => {
        return Err(NormError {
            message: format!($($args),+),
            span: $span
        });
    }
}

pub fn normalize(grammar: pt::Grammar) -> NormResult<r::Grammar> {
    let grammar = try!(macro_expand::expand_macros(grammar));
    let types = try!(tyinfer::infer_types(&grammar));
    lower::lower(grammar, types)
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

// Lowers the parse tree to the repr notation.
mod lower;

///////////////////////////////////////////////////////////////////////////
// Shared routines

mod norm_util;

