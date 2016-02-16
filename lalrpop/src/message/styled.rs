use ansi_term::Style;
use ascii_canvas::AsciiView;
use std::fmt::{Debug, Formatter, Error};
use super::*;

pub struct Styled {
    style_fn: StyleFn,
    content: Box<Content>
}

/// "Style fn", usually `Style::bold` etc
pub type StyleFn = fn(&Style) -> Style;

impl Styled {
    pub fn new(style_fn: StyleFn, content: Box<Content>) -> Self {
        Styled {
            style_fn: style_fn,
            content: content,
        }
    }
}

impl Content for Styled {
    fn min_width(&self) -> usize {
        self.content.min_width()
    }

    fn emit(&self, view: &mut AsciiView) {
        self.content.emit(&mut view.styled(&self.style_fn))
    }

    fn into_wrap_items(self: Box<Self>, wrap_items: &mut Vec<Box<Content>>) {
        let style_fn = self.style_fn;
        super::into_wrap_items_map(self.content,
                                   wrap_items,
                                   |item| Styled::new(style_fn, item))
    }
}

impl Debug for Styled {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.debug_struct("Styled")
           .field("content", &self.content)
           .finish()
    }
}
