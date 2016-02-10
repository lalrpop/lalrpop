use ansi_term::Style;
use std::ops::Range;

mod row;
#[cfg(test)] mod test;

pub use self::row::Row;

pub struct AsciiCanvas {
    columns: usize,
    rows: usize,
    characters: Vec<char>,
    styles: Vec<Style>,
}

impl AsciiCanvas {
    pub fn new(rows: usize, columns: usize) -> Self {
        AsciiCanvas {
            rows: rows,
            columns: columns,
            characters: vec![' '; columns * rows],
            styles: vec![Style::new(); columns * rows],
        }
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
            let index = self.index(r, column);
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
            let index = self.index(row, c);
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

    pub fn write<I>(&mut self,
                    row: usize,
                    column: usize,
                    chars: I)
        where I: Iterator<Item=char>
    {
        self.write_styled(row, column, chars, Style::new());
    }

    pub fn write_styled<I>(&mut self,
                           row: usize,
                           column: usize,
                           chars: I,
                           style: Style)
        where I: Iterator<Item=char>
    {
        for (i, ch) in chars.enumerate() {
            let index = self.index(row, column + i);
            self.characters[index] = ch;
            self.styles[index] = style;
        }
    }
}

