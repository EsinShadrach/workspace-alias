use colorful::{Color, Colorful};

use std::{
    env::{self},
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::{
    core::{config_operations::config_operation, create_alias_in_shell::create_alias_in_shell},
    utils::{
        get_shell::get_shell,
        useful_utils::{cancel_icon, check_mark},
    },
};

pub async fn create_config_file() {
    let icon_cancel = &cancel_icon();

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
                            eprintln!("{} Error opening file: {}", icon_cancel, err);
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
                                let fail_msg =
                                    format!("{icon_cancel} No Alias for alias-thing found")
                                        .color(Color::Red)
                                        .bold();
                                println!("{fail_msg}");
                                create_alias_in_shell(&config_path);
                            } else {
                                println!("{} Alias for alias-thing found", check);
                            }
                        }
                        Err(err) => {
                            let fail_msg = format!("{icon_cancel} Failed to Read `SHELL` {err}")
                                .color(Color::Red)
                                .bold();
                            eprintln!("{fail_msg}");
                            return;
                        }
                    }
                }
                Err(err) => {
                    let fail_msg = format!("{icon_cancel} Failed to get `SHELL` {err}")
                        .color(Color::Red)
                        .bold();
                    eprintln!("{fail_msg}");
                }
            }
        }
        Err(err) => {
            let fail_msg = format!("Failed to get `HOME` path {err}")
                .color(Color::Red)
                .bold();
            eprintln!("{fail_msg}")
        }
    }
}
