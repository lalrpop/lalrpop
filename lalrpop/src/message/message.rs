use ansi_term::Style;
use ascii_canvas::AsciiView;
use filetext::FileText;
use grammar::parse_tree::Span;
use message::Content;
use std::cmp;
use std::fmt::{Debug, Formatter, Error};
use std::rc::Rc;

/// The top-level message display like this:
///
/// ```
/// <span>: <heading>
///
/// <body>
/// ```
///
/// This is equivalent to a
///
/// ```
/// Vert[separate=2] {
///     Horiz[separate=1] {
///         Horiz[separate=0] {
///             Citation { span },
///             Text { ":" },
///         },
///         <heading>,
///     },
///     <body>
/// }
/// ```
pub struct Message {
    span: Span,
    file_text: Rc<FileText>,
    heading: Box<Content>,
    body: Box<Content>,
}

impl Message {
    pub fn new(span: Span,
               file_text: Rc<FileText>,
               heading: Box<Content>,
               body: Box<Content>) -> Self {
        Message {
            span: span,
            file_text: file_text,
            heading: heading,
            body: body,
        }
    }
}

impl Content for Message {
    fn min_width(&self) -> usize {
        let span = self.file_text.span_str(self.span).chars().count();
        let heading = self.heading.min_width();
        let body = self.heading.min_width();
        cmp::max(span + heading + 2, body + 2)
    }

    fn emit(&self, view: &mut AsciiView) {
        let span = self.file_text.span_str(self.span);
        view.write_chars(0, 0, span.chars(), Style::new());
        let count = span.chars().count();
        view.write_chars(0, count, ":".chars(), Style::new());

        let (row, _) = self.heading.emit_at(view, 0, count + 2);
        self.body.emit_at(view, row + 2, 2);
    }

    fn into_wrap_items(self: Box<Self>, wrap_items: &mut Vec<Box<Content>>) {
        wrap_items.push(self);
    }
}

impl Debug for Message {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.debug_struct("Message")
           .field("span", &self.span)
           .field("heading", &self.heading)
           .field("body", &self.body)
           .finish()
    }
}
