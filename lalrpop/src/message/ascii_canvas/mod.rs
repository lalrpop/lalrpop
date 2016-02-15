//! An "ASCII Canvas" allows us to draw lines and write text into a
//! fixed-sized canvas and then convert that canvas into ASCII
//! characters. ANSI styling is supported.

use ansi_term::Style;
use std::cmp;
use std::ops::Range;

mod row;
#[cfg(test)] mod test;

pub use self::row::Row;

pub trait CanvasLike {
    fn columns(&self) -> usize;
    fn read_char(&mut self, row: usize, column: usize) -> char;
    fn write_char(&mut self, row: usize, column: usize, ch: char, style: Style);
}

pub struct AsciiCanvas {
    columns: usize,
    rows: usize,
    characters: Vec<char>,
    styles: Vec<Style>,
}

/// To use an `AsciiCanvas`, first create the canvas, then draw any
/// lines, then write text labels. It is required to draw the lines
/// first so that we can detect intersecting lines properly (we could
/// track which characters belong to lines, I suppose).
impl AsciiCanvas {
    pub fn new(rows: usize, columns: usize) -> Self {
        AsciiCanvas {
            rows: rows,
            columns: columns,
            characters: vec![' '; columns * rows],
            styles: vec![Style::new(); columns * rows],
        }
    }

    fn grow_rows_if_needed(&mut self, new_rows: usize) {
        if new_rows >= self.rows {
            let new_chars = (new_rows - self.rows) * self.columns;
            self.characters.extend((0..new_chars).map(|_| ' '));
            self.styles.extend((0..new_chars).map(|_| Style::new()));
            self.rows = new_rows;
        }
    }

    fn index(&mut self, r: usize, c: usize) -> usize {
        self.grow_rows_if_needed(r + 1);
        self.in_range_index(r, c)
    }

    fn in_range_index(&self, r: usize, c: usize) -> usize {
        assert!(r < self.rows);
        assert!(c <= self.columns);
        r * self.columns + c
    }

    fn start_index(&self, r: usize) -> usize {
        self.in_range_index(r, 0)
    }

    fn end_index(&self, r: usize) -> usize {
        self.in_range_index(r, self.columns)
    }

    pub fn to_strings(&self) -> Vec<Row> {
        (0..self.rows)
            .map(|row| {
                let start = self.start_index(row);
                let end = self.end_index(row);
                let chars = &self.characters[start..end];
                let styles = &self.styles[start..end];
                Row::new(chars, styles)
            })
            .collect()
    }

    pub fn view<'c>(&'c mut self) -> AsciiView<'c> {
        AsciiView::new(self, 0, 0)
    }

    pub fn view_at<'c>(&'c mut self, row: usize, column: usize) -> AsciiView<'c> {
        AsciiView::new(self, row, column)
    }
}

impl CanvasLike for AsciiCanvas {
    fn columns(&self) -> usize {
        self.columns
    }

    fn read_char(&mut self, r: usize, c: usize) -> char {
        let index = self.index(r, c);
        self.characters[index]
    }

    fn write_char(&mut self,
                  row: usize,
                  column: usize,
                  ch: char,
                  style: Style)
    {
        let index = self.index(row, column);
        self.characters[index] = ch;
        self.styles[index] = style;
    }
}

#[derive(Copy, Clone)]
struct Point {
    row: usize,
    column: usize,
}

/// Gives a view onto an AsciiCanvas that has a fixed upper-left
/// point.
pub struct AsciiView<'canvas> {
    // either the base canvas or another view
    base: &'canvas mut CanvasLike,

    // fixed at creation: the content is always allowed to grow down,
    // but cannot grow right more than `num_columns`
    upper_left: Point,

    // this is updated to track content that is emitted
    lower_right: Point,
}

impl<'canvas> AsciiView<'canvas> {
    pub fn new(base: &'canvas mut CanvasLike,
               row: usize,
               column: usize)
               -> AsciiView {
        let upper_left = Point { row: row, column: column };
        AsciiView {
            base: base,
            upper_left: upper_left,
            lower_right: upper_left,
        }
    }

    pub fn view(&mut self, row: usize, column: usize) -> AsciiView {
        AsciiView::new(self, row, column)
    }

    // Finalize the view and learn how much was written.
    pub fn close(self) -> (usize, usize) {
        (self.lower_right.row, self.lower_right.column)
    }

    fn track_max(&mut self, row: usize, column: usize) {
        self.lower_right.row = cmp::max(self.lower_right.column, row);
        self.lower_right.column = cmp::max(self.lower_right.column, column);
    }

    pub fn draw_vertical_line(&mut self,
                              rows: Range<usize>,
                              column: usize)
    {
        for r in rows {
            let new_char = match self.read_char(r, column) {
                ' ' => '|',
                '|' => '|',
                '-' => '+',
                '+' => '+',
                _ => panic!("unexpected character when drawing lines"),
            };
            self.write_char(r, column, new_char, Style::new());
        }
    }

    pub fn draw_horizontal_line(&mut self,
                                row: usize,
                                columns: Range<usize>)
    {
        for c in columns {
            let new_char = match self.read_char(row, c) {
                ' ' => '-',
                '-' => '-',
                '|' => '+',
                '+' => '+',
                _ => panic!("unexpected character when drawing lines"),
            };
            self.write_char(row, c, new_char, Style::new());
        }
    }

    pub fn write_chars<I>(&mut self,
                          row: usize,
                          column: usize,
                          chars: I,
                          style: Style)
        where I: Iterator<Item=char>
    {
        for (i, ch) in chars.enumerate() {
            self.write_char(row, column + i, ch, style);
        }
    }

    /// Wraps words at the column limit into multiple rows.
    pub fn write_wrap<I>(&mut self,
                         mut row: usize,
                         column0: usize,
                         chars: I)
        where I: Iterator<Item=(char, Style)>
    {
        let mut column = column0;
        let mut buffer = Vec::new();
        let max_buf_len = self.columns() - column;

        // Accumulate characters until we have seen a word.
        for (ch, style) in chars {
            if ch.is_whitespace() {
                if !buffer.is_empty() {
                    self.drain_buffer(column0, &mut row, &mut column, &mut buffer);
                    column += 1; // leave a space between next word
                }
            } else {
                buffer.push((ch, style));

                if buffer.len() >= max_buf_len {
                    self.drain_buffer(column0, &mut row, &mut column, &mut buffer);
                }
            }
        }

        if !buffer.is_empty() {
            self.drain_buffer(column0, &mut row, &mut column, &mut buffer);
        }
    }

    fn drain_buffer(&mut self,
                    column0: usize,
                    row: &mut usize,
                    column: &mut usize,
                    buffer: &mut Vec<(char, Style)>)
    {
        let len = buffer.len();
        assert!(len > 0);

        if *column + len > self.columns() {
            *row += 1;
            *column = column0;
        }

        // should be ensured because we track the max-buf-len and
        // eagerly drain if it is exceeded
        assert!(*column + len <= self.columns());

        for (ch, style) in buffer.drain(..) {
            self.write_char(*row, *column, ch, style);
            *column += 1;
        }
    }
}

impl<'canvas> CanvasLike for AsciiView<'canvas> {
    fn columns(&self) -> usize {
        self.base.columns() - self.upper_left.column
    }

    fn read_char(&mut self, row: usize, column: usize) -> char {
        let row = self.upper_left.row + row;
        let column = self.upper_left.row + column;
        self.base.read_char(row, column)
    }

    fn write_char(&mut self,
                  row: usize,
                  column: usize,
                  ch: char,
                  style: Style)
    {
        let row = self.upper_left.row + row;
        let column = self.upper_left.row + column;
        self.track_max(row, column);
        self.base.write_char(row, column, ch, style)
    }
}


