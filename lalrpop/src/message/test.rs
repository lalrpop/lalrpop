use message::ascii_canvas::AsciiCanvas;
use filetext::FileText;
use grammar::parse_tree::Span;
use std::path::PathBuf;
use std::rc::Rc;
use test_util::expect_debug;

use super::*;

fn file_text() -> Rc<FileText> {
    let path = PathBuf::from("tmp.txt");
    let text = r#"foo
bar
baz
"#;
    Rc::new(FileText::new(path, text.to_string()))
}

#[test]
fn hello_world() {
    let msg =
        MessageBuilder::new(Span(0, 2), file_text())
        .heading()
        .text("Hello, world!")
        .end() // heading
        .body()
        .wrap()
        .text("This is a very, very, very, very long sentence. \
               OK, not THAT long!")
        .end()
        .indent_by(4) // note that message already indents by 2
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
    let msg =
        MessageBuilder::new(Span(0, 2), file_text())
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
