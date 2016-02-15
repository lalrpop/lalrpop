use grammar::parse_tree::Span;

pub mod ascii_canvas;
pub mod citation;
pub mod indent;
pub mod lines;
pub mod text;
pub mod wrap;
use self::ascii_canvas::AsciiView;

pub trait Content {
    fn min_width(&self) -> usize;
    fn emit(&self, view: &mut AsciiView);

    /// Emit at a particular upper-left corner, returning the
    /// lower-right corner that was emitted.
    fn emit_at(&self,
               view: &mut AsciiView,
               row: usize,
               column: usize)
               -> (usize, usize) {
        let mut subview = view.view(row, column);
        self.emit(&mut subview);
        subview.close()
    }
}

pub trait WrapContent: Content {
    fn width(&self) -> usize;
}

pub struct Message {
    span: Span,
    heading: Box<Content>,
    body: Box<Content>,
}
