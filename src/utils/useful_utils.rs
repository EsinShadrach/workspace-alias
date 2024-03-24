use colorful::{Color, Colorful};

pub fn check_mark() -> String {
    return "\u{2714}".color(Color::Green).bold().to_string();
}

pub fn cancel_icon() -> String {
    let cancel_icon: String = "\u{2715}".color(Color::Red).to_string();
    return cancel_icon;
}
