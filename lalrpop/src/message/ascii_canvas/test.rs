use ansi_term::Style;
use test_util::expect_debug;

use super::AsciiCanvas;

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

#[test]
fn wrap() {
    let mut canvas = AsciiCanvas::new(0, 10);
    {
        let mut view = canvas.view();
        view.write_wrap(3, 5, "Hi Ho Off ToWorkWeGo".chars(), Style::new());
    }

    expect_debug(
        &canvas.to_strings(),
        r#"
[
    "",
    "",
    "",
    "     Hi Ho",
    "     Off",
    "     ToWor",
    "     kWeGo"
]
"#.trim());
}
