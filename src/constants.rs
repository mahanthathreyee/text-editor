pub const EDITOR_VERSION: &str = "1.0.0";
pub const INPUT_BUFFER_SIZE: usize = 1;

//MESSAGES
pub const MESSAGE_ERROR_INVALID_UTF8: &str = "Invalid UTF-8 sequence";
pub const MESSAGE_ERROR_SETTING_TERMINAL_CONFIG: &str = "Couldn't instantiate editor";
pub const MESSAGE_ERROR_DIMENSION_INTEGER_PARSE: &str = "Dimension integer parse error";

//KEYBOARD KEYBINDINGS
pub const TERMINAL_ESCAPE: &str = "\x1b";
pub const QUIT: &str = "q";

//CURSOR MOVEMENTS
pub const MOVE_CURSOR_UP: &str = "w";
pub const MOVE_CURSOR_DOWN: &str = "s";
pub const MOVE_CURSOR_RIGHT: &str = "d";
pub const MOVE_CURSOR_LEFT: &str = "a";

//ARROW KEYS
pub const ARROW_UP: &str = "A";
pub const ARROW_DOWN: &str = "B";
pub const ARROW_RIGHT: &str = "C";
pub const ARROW_LEFT: &str = "D";

//ANSI CODES
pub const ANSI_SECONDARY_SCREEN_OPEN: &str = "\x1b[?1049h";
pub const ANSI_SECONDARY_SCREEN_CLOSE: &str = "\x1b[?1049l";
pub const ANSI_SHOW_CURSOR: &str = "\x1b[?25h";
pub const ANSI_HIDE_CURSOR: &str = "\x1b[?25l";

//ANSI BACKGROUND COLOR CODES
pub const ANSI_COLOR_RESET: &str = "\x1b[0m";
pub const ANSI_BACKGROUND_COLOR_LIGHT_BLACK: &str = "\x1b[40;1m";
pub const ANSI_BACKGROUND_COLOR_GREEN: &str = "\x1b[48;5;154m";
pub const ANSI_BACKGROUND_COLOR_BLUE: &str = "\x1b[34m";
//ANSI TEXT COLOR CODES
pub const ANSI_TEXT_COLOR_BLACK: &str = "\x1b[38;5;0m";

//EDITOR
pub const EDITOR_NUMBER_LINE_INDEX: u16 = 3;
pub const EDITOR_STATUS_LINE_MODE_WIDTH: u16 = 10;

pub enum EDITOR_MODES {
    NORMAL,
    EDIT
}

impl EDITOR_MODES {
    pub fn name(&self) -> &str {
        match *self {
            EDITOR_MODES::NORMAL => "NORMAL",
            EDITOR_MODES::EDIT => "EDIT"
        }
    }
}