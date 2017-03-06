//! A key part of the lane-table algorithm is the idea of a CONTEXT
//! SET (my name, the paper has no name for this). Basically it
//! represents the LR1 context under which a given conflicting action
//! would take place.
//!
//! So, for example, imagine this grammar:
//!
//! ```notrust
//! A = B x
//!   | C y
//! B = z
//! C = z
//! ```
//!
//! This gives rise to states like:
//!
//! - `S0 = { * B x, * C y, B = * z, C = * z }`
//! - `S1 = { B = z *, C = z * }`
//!
//! This second state has two conflicting items. Let's call them
//! conflicts 0 and 1 respectively. The conflict set would then have
//! two entries (one for each conflict) and it would map each of them
//! to a TokenSet supplying context. So when we trace everything
//! out we might get a ConflictSet of:
//!
//! - `[ 0: x, 1: y ]`
//!
//! In general, you want to ensure that the token sets of all
//! conflicting items are pairwise-disjoint, or else if you get to a
//! state that has both of those items (which, by definition, does
//! arise) you won't know which to take. In this case, we're all set,
//! because item 0 occurs only with lookahead `x` and item 1 with
//! lookahead `y`.

mod test;

#[derive(Clone)]
pub struct ConflictSet {
    values: Vec<Option<TokenSet>>
}

pub struct OverlappingLookahead;

impl ConflictSet {
    pub fn new(num_conflicts: usize) {
        ConflictSet {
            values: (0..num_conflicts).map(|_| None).collect()
        }
    }

    /// Attempts to merge the values `conflict: set` into this
    /// conflict set. If this would result in an invalid conflict set
    /// (where two conflicts have overlapping lookahead), then returns
    /// `Err(OverlappingLookahead)` and has no effect.
    ///
    /// Assuming no errors, returns `Ok(true)` if this resulted in any
    /// modifications, and `Ok(false)` otherwise.
    pub fn insert(&mut self, conflict: usize, set: TokenSet) -> Result<bool, OverlappingLookahead> {
        for value in self.values.iter().filter_map(|v| v.as_ref()) {
            if value.is_intersecting(&set) {
                return Err(OverlappingLookahead);
            }
        }

        if self.values[conflict].is_none() {
            self.values[conflict] = set;
            Ok(true)
        } else {
            Ok(self.values[conflict].union_with(&set))
        }
    }
}
