use termios::{Termios};
use libc::{STDIN_FILENO};

use crate::{constants};

#[derive(Clone)]
pub struct EditorConfig {
    pub orig_termios: Termios,
    
    pub rows: u16,
    pub cols: u16,
    
    pub cursor_x: u16,
    pub cursor_y: u16,

    pub lines: Vec<EditorLine>
}

impl EditorConfig {
    pub fn new() -> EditorConfig {
        EditorConfig {
            orig_termios: Termios::from_fd(STDIN_FILENO).unwrap(),
            rows: 0,
            cols: 0,
            cursor_x: constants::EDITOR_NUMBER_LINE_INDEX,
            cursor_y: 0,
            lines: Vec::new()
        }
    }

    pub fn set_terminal_dimensions(&mut self, rows: u16, cols: u16) {
        self.rows = rows;
        self.cols = cols;
    }
}

#[derive(Clone)]
pub struct EditorLine {
    pub content: String
}

impl EditorLine {
    pub fn new(content: String) -> EditorLine {
        EditorLine {
            content
        }
    }
}