//! Code to compute example inputs given a backtrace.

use ansi_term::Style;
use grammar::repr::{NonterminalString, Symbol};
use message::ascii_canvas::{AsciiCanvas, AsciiView, Row};
use session::Session;

#[cfg(test)] mod test;

/// An "example" input and the way it was derived. This can be
/// serialized into useful text. For example, it might represent
/// something like this:
///
/// ```
///          Looking at
///              |
///              v
/// Ty "->" Ty "->" Ty
/// |        |       |
/// +-Ty-----+       |
/// |                |
/// +-Ty-------------+
/// ```
///
/// The top-line is the `symbols` vector. The groupings below are
/// stored in the `reductions` vector, in order from smallest to
/// largest (they are always properly nested). The `cursor` field
/// indicates the current lookahead token.
///
/// The `symbols` vector is actually `Option<Symbol>` to account
/// for empty reductions:
///
/// ```
/// A       B
/// | |   | |
/// | +-Y-+ |
/// +-Z-----+
/// ```
///
/// The "empty space" between A and B would be represented as `None`.
#[derive(Clone, Debug)]
pub struct Example {
    pub symbols: Vec<ExampleSymbol>,
    pub cursor: usize,
    pub reductions: Vec<Reduction>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ExampleSymbol {
    Symbol(Symbol),
    Epsilon,
}

#[derive(Default)]
pub struct ExampleStyles {
    pub before_cursor: Style,
    pub on_cursor: Style,
    pub after_cursor: Style,
}

#[derive(Copy, Clone, Debug)]
pub struct Reduction {
    pub start: usize,
    pub end: usize,
    pub nonterminal: NonterminalString,
}

impl Example {
    /// Length of each symbol. Each will need *at least* that amount
    /// of space. :) Measure in characters, under the assumption of a
    /// mono-spaced font. Also add a final `0` marker which will serve
    /// as the end position.
    fn lengths(&self) -> Vec<usize> {
        self.symbols.iter()
                    .map(|s| match *s {
                        ExampleSymbol::Symbol(s) => format!("{}", s).chars().count(),
                        ExampleSymbol::Epsilon => 1, // display as " "
                    })
                    .chain(Some(0))
                    .collect()
    }

    fn starting_positions(&self, lengths: &[usize]) -> Vec<usize> {
        lengths.iter()
               .scan(0, |counter, &len| {
                   let start = *counter;

                   // Leave space for "NT " (if "NT" is the name
                   // of the nonterminal).
                   *counter = start + len + 1;

                   Some(start)
               })
               .collect()
    }

    /// Start index where each symbol in the example should appear,
    /// measured in characters. These are spaced to leave enough room
    /// for the reductions below.
    fn positions(&self, lengths: &[usize]) -> Vec<usize> {
        // Initially, position each symbol with one space in between,
        // like:
        //
        //     X Y Z
        let mut positions = self.starting_positions(lengths);

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
            let nt_len = format!("{}", nonterminal).chars().count();

            // Number of symbols we are reducing. This should always
            // be non-zero because even in the case of a \epsilon
            // rule, we ought to be have a `None` entry in the symbol array.
            let num_syms = end - start;
            assert!(num_syms > 0);

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
            // A1 B2 C3 D4 E5
            //             ^ positions[end]
            //            ^ here -- positions[end] - 1
            let end_position = positions[end] - 1;

            // We need space to draw `+-Label-+` between
            // start_position and end_position.
            let required_len = nt_len + 4; // here, 15
            let actual_len = end_position - start_position; // here, 10
            if required_len < actual_len {
                continue; // Got enough space, all set.
            }

            // Have to add `difference` characters altogether.
            let difference = required_len - actual_len; // here, 4

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

                // For the first `extra` symbols, give them amount + 1
                // extra space. After that, just amount. (O(n^2). Sue me.)
                for i in 0 .. extra {
                    shift(&mut positions[start + 1 + i .. end], amount + 1);
                }
                for i in extra .. num_gaps {
                    shift(&mut positions[start + 1 + i .. end], amount);
                }
            }
        }

        positions
    }

    /// Paints a prefix of the symbols from this example using the
    /// given styles. `number` is the number of symbols to print;
    /// typically it would be either `symbols.len()` (all symbols) or
    /// `cursor` (up until the current token).
    pub fn paint_symbols(&self,
                         number: usize,
                         styles: &ExampleStyles)
                         -> Row {
        let lengths = self.lengths();
        let positions = self.starting_positions(&lengths);
        let columns = *positions.last().unwrap();
        let mut canvas = AsciiCanvas::new(1, columns);
        self.paint_symbols_on(&self.symbols[..number], &positions,
                              styles, &mut canvas);
        canvas.to_strings().pop().unwrap()
    }

    #[cfg(test)]
    pub fn paint_unstyled(&self) -> Vec<Row> {
        use std::default::Default;
        self.paint(&ExampleStyles::default())
    }

    pub fn paint(&self, styles: &ExampleStyles) -> Vec<Row> {
        let lengths = self.lengths();
        let positions = self.positions(&lengths);
        let rows = 1 + self.reductions.len() * 2;
        let columns = *positions.last().unwrap();
        let mut canvas = AsciiCanvas::new(rows, columns);
        self.paint_on(styles, &positions, &mut canvas);
        canvas.to_strings()
    }

    fn paint_on(&self,
                styles: &ExampleStyles,
                positions: &[usize],
                view: &mut AsciiView) {
        // Write the labels:
        //    A1   B2  C3  D4 E5 F6
        self.paint_symbols_on(&self.symbols, &positions, styles, view);

        // Draw the brackets for each reduction:
        for (index, reduction) in self.reductions.iter().enumerate() {
            let start_column = positions[reduction.start];
            let end_column = positions[reduction.end] - 1;
            let row = 2 + index * 2;
            view.draw_vertical_line(1 .. row + 1, start_column);
            view.draw_vertical_line(1 .. row + 1, end_column - 1);
            view.draw_horizontal_line(row, start_column .. end_column);
        }

        // Write the labels for each reduction. Do this after the
        // brackets so that ascii canvas can convert `|` to `+`
        // without interfering with the text (in case of weird overlap).
        for (index, reduction) in self.reductions.iter().enumerate() {
            let column = positions[reduction.start] + 2;
            let row = 2 + index * 2;
            view.write_chars(row,
                             column,
                             format!("{}", reduction.nonterminal).chars(),
                             Style::new());
        }
    }

    fn paint_symbols_on(&self,
                        symbols: &[ExampleSymbol],
                        positions: &[usize],
                        styles: &ExampleStyles,
                        view: &mut AsciiView) {
        for (index, ex_symbol) in symbols.iter().enumerate() {
            let style = if index < self.cursor {
                styles.before_cursor
            } else if index == self.cursor {
                // Only display actual terminals in the "on-cursor"
                // font, because it might be misleading to show a
                // nonterminal that way. Really it'd be nice to expand
                // so that the cursor is always a terminal.
                match *ex_symbol {
                    ExampleSymbol::Symbol(Symbol::Terminal(_)) => styles.on_cursor,
                    _ => styles.after_cursor,
                }
            } else {
                styles.after_cursor
            };

            match *ex_symbol {
                ExampleSymbol::Symbol(symbol) => {
                    let column = positions[index];
                    view.write_chars(0, column, format!("{}", symbol).chars(), style);
                }
                ExampleSymbol::Epsilon => {
                }
            }
        }
    }
}

fn shift(positions: &mut [usize], amount: usize) {
    for position in positions {
        *position += amount;
    }
}

impl ExampleStyles {
    pub fn ambig(session: &Session) -> Self {
        ExampleStyles {
            before_cursor: session.ambig_symbols,
            on_cursor: session.ambig_symbols,
            after_cursor: session.ambig_symbols,
        }
    }

    pub fn new(session: &Session) -> Self {
        ExampleStyles {
            before_cursor: session.observed_symbols,
            on_cursor: session.cursor_symbol,
            after_cursor: session.unobserved_symbols,
        }
    }
}
