use ansi_term::{ANSIString, ANSIStrings, Style};
use std::fmt::{Debug, Display, Formatter, Error};

pub struct Row {
    text: String,
    styles: Vec<Style>
}

impl Row {
    pub fn new(chars: &[char], styles: &[Style]) -> Row {
        Row {
            text: chars.iter().cloned().collect(),
            styles: styles.to_vec()
        }
    }
}

impl Display for Row {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        // FIXME -- trim_right when not styled
        let ansi_strings: Vec<ANSIString> =
            self.text.char_indices()
                     .map(|(index, ch)| {
                         let len = ch.len_utf8();
                         &self.text[index..index+len]
                     })
                     .zip(self.styles.iter().cloned())
                     .map(|(str, style)| style.paint(str))
                     .collect();
        Display::fmt(&ANSIStrings(&ansi_strings), fmt)
    }
}

impl Debug for Row {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Debug::fmt(self.text.trim_right(), fmt)
    }
}
