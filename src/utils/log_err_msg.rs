use colorful::{Color, Colorful};

use crate::{utils::useful_utils::cancel_icon, LogErrorMsg};

/// Creates and logs an error message.
///
/// This function takes a [`LogErrorMsg`] struct containing a message and an error string,
/// formats them into a colored error message, prints the message to the standard error stream,
/// and returns the formatted error message as a [`String`].
///
/// # Arguments
///
/// * `err_msg` - A [`LogErrorMsg`] struct containing the error message and error string.
///
/// # Returns
///
/// A [`String`] containing the formatted error message.
///
/// # Examples
///
/// ```
/// use crate::{create_error_msg, LogErrorMsg};
///
/// let err_msg = LogErrorMsg {
///     msg: "Failed to write config".to_owned(),
///     err: "Permission denied".to_owned(),
/// };
///
/// let formatted_error = create_error_msg(err_msg);
/// println!("Formatted error message: {}", formatted_error);
/// ```
///
pub fn create_error_msg(err_msg: LogErrorMsg) -> colorful::core::color_string::CString {
    let icon_cancel = &cancel_icon();
    let err = err_msg.err;
    let msg = err_msg.msg;

    let fail_msg = format!("{icon_cancel} {msg} {err}")
        .color(Color::Red)
        .bold();
    eprintln!("{fail_msg}");
    return fail_msg;
}
