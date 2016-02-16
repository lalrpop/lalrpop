use test_util::expect_debug;

use super::{AsciiCanvas, AsciiView};

#[test]
fn draw_box() {
    let mut canvas = AsciiCanvas::new(5, 10);
    {
        let view: &mut AsciiView = &mut canvas;
        view.draw_vertical_line(2..5, 2);
        view.draw_vertical_line(2..5, 7);
        view.draw_horizontal_line(2, 2..8);
        view.draw_horizontal_line(4, 2..8);
    }
    expect_debug(
        &canvas.to_strings(),
        r#"
[
    "",
    "",
    "  +----+",
    "  |    |",
    "  +----+"
]
"#.trim());
}

#[test]
fn grow_box() {
    let mut canvas = AsciiCanvas::new(0, 10);
    {
        let view: &mut AsciiView = &mut canvas;
        view.draw_vertical_line(2..5, 2);
        view.draw_vertical_line(2..5, 7);
        view.draw_horizontal_line(2, 2..8);
        view.draw_horizontal_line(4, 2..8);
    }
    expect_debug(
        &canvas.to_strings(),
        r#"
[
    "",
    "",
    "  +----+",
    "  |    |",
    "  +----+"
]
"#.trim());
}

