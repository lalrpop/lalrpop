use std::cmp;
use super::*;
use super::ascii_canvas::{AsciiView, CanvasLike};

pub struct Wrap {
    content: Vec<Box<WrapContent>>,
}

impl Content for Wrap {
    fn min_width(&self) -> usize {
        self.content.iter()
                    .map(|c| c.min_width())
                    .max()
                    .unwrap()
    }

    fn emit(&self, view: &mut AsciiView) {
        let columns = view.columns();
        let mut row = 0;    // current row
        let mut height = 1; // max height of anything in this row
        let mut column = 0; // current column in this row

        for content in &self.content {
            let len = content.width();

            if column + len > columns {
                column = 0;
                row += height;
                height = 1;
            }

            assert!(column + len < columns);

            let (c_row, c_column) = content.emit_at(view, row, column);
            assert!(c_column >= column);
            column = c_column + 1;
            height = cmp::max(c_row - row + 1, height);
        }
    }
}
