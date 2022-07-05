mod map;
mod multimap;
mod set;

pub use self::{
    map::{map, Entry, Map},
    multimap::{Collection, Multimap},
    set::{set, Set},
};
