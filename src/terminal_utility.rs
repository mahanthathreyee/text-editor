use std::str;
use std::io::{Read, Write, stdin, stdout};
use termios::{tcsetattr};
use termios::{BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP, IXON, OPOST, TCSAFLUSH};
use libc::{STDIN_FILENO, VMIN, VTIME};
use libc::{STDOUT_FILENO, ioctl, TIOCGWINSZ, winsize};

use crate::{constants, editor_config, editor_visual};

pub fn enable_raw_mode(editor_config: &editor_config::EditorConfig) {
    let mut termios = editor_config.orig_termios;
    
    termios.c_lflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    termios.c_lflag &= !(OPOST);
    termios.c_lflag |= CS8;
    termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
    termios.c_cc[VMIN] = 0;
    termios.c_cc[VTIME] = 1;

    match tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios) {
        Ok(_) => {}
        Err(e) => {
            editor_visual::error_and_exit(format!("{} {}", constants::MESSAGE_ERROR_SETTING_TERMINAL_CONFIG, e));
        }
    }
}

pub fn disable_raw_mode(editor_config: &editor_config::EditorConfig) {
    match tcsetattr(STDIN_FILENO, TCSAFLUSH, &editor_config.orig_termios) {
        Ok(_) => {
            let mut buffer: String = String::new();
            editor_visual::clear_screen(&mut buffer);
            editor_visual::flush_buffer(&mut buffer);
        }
        Err(e) => {
            editor_visual::error_and_exit(format!("{} {}", constants::MESSAGE_ERROR_SETTING_TERMINAL_CONFIG, e));
        }
    }
}

pub fn get_window_size(editor_config: &mut editor_config::EditorConfig) -> bool {
    let mut window_size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut window_size) } != -1 
        || window_size.ws_row == 0 
        || window_size.ws_col == 0 
    {
        let rows = window_size.ws_row;
        let cols = window_size.ws_col;
    
        if rows > 0 && cols > 0 {
            editor_config.set_terminal_dimensions(rows, cols);
            return true;
        }
    } else {
        get_window_size_by_cursor_position(editor_config);
        return true;
    }

    false
}

fn get_window_size_by_cursor_position(editor_config: &mut editor_config::EditorConfig) {
    println!("\x1b[999C\x1b[999B\x1b[6n");
    stdout().flush().unwrap();
    
    
    let mut input_buffer = [0; constants::INPUT_BUFFER_SIZE];
    let mut dimensions: Vec<char> = Vec::new(); //Stores the split of "\u{1b}[<rows>;<cols>R"
    
    while stdin().read_exact(&mut input_buffer).is_ok() {
        match str::from_utf8(& input_buffer) {
            Ok(input_char) => {
                let input_str = input_char.to_string();
                dimensions.extend(input_str.chars());
            }
            Err(e) => {
                editor_visual::error_and_exit(format!("{} {}", constants::MESSAGE_ERROR_DIMENSION_INTEGER_PARSE, e));
            }
        }
    }

    dimensions.remove(0);    //Remove '\u{1b}'
    dimensions.remove(0);    //Remove `[`
    dimensions.remove(dimensions.len() - 1);  //Remove 'R'

    let dimensions: String = dimensions.into_iter().collect();
    let dimensions: Vec<&str> = dimensions.split(';').collect();

    let rows = dimensions[0].parse::<u16>().expect(constants::MESSAGE_ERROR_DIMENSION_INTEGER_PARSE);
    let cols = dimensions[1].parse::<u16>().expect(constants::MESSAGE_ERROR_DIMENSION_INTEGER_PARSE);

    editor_config.set_terminal_dimensions(rows, cols);
}