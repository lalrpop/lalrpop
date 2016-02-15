use grammar::parse_tree::Span;
use std::io;

pub mod ascii_canvas;
use self::ascii_canvas::AsciiCanvas;

pub trait Content {
    fn emit(&mut self, out: &mut AsciiCanvas, upper_left: Point);
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point {
    row: usize,
    col: usize,
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



