use std::{
    env::{self},
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::{
    core::{config_operations::config_operation, create_alias_in_shell::create_alias_in_shell},
    utils::{get_shell::get_shell, log_err_msg::create_error_msg, useful_utils::check_mark},
    LogErrorMsg,
};

pub async fn create_config_file() {
    match env::var("HOME") {
        Ok(home_path) => {
            let check = &check_mark();
            println!("{} Found Home Path  \"{home_path}/\"", check);

            match get_shell() {
                Ok(shell) => {
                    // Construct path to shell
                    let config_path = format!("{home_path}/{shell}");
                    // Reassign config_path to a path instead of a string
                    let config_path = Path::new(&config_path);

                    let file = match File::open(config_path) {
                        Ok(file) => file,
                        Err(err) => {
                            create_error_msg(LogErrorMsg {
                                msg: "Error opening file:".to_owned(),
                                err: err.to_string(),
                            });
                            return;
                        }
                    };

                    let mut shell_config_file = String::new();
                    let mut reader = BufReader::new(file);
                    let read_file_string = reader.read_to_string(&mut shell_config_file);
                    let alias_config_path = format!("{home_path}/alias-thing");
                    config_operation(Path::new(&alias_config_path)).await;

                    match read_file_string {
                        Ok(_) => {
                            let has_alias: Vec<&str> = shell_config_file
                                .split("\n")
                                .filter(|x| x.contains("alias alias-thing"))
                                .collect();

                            if has_alias.is_empty() {
                                create_error_msg(LogErrorMsg {
                                    msg: "No Alias for alias-thing found".to_owned(),
                                    err: String::new(),
                                });
                                create_alias_in_shell(&config_path, alias_config_path);
                            } else {
                                println!("{} Alias for alias-thing found", check);
                            }
                        }
                        Err(err) => {
                            create_error_msg(LogErrorMsg {
                                msg: "Failed to Read `SHELL`".to_owned(),
                                err: err.to_string(),
                            });
                            return;
                        }
                    }
                }
                Err(err) => {
                    create_error_msg(LogErrorMsg {
                        msg: "Failed to get `SHELL`".to_owned(),
                        err: err.to_string(),
                    });
                }
            }
        }
        Err(err) => {
            create_error_msg(LogErrorMsg {
                msg: "Failed to get `HOME` path".to_owned(),
                err: err.to_string(),
            });
        }
    }
}
