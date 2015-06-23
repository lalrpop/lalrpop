use std::collections::VecDeque;
use std::hash::Hash;
use util::{map, Map};

pub struct KernelSet<I: StateIndex> {
    kernels: VecDeque<I::Kernel>,
    map: Map<I::Kernel, I>,
}

pub trait StateIndex: Copy {
    type Kernel: Clone + Hash + Eq;

    fn from(c: usize) -> Self;
}

impl<I: StateIndex> KernelSet<I> {
    pub fn new() -> KernelSet<I> {
        KernelSet { kernels: VecDeque::new(), map: map() }
    }

    pub fn add_state(&mut self, kernel: I::Kernel) -> I {
        let kernels = &mut self.kernels;
        *self.map.entry(kernel.clone()).or_insert_with(|| {
            let index = kernels.len();
            kernels.push_back(kernel);
            StateIndex::from(index)
        })
    }

    pub fn next(&mut self) -> Option<I::Kernel> {
        self.kernels.pop_front()
    }
}
