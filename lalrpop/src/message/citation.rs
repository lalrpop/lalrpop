use ascii_canvas::AsciiView;
use file_text::FileText;
use grammar::parse_tree::Span;
use std::fmt::{Debug, Formatter, Error};
use std::rc::Rc;
use style::Style;

use super::*;

pub struct Citation {
    span: Span,
    file_text: Rc<FileText>
}

impl Content for Citation {
    fn min_width(&self) -> usize {
        let text = self.file_text.span_str(self.span);
        text.chars().count()
     }

    fn emit(&self, view: &mut AsciiView) {
        let text = self.file_text.span_str(self.span);
        view.write_chars(0, 0, text.chars(), Style::new())
    }

    fn into_wrap_items(self: Box<Self>, wrap_items: &mut Vec<Box<Content>>) {
        wrap_items.push(self);
    }
}

impl Debug for Citation {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.debug_struct("Citation")
           .field("span", &self.span)
           .finish()
    }
}
