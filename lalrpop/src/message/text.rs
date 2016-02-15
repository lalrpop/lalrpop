use ansi_term::Style;

use super::ascii_canvas::AsciiView;
use super::*;

/// A string to be displayed in one unit with some particular style;
/// typically a single word.
pub struct Text {
    text: String,
    style: Style,
}

impl Content for Text {
    fn min_width(&self) -> usize {
        self.text.chars().count()
    }

    fn emit(&self, view: &mut AsciiView) {
        view.write_chars(0, 0, self.text.chars(), self.style)
    }
}
