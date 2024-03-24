use colorful::{Color, Colorful};
use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufReader, Read, Write},
    path::Path,
};

use crate::{
    utils::useful_utils::{cancel_icon, check_mark},
    WorkspaceError,
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
                            eprintln!("{} Error opening file: {}", cancel_icon(), err);
                            return;
                        }
                    };

                    let mut shell_config_file = String::new();
                    let mut reader = BufReader::new(file);
                    let read_file_string = reader.read_to_string(&mut shell_config_file);

                    match read_file_string {
                        Ok(_) => {
                            let has_alias: Vec<&str> = shell_config_file
                                .split("\n")
                                .filter(|x| x.contains("alias alias-thing"))
                                .collect();

                            if has_alias.is_empty() {
                                let icon_cancel = cancel_icon();
                                let fail_msg =
                                    format!("{icon_cancel} No Alias for alias-thing found")
                                        .color(Color::Red)
                                        .bold();
                                println!("{fail_msg}");
                                create_alias_in_shell(&config_path);
                            } else {
                                let check = &check_mark();
                                println!("{} Alias for alias-thing found", check);
                            }
                        }
                        Err(err) => {
                            let icon_cancel = cancel_icon();
                            let fail_msg = format!("{icon_cancel} Failed to Read `SHELL` {err}")
                                .color(Color::Red)
                                .bold();
                            eprintln!("{fail_msg}");
                            return;
                        }
                    }
                }
                Err(err) => {
                    let icon_cancel = cancel_icon();
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

fn get_shell() -> Result<String, WorkspaceError> {
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

fn create_alias_in_shell(shell_path: &Path) {
    let alias = "source ~/alias-config/.workspace_alias && call_binary_here";
    let alias_line = format!("alias alias-thing = \"{alias}\"");

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
                    let icon_cancel = cancel_icon();
                    let fail_msg = format!(
                        "{icon_cancel} Failed to add alias to {:?} {err}",
                        Some(shell_path).expect("failed to get path")
                    )
                    .color(Color::Red)
                    .bold();
                    eprintln!("{fail_msg}");
                }
            }
        }
        Err(err) => {
            let icon_cancel = cancel_icon();
            let fail_msg = format!(
                "{icon_cancel} Failed to open to add alias {:?} {err}",
                Some(shell_path).expect("failed to get path")
            )
            .color(Color::Red)
            .bold();
            eprintln!("{fail_msg}");
        }
    }
}

/*
 * - Read ZSH config
 * if command alias-thing doesn't exist and folder alias-thing doesn't exist
 * - Create alias-thing folder, pull json config
 * - add "alias-thing --init" to so and call binary to get workspace configurations
 * */

// let home_path = match env::var("HOME") {
//     Ok(path) => path,
//     Err(err) => {
//         let msg = "Failed to get home path".color(Color::Red);
//         eprintln!("{msg}, {err}");
//         return;
//     }
// };

// let config_folder = format!("{}/alias-config", home_path);
// let create_dir_fn = fs::create_dir(&config_folder);
// let config_path = Path::new(&config_folder);

// if !config_path.exists() {
//     match create_dir_fn {
//         Ok(_) => {
//             let msg = "Created Config Folder Succeffully".color(Color::Green);
//             println!("{msg}");
//         }
//         Err(err) => {
//             let msg = "Failed to get home path".color(Color::Red);
//             eprintln!("{msg}, {err}");
//         }
//     }
// } else {
//     let msg = "Config Path Already Exists".color(Color::Yellow);
//     println!("{msg}");
// }

// // Create config.json
// let config_json = config_path.join("config.json");
// let alias_config_file = config_path.join(".workspace_alias");

// if config_json.exists() {
//     let msg = "config.json Already Exists".color(Color::Yellow);
//     println!("{msg}");
// } else {
//     let mut json_file = match File::create(&config_json) {
//         Ok(file_to_write) => file_to_write,
//         Err(err) => {
//             let msg = "Failed to create json".color(Color::Red);
//             eprintln!("{msg}, {err}");
//             return;
//         }
//     };

//     let val = async {
//         let config_req = reqwest::get(
//             "https://raw.githubusercontent.com/EsinShadrach/workspace-alias/main/config.json",
//         )
//         .await;

//         match config_req {
//             Ok(okay) => {
//                 let json_text = okay.text().await.expect("Failed to read okay text");
//                 return json_text;
//             }
//             Err(err) => {
//                 let msg = "Failed to Write config.json".color(Color::Red);
//                 let err_msg = format!("{msg}, {err}");
//                 panic!("{err_msg}");
//             }
//         }
//     }
//     .await;

//     match json_file.write_all(val.as_bytes()) {
//         Ok(_) => {
//             let msg = format!(
//                 "Created {} {}",
//                 "config.json".bold(),
//                 "Succeffully".color(Color::Green)
//             )
//             .color(Color::Green);
//             println!("{msg}");
//         }
//         Err(err) => {
//             let msg = "Failed to Write config.json".color(Color::Red);
//             eprintln!("{msg}, {err}");
//         }
//     }
// }

// if alias_config_file.exists() {
//     let msg = ".workspace_alias Already Exists".color(Color::Yellow);
//     println!("{msg}");
//     return;
// }

// match File::create(&alias_config_file) {
//     Ok(mut x) => {
//         let fail_msg = format!(
//             "{}",
//             "Failed to write Workspace_alias to disk".color(Color::Red)
//         );
//         let success_msg = format!("Written .workspace_alias to disk").color(Color::Green);

//         println!("{success_msg}");
//         x.write(b"apples are red").expect(&fail_msg);
//     }
//     Err(err) => {
//         let msg = "Failed to Write alias file".color(Color::Red);
//         eprintln!("{msg}, {err}");
//     }
// };
