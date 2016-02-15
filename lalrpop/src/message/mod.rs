use grammar::parse_tree::Span;

pub mod ascii_canvas;
use self::ascii_canvas::AsciiView;

pub trait Content {
    fn emit(&mut self, view: &mut AsciiView);
}

pub struct Message {
    span: Span,
    heading: Box<Content>,
    body: Box<Content>,
}

pub struct Text {
    span: Span,
    heading: Box<Content>,
    body: Box<Content>,
}



