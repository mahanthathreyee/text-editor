use crate::{constants, editor, editor_config, editor_visual};

pub fn move_cursor(editor_config: &editor_config::EditorConfig, buffer: &mut String) {
    buffer.push_str(&constants::ANSI_HIDE_CURSOR);
    buffer.push_str(&format!("\x1b[{};{}H", editor_config.cursor_y, editor_config.cursor_x));
    buffer.push_str(&constants::ANSI_SHOW_CURSOR);
}

pub fn move_cursor_to_position(x: u16, y:u16, editor_config: &mut editor_config::EditorConfig, buffer: &mut String) {
    editor_config.cursor_x = x;
    editor_config.cursor_y = y;
    move_cursor(editor_config, buffer);
}

pub fn save_cursor_and_move_position(x: u16, y:u16, buffer: &mut String, hide_cursor: bool) {
    if hide_cursor {   
        buffer.push_str(&constants::ANSI_HIDE_CURSOR);
    }
    buffer.push_str(constants::ANSI_CURSOR_SAVE);
    buffer.push_str(&format!("\x1b[{};{}H", y, x));
}

pub fn restore_cursor(buffer: &mut String) {
    buffer.push_str(constants::ANSI_CURSOR_RESTORE);
    buffer.push_str(&constants::ANSI_SHOW_CURSOR);
}

pub fn editor_scroll(editor_config: &mut editor_config::EditorConfig, input_char: String, scroll: u16) {
    let mut buffer = String::new();
    match input_char.as_str() {
        constants::MOVE_CURSOR_UP => {
            if editor_config.cursor_y > scroll && editor_config.cursor_y - scroll > 0 {
                editor_config.cursor_y -= scroll;
            }
        }
        constants::MOVE_CURSOR_DOWN => {
            if editor_config.cursor_y + scroll < (editor_config.lines.len() - 1) as u16 {
                editor_config.cursor_y += scroll;
            }
        }
        constants::MOVE_CURSOR_LEFT => {
            if editor_config.cursor_x - scroll > constants::EDITOR_NUMBER_LINE_INDEX + 1 {
                editor_config.cursor_x -= scroll;
            }
        }
        constants::MOVE_CURSOR_RIGHT => {
            let max_cursor_position = (editor_config.lines[(editor_config.cursor_y) as usize].content.len() + 1) as u16 + constants::EDITOR_NUMBER_LINE_INDEX + 2;
            if editor_config.cursor_x < max_cursor_position && editor_config.cursor_x + scroll < max_cursor_position {
                editor_config.cursor_x += scroll;
            }
        }
        _ => println!("Unknown command {}", input_char)
    }
    move_cursor(editor_config, &mut buffer);
    editor::update_position_in_status_line(editor_config, &mut buffer);
    editor_visual::flush_buffer(&mut buffer);
}