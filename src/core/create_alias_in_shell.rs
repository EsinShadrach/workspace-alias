use std::{fs::OpenOptions, io::Write, path::Path};

use crate::{
    utils::{log_err_msg::create_error_msg, useful_utils::check_mark},
    LogErrorMsg,
};

pub fn create_alias_in_shell(shell_path: &Path, alias_config_path: String) {
    // Get alias-thing directory
    // make it the path for write
    let alias = format!("source {alias_config_path}/.workspace-alias");
    let alias_line = format!("alias alias-thing=\"{alias}\"");

    let file = OpenOptions::new().write(true).append(true).open(shell_path);
    match file {
        Ok(mut xr) => {
            let write_res = xr.write_all(alias_line.as_bytes());
            match write_res {
                Ok(_) => {
                    let check = &check_mark();
                    println!("{} alias alias-thing written", check);
                }
                Err(err) => {
                    create_error_msg(LogErrorMsg {
                        msg: "Failed to add alias to".to_owned(),
                        err: err.to_string(),
                    });
                }
            }
        }
        Err(err) => {
            create_error_msg(LogErrorMsg {
                msg: "Failed to open to add alias".to_owned(),
                err: err.to_string(),
            });
        }
    }
}
