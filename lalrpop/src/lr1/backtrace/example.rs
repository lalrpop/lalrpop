//! Code to compute example inputs given a backtrace.

use lr1::LR0Item;

use super::{BacktraceNode, Example, Reduction};

pub struct ExampleIterator<'ex> {
    stack: Vec<ExampleState<'ex>>,
}

#[derive(Debug)]
struct ExampleState<'ex> {
    // Node we are exploring
    node: &'ex BacktraceNode<'ex>,

    // Index of next parent to explore
    index: usize,
}

impl<'ex> ExampleIterator<'ex> {
    pub fn new(backtrace: &'ex BacktraceNode<'ex>) -> Self {
        let mut this = ExampleIterator { stack: vec![] };
        this.stack.push(ExampleState { node: backtrace, index: 0 });
        this.populate();
        this
    }

    fn populate(&mut self) -> bool {
        let parent = {
            // Obtain parent from top of stack, if any, and increment
            // index for top of stack.
            let top = self.stack.last_mut().expect("populate called but stack is empty");
            let index = top.index;
            if index == top.node.parents.len() {
                return false; // top of stack has no parent
            }
            top.index += 1;
            &top.node.parents[index]
        };
        self.stack.push(ExampleState { node: parent, index: 0 });
        self.populate();
        return true; // top of stack had a parent (now pushed)
    }

    fn iterate(&mut self) {
        // When this function is called, the top of the stack should
        // always be some leaf node in the tree.
        let top = self.stack.pop().unwrap();
        assert!(top.node.parents.len() == 0 && top.index == 0);

        while !self.stack.is_empty() {
            if self.populate() {
                return;
            }

            self.stack.pop();
        }
    }

    fn unwind<I: Iterator<Item=&'ex LR0Item<'ex>>>(&self,
                                                   mut rev_items: I,
                                                   example: &mut Example) {
        let item = if let Some(item) = rev_items.next() {
            item
        } else {
            return;
        };

        let start = example.symbols.len();

        let prefix = &item.production.symbols[..item.index];
        example.symbols.extend(prefix);

        self.unwind(rev_items, example);

        if item.index != item.production.symbols.len() {
            let suffix = &item.production.symbols[item.index+1..];
            example.symbols.extend(suffix);
        }

        let end = example.symbols.len();
        example.reductions.push(Reduction {
            start: start,
            end: end,
            nonterminal: item.production.nonterminal
        });
    }
}

impl<'ex> Iterator for ExampleIterator<'ex> {
    type Item = Example;

    fn next(&mut self) -> Option<Example> {
        if self.stack.is_empty() {
            return None;
        }

        let mut example = Example {
            symbols: vec![],
            reductions: vec![],
        };

        {
            let rev_items = self.stack.iter().rev().map(|s| &s.node.item);
            self.unwind(rev_items, &mut example);
        }

        self.iterate();

        Some(example)
    }
}

