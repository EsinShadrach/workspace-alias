use std::env::{self};

use crate::{
    utils::{log_err_msg::create_error_msg, useful_utils::check_mark},
    LogErrorMsg, WorkspaceError,
};

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
