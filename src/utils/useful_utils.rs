use colorful::{Color, Colorful};

/// Returns a check mark icon as a colored and bold string.
///
/// This function returns a check mark icon (✓) as a colored and bold string,
/// indicating a successful operation or completion.
///
/// # Returns
///
/// A `String` containing the check mark icon as a colored and bold string.
///
/// # Examples
///
/// ```
/// use crate::check_mark;
///
/// let check_mark = check_mark();
/// println!("Check mark: {}", check_mark);
/// ```
///
pub fn check_mark() -> String {
    return "\u{2714}".color(Color::Green).bold().to_string();
}

/// Returns a cancel icon as a colored string.
///
/// This function returns a cancel icon (✕) as a colored string,
/// indicating a failed operation or cancellation.
///
/// # Returns
///
/// A `String` containing the cancel icon as a colored string.
///
/// # Examples
///
/// ```
/// use crate::cancel_icon;
///
/// let cancel_icon = cancel_icon();
/// println!("Cancel icon: {}", cancel_icon);
/// ```
///
pub fn cancel_icon() -> String {
    let cancel_icon: String = "\u{2715}".color(Color::Red).to_string();
    return cancel_icon;
}
