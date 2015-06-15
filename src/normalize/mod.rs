/*!
 * Normalization processes a parse tree until it is in suitable form to
 * be converted to the more canonical form. This is done as a series of
 * passes, each contained in their own module below.
 */

use grammar::parse_tree as pt;

pub fn normalize(input: &pt::Grammar) -> Result<pt::Grammar> {
}

// These are executed *IN ORDER*:

// Expands macros
//
//     X = ...1 Vec<X> ...2
//
// to
//
//     X = ...1 Vec_X ...2
//     Vec_X = ...;
mod macro_expand;

// Converts
//
//     X = ...1 (A B C) ...2
//
// to
//
//     X = ...1 A_B_C ...2
//     A_B_C = A B C
mod nonterminalize;

// Converts
//
//     X = ...1 Y* ...2
//
// to
//
//     X = ...1    ...2
//       | ...1 Y+ ...2
mod remove_star;

// Converts X+ to a new terminal X_PLUS like:
//
//     X_PLUS = {
//         <e:X> => { vec![x] }
//         <v:X_PLUS> <e:X> => { let mut v = v; v.push(e); v }
//     }
mod remove_plus;

// Converts
//
//     X = ...1 Y? ...2
//
// to
//
//     X = ...1    ...2
//       | ...1 Y  ...2
mod remove_question;

// Infers types for all nonterminals where possible, or reports a
// suitable error.
mod actionify;

