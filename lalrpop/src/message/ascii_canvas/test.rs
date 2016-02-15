use super::AsciiCanvas;
use test_util::expect_debug;

#[test]
fn draw_box() {
    let mut canvas = AsciiCanvas::new(5, 10);
    canvas.draw_vertical_line(2..5, 2);
    canvas.draw_vertical_line(2..5, 7);
    canvas.draw_horizontal_line(2, 2..8);
    canvas.draw_horizontal_line(4, 2..8);
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
    canvas.draw_vertical_line(2..5, 2);
    canvas.draw_vertical_line(2..5, 7);
    canvas.draw_horizontal_line(2, 2..8);
    canvas.draw_horizontal_line(4, 2..8);
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
