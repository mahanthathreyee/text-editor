use std::io::{Write, stdout};

use constants::ANSI_COLOR_RESET;

use crate::{constants, editor_config, editor_cursor};

pub fn flush_buffer(buffer: &mut String) {
    print!("{}", buffer);
    stdout().flush().unwrap();
    buffer.clear();
}

pub fn clear_screen(buffer: &mut String) {
    buffer.push_str(&constants::ANSI_SECONDARY_SCREEN_OPEN);
}

pub fn editor_refresh_screen(editor_config: &editor_config::EditorConfig) {
    let mut buffer: String = String::new();
    buffer.push_str(&constants::ANSI_SECONDARY_SCREEN_OPEN);
    buffer.push_str(&constants::ANSI_HIDE_CURSOR);
    editor_draw_rows(editor_config, &mut buffer);
    buffer.push_str("\x1b[H");
    buffer.push_str(constants::ANSI_SHOW_CURSOR);
    flush_buffer(&mut buffer);
}

pub fn editor_draw_rows(editor_config: &editor_config::EditorConfig, buffer: &mut String) {
    for index in 0..(editor_config.rows - 1) {
        if index == editor_config.rows / 3 {
            welcome_message(editor_config, buffer);
            buffer.push_str("\r\n");
        } else if index < editor_config.rows - 2 {
            buffer.push_str(&format!("{}~{}", constants::ANSI_BACKGROUND_COLOR_BLUE, constants::ANSI_COLOR_RESET));
            buffer.push_str("\r\n");
        } else if index == editor_config.rows - 2 {
            draw_status_line(editor_config, buffer);
        }
    }
}

fn welcome_message(editor_config: &editor_config::EditorConfig, buffer: &mut String) {
    let welcome = format!("My Rust Editor -- v{}", constants::EDITOR_VERSION);
    buffer.push_str("\x1b[34m~\x1b[0m");
    
    let padding = (editor_config.cols - (welcome.len() as u16)) / 2;
    for _ in 0..padding {
        buffer.push_str(" ");
    }

    buffer.push_str(&welcome);
}

pub fn error_and_exit(message: String) {
    let mut buffer: String = String::new();
    clear_screen(&mut buffer);
    flush_buffer(&mut buffer);
    if ! message.is_empty() {
        panic!(message);
    }
}

fn draw_status_line(editor_config: &editor_config::EditorConfig, buffer: &mut String) {
    for index in 0..editor_config.cols {
        if index < constants::EDITOR_STATUS_LINE_MODE_WIDTH || index > (editor_config.cols - 1) - constants::EDITOR_STATUS_LINE_MODE_WIDTH {
            buffer.push_str(&format!("{} {}", constants::ANSI_BACKGROUND_COLOR_GREEN, ANSI_COLOR_RESET));
        } else {
            buffer.push_str(&format!("{} {}", constants::ANSI_BACKGROUND_COLOR_LIGHT_BLACK, ANSI_COLOR_RESET));
        }
    }
}

pub fn clear_command_line(editor_config: &editor_config::EditorConfig, buffer: &mut String) {
    editor_cursor::save_cursor_and_move_position(0, editor_config.rows, buffer, true);
    buffer.push_str(constants::ANSI_CLEAR_ENTIRE_LINE);
    editor_cursor::restore_cursor(buffer);
}