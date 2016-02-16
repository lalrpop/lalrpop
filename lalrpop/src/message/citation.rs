use ansi_term::Style;
use filetext::FileText;
use grammar::parse_tree::Span;
use std::rc::Rc;

use super::*;
use super::ascii_canvas::AsciiView;

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
