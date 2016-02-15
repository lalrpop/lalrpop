use super::*;
use super::ascii_canvas::AsciiView;

pub struct Indent {
    amount: usize,
    content: Box<Content>
}

impl Content for Indent {
    fn min_width(&self) -> usize {
        self.content.min_width() + self.amount
    }

    fn emit(&self, view: &mut AsciiView) {
        let mut subview = view.view(0, self.amount);
        self.content.emit(&mut subview);
    }
}
