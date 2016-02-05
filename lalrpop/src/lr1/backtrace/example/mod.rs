//! Code to compute example inputs given a backtrace.

use lr1::LR0Item;

use super::{BacktraceNode, Example, Reduction};

mod ascii_canvas;
#[cfg(test)] mod test;

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

        // Push items before the cursor in the current item.
        // e.g, in `Foo = W X (*) Y Z`, push "W X".
        let prefix = &item.production.symbols[..item.index];
        example.symbols.extend(prefix.iter().map(|&s| Some(s)));

        // Recurse to expand the item *at* the cursor (if any).  e.g.,
        // in `Foo = W X (*) Y Z`, this would expand `Y` with its
        // derivation.
        self.unwind(rev_items, example);

        // Push items after the cursor in the current item.
        // e.g., in `Foo = W X (*) Y Z`, push "Z".
        if item.index != item.production.symbols.len() {
            let suffix = &item.production.symbols[item.index+1..];
            example.symbols.extend(suffix.iter().map(|&s| Some(s)));
        }

        // If it turns out that we did not push anything, then push
        // `None` to represent the "empty sequence" that is being
        // reduced here (e.g., if the item is `Foo = (*)`).
        if start == example.symbols.len() {
            example.symbols.push(None);
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

impl Example {
    /// Length of each symbol. Each will need *at least* that amount
    /// of space. :) Measure in characters, under the assumption of a
    /// mono-spaced font. Also add a final `0` marker which will serve
    /// as the end position.
    fn lengths(&self) -> Vec<usize> {
        self.symbols.iter()
                    .map(|s| match *s {
                        Some(s) => format!("{}", s).chars().count(),
                        None => 1, // display \epsilon symbols as " ", effectively.
                    })
                    .chain(Some(0))
                    .collect()
    }

    /// Start index where each symbol in the example should appear,
    /// measured in characters. These are spaced to leave enough room
    /// for the reductions below.
    fn positions(&self, lengths: &[usize]) -> Vec<usize> {
        println!("positions({:?}, lengths={:?})", self.symbols, lengths);

        // Initially, position each symbol with one space in between,
        // like:
        //
        //     X Y Z
        let mut positions: Vec<_> =
            lengths.iter()
                   .scan(0, |counter, &len| {
                       let start = *counter;

                       // Leave space for "NT " (if "NT" is the name
                       // of the nonterminal).
                       *counter = start + len + 1;

                       Some(start)
                   })
                   .collect();

        // The final position is a marker. Currently we will leave
        // an extra space, so remove it:
        //
        //     X Y Z
        //           ^ where the marker would be otherwise
        *positions.last_mut().unwrap() -= 1;

        println!("positions: initial={:?}", positions);

        // Adjust spacing to account for the nonterminal labels
        // we will have to add. It will display
        // like this:
        //
        //    A1 B2 C3 D4 E5 F6
        //    |         |
        //    +-Label---+
        //
        // But if the label is long we may have to adjust the spacing
        // of the covered items (here, we changed them to two spaces,
        // except the first gap, which got 3 spaces):
        //
        //    A1   B2  C3  D4 E5 F6
        //    |             |
        //    +-LongLabel22-+
        for &Reduction { start, end, nonterminal } in &self.reductions {
            println!("positions: nonterminal={:?} start={:?} end={:?}",
                     nonterminal, start, end);

            let nt_len = format!("{}", nonterminal).chars().count();

            // Number of symbols we are reducing. This should always
            // be non-zero because even in the case of a \epsilon
            // rule, we ought to be have a `None` entry in the symbol array.
            let num_syms = end - start;
            assert!(num_syms > 0);
            println!("positions: num_syms={:?} nt_len={:?}", num_syms, nt_len);

            // Let's use the expansion from above as our running example.
            // We start out with positions like this:
            //
            //    A1 B2 C3 D4 E5 F6
            //    |             |
            //    +-LongLabel22-+
            //
            // But we want LongLabel to end at D4. No good.

            // Start of first symbol to be reduced. Here, 0.
            //
            // A1 B2 C3 D4
            // ^ here
            let start_position = positions[start];

            // End of last symbol to be reduced. Here, 11.
            //
            // A1 B2 C3 D4
            //            ^ here
            let end_position = positions[end - 1] + lengths[end - 1];

            println!("positions: start_position={:?} end_position={:?}",
                     start_position,
                     end_position);

            // We need space to draw `+-Label-+` between
            // start_position and end_position.
            let required_len = nt_len + 4; // here, 15
            let actual_len = end_position - start_position; // here, 10
            if required_len < actual_len {
                continue; // Got enough space, all set.
            }

            // Have to add `difference` characters altogether.
            let difference = required_len - actual_len; // here, 4

            println!("positions: nt_len={:?} required_len={:?} \
                      actual_len={:?} difference={:?}",
                     nt_len, required_len,
                     actual_len, difference);

            // Increment over everything that is not part of this nonterminal.
            // In the example above, that is E5 and F6.
            shift(&mut positions[end..], difference);

            if num_syms > 1 {
                // If there is just one symbol being reduced here,
                // then we have shifted over the things that follow
                // it, and we are done. This would be a case like:
                //
                //     X         Y Z
                //     |       |
                //     +-Label-+
                //
                // (which maybe ought to be rendered slightly
                // differently).
                //
                // But if there are multiple symbols, we're not quite
                // done, because there would be an unsightly gap:
                //
                //       (gaps)
                //      |  |  |
                //      v  v  v
                //    A1 B2 C3 D4     E5 F6
                //    |             |
                //    +-LongLabel22-+
                //
                // we'd like to make things line up, so we have to
                // distribute that extra space internally by
                // increasing the "gaps" (marked above) as evenly as
                // possible (basically, full justification).
                //
                // We do this by dividing up the spaces evenly and
                // then taking the remainder `N` and distributing 1
                // extra to the first N.
                let num_gaps = num_syms - 1; // number of gaps we can adjust. Here, 3.
                let amount = difference / num_gaps; // what to add to each gap. Here, 1.
                let extra = difference % num_gaps; // the remainder. Here, 1.

                println!("positions: num_gaps={:?} amount={:?} extra={:?}",
                         num_gaps, amount, extra);

                // For the first `extra` symbols, give them amount + 1
                // extra space. After that, just amount. (O(n^2). Sue me.)
                for i in 0 .. extra {
                    shift(&mut positions[start + 1 + i .. end], amount + 1);
                }
                for i in extra .. num_gaps {
                    shift(&mut positions[start + 1 + i .. end], amount);
                }
            }

            println!("positions: current = {:?}", positions);
        }

        positions
    }
}

fn shift(positions: &mut [usize], amount: usize) {
    for position in positions {
        *position += amount;
    }
}
