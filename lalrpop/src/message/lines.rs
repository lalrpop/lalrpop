use super::*;
use super::ascii_canvas::AsciiView;

pub struct Lines {
    lines: Vec<Box<Content>>,
    separate: usize, // 0 => overlapping, 1 => each on its own line, 2 => paragraphs
}

impl Lines {
    pub fn new(lines: Vec<Box<Content>>, separate: usize) -> Self {
        Lines {
            lines: lines,
            separate: separate,
        }
    }
}

impl Content for Lines {
    fn min_width(&self) -> usize {
        self.lines.iter()
                  .map(|c| c.min_width())
                  .max()
                  .unwrap()
    }

    fn emit(&self, view: &mut AsciiView) {
        emit_lines(view, &self.lines, self.separate);
    }
}

pub fn emit_lines(view: &mut AsciiView,
                  lines: &[Box<Content>],
                  separate: usize)
{
    let mut row = 0;
    for line in lines {
        let (end_row, _) = line.emit_at(view, row, 0);
        row = end_row + separate;
    }
}
