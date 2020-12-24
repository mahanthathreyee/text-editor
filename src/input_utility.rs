use std::str;
use std::io::{Read, stdin};

use crate::{constants, editor_visual};

pub(crate) fn editor_read_key() -> Option<String> {
    let mut input_buffer = [0; constants::INPUT_BUFFER_SIZE];
    
    while stdin().read_exact(&mut input_buffer).is_err() {}
    match str::from_utf8(& input_buffer) {
        Ok(input_char) => {
            Some(input_char.to_string())
        }
        Err(e) => {
            editor_visual::error_and_exit(format!("{} {}", constants::MESSAGE_ERROR_INVALID_UTF8, e));
            None
        }
    }
}

pub fn preprocess_characters(input_char: String) -> String {
    match input_char.as_str() {
        constants::TERMINAL_ESCAPE => {
            editor_read_key();
            let arrow_key = editor_read_key();
            if let Some(arrow) = arrow_key {
                match arrow.as_str() {
                    constants::ARROW_UP => constants::MOVE_CURSOR_UP.to_string(),
                    constants::ARROW_DOWN => constants::MOVE_CURSOR_DOWN.to_string(),
                    constants::ARROW_RIGHT => constants::MOVE_CURSOR_RIGHT.to_string(),
                    constants::ARROW_LEFT => constants::MOVE_CURSOR_LEFT.to_string(),
                    _ => input_char
                }
            } else {
                input_char
            }
        }
        _ => input_char
    }
}