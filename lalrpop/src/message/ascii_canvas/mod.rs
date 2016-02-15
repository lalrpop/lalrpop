//! An "ASCII Canvas" allows us to draw lines and write text into a
//! fixed-sized canvas and then convert that canvas into ASCII
//! characters. ANSI styling is supported.

use ansi_term::Style;
use std::cmp;
use std::ops::Range;

mod row;
#[cfg(test)] mod test;

pub use self::row::Row;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point {
    row: usize,
    column: usize,
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

    pub fn columns(&self) -> usize {
        self.columns
    }

    fn grow_rows_if_needed(&mut self, new_rows: usize) {
        if new_rows >= self.rows {
            let new_chars = (new_rows - self.rows) * self.columns;
            self.characters.extend((0..new_chars).map(|_| ' '));
            self.styles.extend((0..new_chars).map(|_| Style::new()));
            self.rows = new_rows;
        }
    }

    fn index_grow(&mut self, r: usize, c: usize) -> usize {
        self.grow_rows_if_needed(r + 1);
        self.index(r, c)
    }

    fn index(&self, r: usize, c: usize) -> usize {
        assert!(r < self.rows);
        assert!(c <= self.columns);
        r * self.columns + c
    }

    fn start_index(&self, r: usize) -> usize {
        self.index(r, 0)
    }

    fn end_index(&self, r: usize) -> usize {
        self.index(r, self.columns)
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

    pub fn draw_vertical_line(&mut self,
                              rows: Range<usize>,
                              column: usize)
    {
        self.draw_vertical_line_styled(rows, column, Style::new())
    }

    pub fn draw_vertical_line_styled(&mut self,
                                     rows: Range<usize>,
                                     column: usize,
                                     style: Style)
    {
        for r in rows {
            let index = self.index_grow(r, column);
            let new_char = match self.characters[index] {
                ' ' => '|',
                '|' => '|',
                '-' => '+',
                '+' => '+',
                _ => panic!("unexpected character when drawing lines"),
            };
            self.characters[index] = new_char;
            self.styles[index] = style;
        }
    }

    pub fn draw_horizontal_line(&mut self,
                                row: usize,
                                columns: Range<usize>)
    {
        for c in columns {
            let index = self.index_grow(row, c);
            let new_char = match self.characters[index] {
                ' ' => '-',
                '-' => '-',
                '|' => '+',
                '+' => '+',
                _ => panic!("unexpected character when drawing lines"),
            };
            self.characters[index] = new_char;
        }
    }

    pub fn write_char(&mut self,
                      row: usize,
                      column: usize,
                      ch: char,
                      style: Style)
    {
        let index = self.index_grow(row, column);

        self.characters[index] = ch;
        self.styles[index] = style;
    }

    pub fn view<'c>(&'c mut self) -> AsciiView<'c> {
        AsciiView::new(self, Point { row: 0, column: 0 })
    }

    pub fn view_at<'c>(&'c mut self, upper_left: Point) -> AsciiView<'c> {
        AsciiView::new(self, upper_left)
    }
}

/// Gives a view onto an AsciiCanvas that has a fixed upper-left
/// point.
pub struct AsciiView<'canvas> {
    canvas: &'canvas mut AsciiCanvas,

    // fixed at creation: the content is always allowed to grow down,
    // but cannot grow right more than `num_columns`
    upper_left: Point,

    // this is updated to track content that is emitted
    lower_right: Point,
}

impl<'canvas> AsciiView<'canvas> {
    pub fn new(canvas: &'canvas mut AsciiCanvas,
               upper_left: Point)
               -> AsciiView {
        AsciiView {
            canvas: canvas,
            upper_left: upper_left,
            lower_right: upper_left,
        }
    }

    fn track_max(&mut self, row: usize, column: usize) {
        self.track_max_row(row);
        self.track_max_column(column);
    }

    fn track_max_row(&mut self, row: usize) {
        self.lower_right.row = cmp::max(self.lower_right.column, row);
    }

    fn track_max_column(&mut self, column: usize) {
        self.lower_right.column = cmp::max(self.lower_right.column, column);
    }

    pub fn columns(&self) -> usize {
        self.canvas.columns() - self.upper_left.column
    }

    pub fn draw_vertical_line(&mut self,
                              rows: Range<usize>,
                              column: usize)
    {
        let upper_left = self.upper_left;
        self.track_max(rows.end + upper_left.row - 1, column);
        self.canvas.draw_vertical_line(
            (rows.start + upper_left.row .. rows.end + upper_left.row),
            column + upper_left.column)
    }

    pub fn draw_horizontal_line(&mut self,
                                row: usize,
                                columns: Range<usize>)
    {
        let upper_left = self.upper_left;
        self.track_max(row + upper_left.row, columns.end + upper_left.column - 1);
        self.canvas.draw_horizontal_line(
            row + upper_left.row,
            (columns.start + upper_left.column .. columns.end + upper_left.column))
    }

    pub fn write_chars<I>(&mut self,
                          row: usize,
                          column: usize,
                          chars: I,
                          style: Style)
        where I: Iterator<Item=char>
    {
        let row = row + self.upper_left.row;
        let column = column + self.upper_left.column;

        for (i, ch) in chars.enumerate() {
            self.track_max(row, column + i);
            self.canvas.write_char(row, column + i, ch, style);
        }
    }

    /// Wraps words at the column limit into multiple rows.
    pub fn write_wrap<I>(&mut self,
                         row: usize,
                         column: usize,
                         chars: I,
                         style: Style)
        where I: Iterator<Item=char>
    {
        let mut row = row + self.upper_left.row;
        let column0 = column + self.upper_left.column;
        let mut column = column0;
        let mut buffer = String::new();
        let max_buf_len = self.columns() - column;

        // Accumulate characters until we have seen a word.
        for ch in chars {
            if ch.is_whitespace() {
                if !buffer.is_empty() {
                    self.drain_buffer(column0, style,
                                      &mut row, &mut column, &mut buffer);
                    column += 1; // leave a space between next word
                }
            } else {
                buffer.push(ch);

                if buffer.len() >= max_buf_len {
                    self.drain_buffer(column0, style,
                                      &mut row, &mut column, &mut buffer);
                }
            }
        }

        if !buffer.is_empty() {
            self.drain_buffer(column0, style,
                              &mut row, &mut column, &mut buffer);
        }
    }

    fn drain_buffer(&mut self,
                    column0: usize,
                    style: Style,
                    row: &mut usize,
                    column: &mut usize,
                    buffer: &mut String)
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

        self.write_chars(*row, *column, buffer.drain(..), style);
        *column += len;
    }
}
