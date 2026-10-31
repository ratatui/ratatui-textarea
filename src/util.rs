use ratatui_core::buffer::CellWidth as _;

pub fn spaces(size: u8) -> &'static str {
    const SPACES: &str = "                                                                                                                                                                                                                                                                ";
    &SPACES[..size as usize]
}

/// The number of cells ratatui draws `c` in. Unlike `UnicodeWidthChar::width`, this counts the
/// halfwidth katakana sound marks `ﾞ` and `ﾟ` as one cell each. Control chars are not drawn, so
/// they take no cells.
pub fn char_width(c: char) -> usize {
    if c.is_control() {
        0
    } else {
        c.encode_utf8(&mut [0; 4]).cell_width().into()
    }
}

pub fn num_digits(i: usize) -> u8 {
    f64::log10(i as f64) as u8 + 1
}

#[derive(Debug, Clone)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
    pub offset: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize, offset: usize) -> Self {
        Self { row, col, offset }
    }
}
