use termios::{Termios};
use libc::{STDIN_FILENO};

use crate::terminal_utility;

#[derive(Copy, Clone)]
pub struct EditorConfig {
    pub orig_termios: Termios,
    pub rows: u16,
    pub cols: u16 
}

impl EditorConfig {
    pub fn new() -> EditorConfig {
        EditorConfig {
            orig_termios: Termios::from_fd(STDIN_FILENO).unwrap(),
            rows: 0,
            cols: 0
        }
    }

    pub fn set_terminal_dimensions(&mut self, rows: u16, cols: u16) {
        self.rows = rows;
        self.cols = cols;
    }

    pub fn init_editor(&mut self) -> bool {
        terminal_utility::get_window_size(self)
    }
}
