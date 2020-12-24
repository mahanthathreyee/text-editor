use std::str;
use std::io::{Read, stdin};

use crate::{constants, editor_config, editor_visual, editor_cursor};

fn editor_read_key() -> Option<String> {
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

fn preprocess_characters(input_char: String) -> String {
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

pub fn editor_process_key(editor_config: &mut editor_config::EditorConfig) {    
    let mut buffer = String::new();
    loop {
        let input = editor_read_key();
        if let Some(input_char) = input {
            let input_char = preprocess_characters(input_char);
            match input_char.as_str() {
                constants::QUIT => {
                    editor_visual::error_and_exit(String::new());
                    break;
                }
                constants::MOVE_CURSOR_UP | constants::MOVE_CURSOR_LEFT | constants::MOVE_CURSOR_DOWN | constants::MOVE_CURSOR_RIGHT => {
                    editor_cursor::editor_scroll(editor_config, input_char, 1, &mut buffer);
                }
                _ => {
                    println!("Unknown command {}", input_char);
                }
            }
        }
         if ! buffer.is_empty() {
             editor_visual::flush_buffer(&mut buffer);
         }
    }
}