use constants::EDITOR_MODES;

use crate::{constants, editor_config, editor_cursor, editor_visual, terminal_utility, input_utility, utility, input_process};

pub fn init_editor(editor_config: &mut editor_config::EditorConfig) {
    terminal_utility::get_window_size(editor_config);
    editor_visual::editor_refresh_screen(editor_config);
    editor_config.lines.push(editor_config::EditorLine::new("".to_string()));
    write_line(editor_config, &mut String::new(), 0);
    
    let mut buffer = String::new();
    update_mode_in_status_line(editor_config, &mut buffer, &EDITOR_MODES::INSERT);
    update_position_in_status_line(editor_config, &mut buffer);
    write_line(editor_config, &mut buffer, 0);
}

pub fn write_line(editor_config: &mut editor_config::EditorConfig, buffer: &mut String, line_index: usize) {
    if line_index < editor_config.lines.len() {
        let line = editor_config.lines[line_index].clone();
        editor_cursor::move_cursor_to_position(0, line_index as u16, editor_config, buffer);
        
        let line = format!("  {} {}", line_index + 1, line.content);
        buffer.push_str(&line);

        editor_cursor::move_cursor_to_position((line.len() + 1) as u16, editor_config.cursor_y, editor_config, buffer);
        editor_visual::flush_buffer(buffer);
    }
}   

pub fn update_mode_in_status_line(editor_config: &mut editor_config::EditorConfig, buffer: &mut String, editor_mode: &constants::EDITOR_MODES) {
    let initial_cursor: (u16, u16) = (editor_config.cursor_x, editor_config.cursor_y);

    editor_cursor::move_cursor_to_position(1, editor_config.rows - 1, editor_config, buffer);
    let mode_text = utility::center_text(editor_mode.name().to_string(), constants::EDITOR_STATUS_LINE_MODE_WIDTH as usize);
    buffer.push_str(&format!("{}{}{}{}", constants::ANSI_BACKGROUND_COLOR_GREEN, constants::ANSI_TEXT_COLOR_BLACK, &mode_text, constants::ANSI_COLOR_RESET));
    editor_cursor::move_cursor_to_position(initial_cursor.0, initial_cursor.1, editor_config, buffer);
}

pub fn update_position_in_status_line(editor_config: &mut editor_config::EditorConfig, buffer: &mut String) {
    let initial_cursor: (u16, u16) = (editor_config.cursor_x, editor_config.cursor_y);

    editor_cursor::move_cursor_to_position((editor_config.cols + 1) - constants::EDITOR_STATUS_LINE_MODE_WIDTH, editor_config.rows - 1, editor_config, buffer);
    let mode_text = utility::center_text(format!("{}:{}", initial_cursor.1, initial_cursor.0 - (constants::EDITOR_NUMBER_LINE_INDEX + 2)), constants::EDITOR_STATUS_LINE_MODE_WIDTH as usize);
    buffer.push_str(&format!("{}{}{}{}", constants::ANSI_BACKGROUND_COLOR_GREEN, constants::ANSI_TEXT_COLOR_BLACK, &mode_text, constants::ANSI_COLOR_RESET));
    editor_cursor::move_cursor_to_position(initial_cursor.0, initial_cursor.1, editor_config, buffer);
}

pub fn editor_process_key(editor_config: &mut editor_config::EditorConfig) {    
    loop {
        let input = input_utility::editor_read_key();
        if let Some(input_char) = input {
            let input_char = input_utility::preprocess_characters(input_char);
            match input_char.as_str() {
                constants::COMMAND_START => {
                    if input_process::command_matcher(&input_char, editor_config) {
                        break;
                    }
                }
                constants::MOVE_CURSOR_UP | constants::MOVE_CURSOR_LEFT | constants::MOVE_CURSOR_DOWN | constants::MOVE_CURSOR_RIGHT => {
                    editor_cursor::editor_scroll(editor_config, input_char, 1);
                }
                _ => {}
            }
        }
    }
}
