pub fn center_text(text: String, width: usize) -> String {
    let space = " ".repeat((width - text.len()) / 2);
    let odd_width_paddind = if (width - text.len()) % 2 == 0 { "" } else { " " };
    format!("{}{}{}{}", odd_width_paddind, space, text, space)
}