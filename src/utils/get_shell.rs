use std::env::{self};

use crate::{
    utils::{log_err_msg::create_error_msg, useful_utils::check_mark},
    LogErrorMsg, WorkspaceError,
};
/// Retrieves the shell configuration file path based on the value of the `SHELL` environment variable.
///
/// This function attempts to retrieve the shell configuration file path based on the value of the `SHELL` environment variable.
/// If the `SHELL` environment variable is set and contains the string "zsh", it returns `".zshrc"`.
/// Otherwise, it returns `".bashrc"`.
///
/// If the `SHELL` environment variable is not set or if an error occurs while accessing it, an error of type [`WorkspaceError`] is returned.
///
/// # Errors
///
/// This function can return an error of type [`WorkspaceError`] if:
/// - The `SHELL` environment variable is not set or an error occurs while accessing it.
///
/// # Examples
///
/// ```
/// use crate::get_shell;
///
/// match get_shell() {
///     Ok(shell_path) => println!("Shell configuration file path: {}", shell_path),
///     Err(err) => eprintln!("Error: {:?}", err),
/// }
/// ```
///
pub fn get_shell() -> Result<String, WorkspaceError> {
    let shell = env::var("SHELL");
    match shell {
        Ok(shell) => {
            let mut shell_path = String::new();

            if shell.contains("zsh") {
                shell_path.push_str(".zshrc");
            } else {
                shell_path.push_str(".bashrc");
            }

            let check = &check_mark();
            println!("{} Found SHELL \"{shell}\" {shell_path}", { check });

            return Ok(shell_path);
        }
        Err(err) => {
            create_error_msg(LogErrorMsg {
                msg: "Failed to get `SHELL`".to_owned(),
                err: err.to_string(),
            });

            return Err(WorkspaceError::DirectoryCheckFailed);
        }
    }
}
