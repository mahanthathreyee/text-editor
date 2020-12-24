use crate::{constants, editor_config, editor_cursor, editor_visual, input_utility};

pub fn command_matcher(input_char: &str, editor_config: &mut editor_config::EditorConfig) -> bool {
    let mut buffer = String::new();

    buffer.push_str(constants::ANSI_CURSOR_THIN);
    editor_cursor::move_cursor_to_position(0, editor_config.rows, editor_config, &mut buffer);
    buffer.push_str(input_char);
    editor_visual::flush_buffer(&mut buffer);

    let mut command_buffer = String::new();
    loop {
        if let Some(mut command_char) = input_utility::editor_read_key() {
            match command_char.as_str() {
                constants::NEW_LINE => break,
                _ => {
                    if command_char.chars().all(char::is_alphanumeric) {
                        command_buffer.push_str(&command_char.to_string());
                        editor_visual::flush_buffer(&mut command_char);
                    }
                }
            }
        }
    }

    if ! command_buffer.is_empty() {
        match command_buffer.as_str() {
            constants::QUIT => {
                editor_visual::error_and_exit(String::new());
                true
            }
            _ => false
        }
    } else {
        false
    }
}