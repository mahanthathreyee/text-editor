#[allow(warnings)]

#[path ="./constants.rs"]
mod constants;

#[path ="./terminal_utility.rs"]
mod terminal_utility;

#[path ="./input_utility.rs"]
mod input_utility;

#[path ="./editor_visual.rs"]
mod editor_visual;

#[path ="./editor_config.rs"]
mod editor_config;

#[path ="./editor_cursor.rs"]
mod editor_cursor;

#[path ="./editor.rs"]
mod editor;

#[path ="./utility.rs"]
mod utility;

fn main() {
    let mut editor_config = editor_config::EditorConfig::new();
    terminal_utility::enable_raw_mode(&editor_config);
    
    editor::init_editor(&mut editor_config);
    input_utility::editor_process_key(&mut editor_config);

    terminal_utility::disable_raw_mode(&editor_config);
}