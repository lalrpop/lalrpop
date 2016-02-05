use std::ops::Range;

mod test;

pub struct AsciiCanvas {
    columns: usize,
    rows: usize,
    characters: Vec<char>,
}

impl AsciiCanvas {
    pub fn new(rows: usize, columns: usize) -> Self {
        AsciiCanvas {
            rows: rows,
            columns: columns,
            characters: vec![' '; columns * rows],
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

    fn row(&self, r: usize) -> &[char] {
        &self.characters[self.start_index(r) .. self.end_index(r)]
    }

    pub fn to_strings(&self) -> Vec<String> {
        (0..self.rows)
            .map(|row| {
                let s: String = self.row(row).iter().cloned().collect();
                String::from(s.trim_right())
            })
            .collect()
    }

    pub fn draw_vertical_line(&mut self,
                              rows: Range<usize>,
                              column: usize)
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
        for (i, ch) in chars.enumerate() {
            let index = self.index(row, column + i);
            self.characters[index] = ch;
        }
    }
}
