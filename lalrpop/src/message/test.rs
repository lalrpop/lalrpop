use ascii_canvas::AsciiCanvas;
use file_text::FileText;
use grammar::parse_tree::Span;
use message::builder::MessageBuilder;
use session::Session;
use std::path::PathBuf;
use std::rc::Rc;
use test_util::expect_debug;
use tls::Tls;

use super::*;

fn install_tls() -> Tls {
    let session = Rc::new(Session::test());
    let path = PathBuf::from("tmp.txt");
    let text = r#"foo
bar
baz
"#;
    let file_text = Rc::new(FileText::new(path, text.to_string()));
    Tls::install(session, file_text)
}

#[test]
fn hello_world() {
    let _tls = install_tls();
    let msg =
        MessageBuilder::new(Span(0, 2))
        .heading()
        .text("Hello, world!")
        .end()
        .body()
        .wrap()
        .text("This is a very, very, very, very long sentence. \
               OK, not THAT long!")
        .end()
        .indent_by(4)
        .end()
        .end();
    let min_width = msg.min_width();
    println!("min_width={}", min_width);
    let mut canvas = AsciiCanvas::new(0, min_width);
    msg.emit(&mut canvas);
    expect_debug(&canvas.to_strings(), r#"
[
    "tmp.txt:1:1: 1:2: Hello, world!",
    "",
    "      This is a very, very,",
    "      very, very long sentence.",
    "      OK, not THAT long!"
]
"#.trim());
}

#[test]
fn paragraphs() {
    let _tls = install_tls();
    let msg =
        MessageBuilder::new(Span(0, 2))
        .heading()
        .text("Hello, world!")
        .end() // heading
        .body()
        .paragraphs()
        .wrap()
        .text("This is the first paragraph. It contains a lot of really interesting \
               information that the reader will no doubt peruse with care.")
        .end()
        .wrap()
        .text("This is the second paragraph. It contains even more really interesting \
               information that the reader will no doubt skip over with wild abandon.")
        .end()
        .wrap()
        .text("This is the final paragraph. The reader won't even spare this one \
               a second glance, despite it containing just waht they need to know \
               to solve their problem and to derive greater pleasure from life. \
               The secret: All you need is love! Dum da da dum.")
        .end()
        .end()
        .end()
        .end();
    let min_width = msg.min_width();
    println!("min_width={}", min_width);
    let mut canvas = AsciiCanvas::new(0, min_width);
    msg.emit(&mut canvas);
    expect_debug(&canvas.to_strings(), r#"
[
    "tmp.txt:1:1: 1:2: Hello, world!",
    "",
    "  This is the first paragraph.",
    "  It contains a lot of really",
    "  interesting information that",
    "  the reader will no doubt",
    "  peruse with care.",
    "",
    "  This is the second paragraph.",
    "  It contains even more really",
    "  interesting information that",
    "  the reader will no doubt skip",
    "  over with wild abandon.",
    "",
    "  This is the final paragraph.",
    "  The reader won\'t even spare",
    "  this one a second glance,",
    "  despite it containing just",
    "  waht they need to know to",
    "  solve their problem and to",
    "  derive greater pleasure from",
    "  life. The secret: All you",
    "  need is love! Dum da da dum."
]
"#.trim());
}
