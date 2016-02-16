use grammar::parse_tree::Span;

pub mod ascii_canvas;
pub mod builder;
pub mod citation;
pub mod horiz;
pub mod message;
pub mod indent;
pub mod styled;
pub mod text;
pub mod vert;
pub mod wrap;
use self::ascii_canvas::AsciiView;

/// Content which can be rendered.
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
        let mut shifted_view = view.shift(row, column);
        self.emit(&mut shifted_view);
        shifted_view.close()
    }

    /// When items are enclosed into a wrap, this method deconstructs
    /// them into their indivisible components.
    fn into_wrap_items(self: Box<Self>, wrap_items: &mut Vec<Box<Content>>);
}

/// Helper function: convert `content` into wrap items and then map
/// those with `op`, appending the final result into `wrap_items`.
/// Useful for "modifier" content items like `Styled` that do not
/// affect wrapping.
fn into_wrap_items_map<OP,C>(content: Box<Content>,
                             wrap_items: &mut Vec<Box<Content>>,
                             op: OP)
    where OP: FnMut(Box<Content>) -> C,
          C: Content + 'static,
{
        let mut subvector = vec![];
        content.into_wrap_items(&mut subvector);
        wrap_items.extend(
            subvector.into_iter()
                     .map(op)
                     .map(|item| Box::new(item) as Box<Content>));
}

pub use self::message::Message;
pub use self::builder::MessageBuilder;
