//! An "ASCII Canvas" allows us to draw lines and write text into a
//! fixed-sized canvas and then convert that canvas into ASCII
//! characters. ANSI styling is supported.

use ansi_term::Style;
use std::cmp;
use std::ops::Range;

mod row;
#[cfg(test)] mod test;

pub use self::row::Row;

pub trait AsciiView {
    fn columns(&self) -> usize;
    fn read_char(&mut self, row: usize, column: usize) -> char;
    fn write_char(&mut self, row: usize, column: usize, ch: char, style: Style);
}

impl<'a> AsciiView+'a {
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

    pub fn shift<'c>(&'c mut self, row: usize, column: usize) -> ShiftedView<'c> {
        ShiftedView::new(self, row, column)
    }

    pub fn styled<'c, F>(&'c mut self, style_fn: F) -> StyleView<'c, F>
        where F: Fn(&Style) -> Style
    {
        StyleView::new(self, style_fn)
    }
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
}

impl AsciiView for AsciiCanvas {
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
pub struct ShiftedView<'canvas> {
    // either the base canvas or another view
    base: &'canvas mut AsciiView,

    // fixed at creation: the content is always allowed to grow down,
    // but cannot grow right more than `num_columns`
    upper_left: Point,

    // this is updated to track content that is emitted
    lower_right: Point,
}

impl<'canvas> ShiftedView<'canvas> {
    pub fn new(base: &'canvas mut AsciiView,
               row: usize,
               column: usize)
               -> Self {
        let upper_left = Point { row: row, column: column };
        ShiftedView {
            base: base,
            upper_left: upper_left,
            lower_right: upper_left,
        }
    }

    // Finalize the view and learn how much was written.  What is
    // returned is the *maximal* row/column -- so if you write to that
    // location, you would overwrite some of the content that was
    // written.
    pub fn close(self) -> (usize, usize) {
        (self.lower_right.row, self.lower_right.column)
    }

    fn track_max(&mut self, row: usize, column: usize) {
        self.lower_right.row = cmp::max(self.lower_right.row, row);
        self.lower_right.column = cmp::max(self.lower_right.column, column);
    }
}

impl<'canvas> AsciiView for ShiftedView<'canvas> {
    fn columns(&self) -> usize {
        self.base.columns() - self.upper_left.column
    }

    fn read_char(&mut self, row: usize, column: usize) -> char {
        let row = self.upper_left.row + row;
        let column = self.upper_left.column + column;
        self.base.read_char(row, column)
    }

    fn write_char(&mut self,
                  row: usize,
                  column: usize,
                  ch: char,
                  style: Style)
    {
        let row = self.upper_left.row + row;
        let column = self.upper_left.column + column;
        self.track_max(row, column);
        self.base.write_char(row, column, ch, style)
    }
}

pub struct StyleView<'canvas, F>
    where F: Fn(&Style) -> Style
{
    base: &'canvas mut AsciiView,
    style_fn: F
}

impl<'canvas, F> StyleView<'canvas, F>
    where F: Fn(&Style) -> Style
{
    pub fn new(base: &'canvas mut AsciiView, style_fn: F) -> Self {
        StyleView {
            base: base,
            style_fn: style_fn
        }
    }
}

impl<'canvas, F> AsciiView for StyleView<'canvas, F>
    where F: Fn(&Style) -> Style
{
    fn columns(&self) -> usize {
        self.base.columns()
    }

    fn read_char(&mut self, row: usize, column: usize) -> char {
        self.base.read_char(row, column)
    }

    fn write_char(&mut self,
                  row: usize,
                  column: usize,
                  ch: char,
                  style: Style)
    {
        let style = (self.style_fn)(&style);
        self.base.write_char(row, column, ch, style)
    }
}
