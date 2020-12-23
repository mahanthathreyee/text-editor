use std::io::{Write, stdout};

use crate::editor_config;

pub fn clear_screen(buffer: &mut String) {
    buffer.push_str("\x1b[H");
}

pub fn editor_refresh_screen(editor_config: editor_config::EditorConfig) {
    let mut buffer: String = String::new();
    clear_screen(&mut buffer);
    editor_draw_rows(editor_config, &mut buffer);
    buffer.push_str("\x1b[H");
    print!("{}", buffer);
    stdout().flush().unwrap();
}

pub fn editor_draw_rows(editor_config: editor_config::EditorConfig, buffer: &mut String) {
    for index in 0..(editor_config.rows) {
        buffer.push_str("~");
        buffer.push_str("\x1b[K");
        if index < editor_config.rows - 1 {
            buffer.push_str("\r\n");
        }
    }
}