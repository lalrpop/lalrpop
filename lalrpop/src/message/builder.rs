use ansi_term::Style;
use filetext::FileText;
use grammar::parse_tree::Span;
use message::{Content, Message};
use message::indent::Indent;
use message::horiz::Horiz;
use message::styled::Styled;
use message::text::Text;
use message::vert::Vert;
use message::wrap::Wrap;
use std::rc::Rc;

pub struct MessageBuilder {
    span: Span,
    file_text: Rc<FileText>,
    heading: Option<Box<Content>>,
    body: Option<Box<Content>>,
}

pub struct HeadingCharacter {
    message: MessageBuilder,
}

pub struct BodyCharacter {
    message: MessageBuilder,
}

impl MessageBuilder {
    pub fn new(span: Span, file_text: Rc<FileText>) -> Self {
        MessageBuilder { span: span, file_text: file_text,
                         heading: None, body: None }
    }

    pub fn heading(self) -> Builder<HeadingCharacter> {
        Builder::new(HeadingCharacter { message: self })
    }

    pub fn body(self) -> Builder<BodyCharacter> {
        Builder::new(BodyCharacter { message: self })
    }

    pub fn end(self) -> Message {
        Message::new(self.span,
                     self.file_text,
                     self.heading.expect("never defined a heading"),
                     self.body.expect("never defined a body"))
    }
}

impl Character for HeadingCharacter {
    type End = MessageBuilder;

    fn end(mut self, items: Vec<Box<Content>>) -> MessageBuilder {
        assert!(self.message.heading.is_none(),
                "already defined a heading for this message");
        self.message.heading = Some(Box::new(Vert::new(items, 1)));
        self.message
    }
}

impl Character for BodyCharacter {
    type End = MessageBuilder;

    fn end(mut self, items: Vec<Box<Content>>) -> MessageBuilder {
        assert!(self.message.body.is_none(),
                "already defined a body for this message");
        self.message.body = Some(Box::new(Vert::new(items, 2)));
        self.message
    }
}

///////////////////////////////////////////////////////////////////////////
// Builder -- generic helper for multi-part items

pub struct Builder<C: Character> {
    items: Vec<Box<Content>>,
    character: C,
}

impl<C: Character> Builder<C> {
    fn new(character: C) -> Self {
        Builder {
            items: vec![],
            character: character,
        }
    }

    pub fn push(mut self, item: Box<Content>) -> Self {
        self.items.push(item);
        self
    }

    fn pop(&mut self) -> Option<Box<Content>> {
        self.items.pop()
    }

    pub fn vert(self, separate: usize) -> Builder<VertCharacter<C>> {
        Builder::new(VertCharacter {
            base: self,
            separate: separate,
        })
    }

    pub fn lines(self) -> Builder<VertCharacter<C>> {
        self.vert(1)
    }

    pub fn paragraphs(self) -> Builder<VertCharacter<C>> {
        self.vert(2)
    }

    pub fn horiz(self, separate: usize) -> Builder<HorizCharacter<C>> {
        Builder::new(HorizCharacter {
            base: self,
            separate: separate,
        })
    }

    /// "item1 item2"
    pub fn spaced(self) -> Builder<HorizCharacter<C>> {
        self.horiz(2) // what you normally want
    }

    pub fn wrap(self) -> Builder<WrapCharacter<C>> {
        Builder::new(WrapCharacter {
            base: self,
        })
    }

    pub fn bold(mut self) -> Self {
        let content = self.pop().expect("bold must be applied to an item");
        self.push(Box::new(Styled::new(Style::bold, content)))
    }

    pub fn underline(mut self) -> Self {
        let content = self.pop().expect("underline must be applied to an item");
        self.push(Box::new(Styled::new(Style::underline, content)))
    }

    pub fn indent(mut self, amount: usize) -> Self {
        let content = self.pop().expect("indent must be applied to an item");
        self.push(Box::new(Indent::new(4, content)))
    }

    pub fn text<T:ToString>(mut self, text: T) -> Self {
        self.push(Box::new(Text::new(text.to_string())))
    }

    pub fn end(self) -> C::End {
        self.character.end(self.items)
    }
}

pub trait Character {
    type End;
    fn end(self, items: Vec<Box<Content>>) -> Self::End;
}

///////////////////////////////////////////////////////////////////////////

pub struct HorizCharacter<C: Character> {
    base: Builder<C>,
    separate: usize,
}

impl<C: Character> Character for HorizCharacter<C> {
    type End = Builder<C>;

    fn end(self, items: Vec<Box<Content>>) -> Builder<C> {
        self.base.push(Box::new(Horiz::new(items, self.separate)))
    }
}

///////////////////////////////////////////////////////////////////////////

pub struct VertCharacter<C: Character> {
    base: Builder<C>,
    separate: usize,
}

impl<C: Character> Character for VertCharacter<C> {
    type End = Builder<C>;

    fn end(self, items: Vec<Box<Content>>) -> Builder<C> {
        self.base.push(Box::new(Vert::new(items, self.separate)))
    }
}

///////////////////////////////////////////////////////////////////////////

pub struct WrapCharacter<C: Character> {
    base: Builder<C>,
}

impl<C: Character> Character for WrapCharacter<C> {
    type End = Builder<C>;

    fn end(self, items: Vec<Box<Content>>) -> Builder<C> {
        self.base.push(Box::new(Wrap::new(items)))
    }
}
