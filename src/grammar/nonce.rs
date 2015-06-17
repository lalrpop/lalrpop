/*!

A NONCE is just a guaranteed unique identifier. We use it to create
persistent identity for alternatives as we transform the grammar.

*/

use std::cell::Cell;

thread_local! {
    static NONCE: Cell<u32> = Cell::new(0)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Nonce {
    counter: u32
}

impl Nonce {
    pub fn new() -> Nonce {
        NONCE.with(|counter| {
            let c = counter.get();
            counter.set(c.checked_add(1).unwrap());
            Nonce { counter: c }
        })
    }
}
