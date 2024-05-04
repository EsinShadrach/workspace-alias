use colorful::{Color, Colorful};

use crate::{utils::useful_utils::cancel_icon, LogErrorMsg};

/// I ended up repeating this pattern of logging errors out
/// so i just thought, that maybe i should just log it out instead
/// Takes in struct `LogErrorMsg`
/// outputs `cancel_icon {msg} {err}`
/// # Example
/// ```rust
/// create_error_msg(LogErrorMsg {
///    msg: "Your Error Msg here".to_owned(),
///    err: err.to_string(),
///});
/// ```
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
