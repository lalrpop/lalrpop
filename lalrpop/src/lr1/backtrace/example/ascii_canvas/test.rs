use super::AsciiCanvas;

#[test] fn draw_box() {
    let mut canvas = AsciiCanvas::new(5, 10);
    canvas.draw_vertical_line(2..5, 2);
    canvas.draw_vertical_line(2..5, 7);
    canvas.draw_horizontal_line(2, 2..8);
    canvas.draw_horizontal_line(4, 2..8);
    assert_eq!(
        canvas.to_strings(),
        vec!["",
             "",
             "  +----+",
             "  |    |",
             "  +----+"]);
}
