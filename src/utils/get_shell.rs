use colorful::{Color, Colorful};

use std::env::{self};

use crate::{
    utils::useful_utils::{cancel_icon, check_mark},
    WorkspaceError,
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
            let icon_cancel = cancel_icon();
            let fail_msg = format!("{icon_cancel} Failed to get `SHELL` {err}")
                .color(Color::Red)
                .bold();
            eprintln!("{fail_msg}");

            return Err(WorkspaceError::DirectoryCheckFailed);
        }
    }
}
