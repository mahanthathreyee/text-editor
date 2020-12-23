use std::str;
use std::io::{Read, stdin};

use crate::{constants, editor_config, editor_visual};

fn editor_read_key() -> Option<String> {
    let mut input_buffer = [0; constants::INPUT_BUFFER_SIZE];
    
    while stdin().read_exact(&mut input_buffer).is_err() {}
    match str::from_utf8(& input_buffer) {
        Ok(input_char) => {
            Some(input_char.to_string())
        }
        Err(e) => panic!("{} {}", constants::MESSAGE_ERROR_INVALID_UTF8, e)
    }
}

pub fn editor_process_key(editor_config: editor_config::EditorConfig) {
    editor_visual::editor_refresh_screen(editor_config);
    
    let input = editor_read_key();
    println!("{:?}", input);

    match input {
        Some(input_char) => {
            match input_char.as_str() {
                constants::CTRL_Q => {
                    println!("Quitting")
                }
                _ => println!("Unknown command")
            }
        }

        None => {
            println!("Unknown command");
        }
    }
}