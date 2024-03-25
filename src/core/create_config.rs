use colorful::{Color, Colorful};
use reqwest::Error;

use std::{
    env::{self},
    fs::{self, File, OpenOptions},
    io::{BufReader, Read, Write},
    path::Path,
};

use crate::{
    core::create_lang_map::create_lang_map,
    utils::useful_utils::{cancel_icon, check_mark},
    WorkspaceError,
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
    // Get alias-thing directory
    // make it the path for write
    let alias = "source ~/alias-config/.workspace_alias && call_binary_here";
    let alias_line = format!("alias alias-thing = \"{alias}\"");
    let icon_cancel = cancel_icon();

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

async fn config_operation(alias_config_path: &Path) {
    let check = &check_mark();
    let icon_cancel = cancel_icon();
    let aliases_store = alias_config_path.join(".workspace-alias");
    let config_json = alias_config_path.join("config.json");

    if !alias_config_path.exists() {
        match fs::create_dir(alias_config_path) {
            Ok(_) => {
                println!("{} alias-thing directory created", check);
            }
            Err(err) => {
                let icon_cancel = cancel_icon();
                let fail_msg = format!(
                    "{icon_cancel} Failed to Create Directory to add alias {:?} {err}",
                    Some(alias_config_path).expect("failed to get path")
                )
                .color(Color::Red)
                .bold();
                eprintln!("{fail_msg}");
            }
        }
    }

    if aliases_store.exists() {
        let exclamation_mark = "!".color(Color::Yellow);
        let msg = ".workspace_alias Already Exists";
        println!("{exclamation_mark} {msg}");
    } else {
        println!("{} no .workspace_alias file found", icon_cancel);
        match File::create(&aliases_store) {
            Ok(mut xr) => {
                match xr.write(b"# Workspace Aliases") {
                    Ok(_) => {
                        let msg = format!("{check} Written .workspace_alias");
                        println!("{msg}");
                    }
                    Err(err) => {
                        let msg = format!("{icon_cancel} Failed to create .workspace_alias")
                            .color(Color::Red);
                        println!("{msg}");

                        eprintln!("{err}")
                    }
                };
            }
            Err(err) => {
                let msg =
                    format!("{icon_cancel} Failed to create .workspace_alias").color(Color::Red);
                println!("{msg}");

                eprintln!("{err}")
            }
        }
    }

    if config_json.exists() {
        let exclamation_mark = "!".color(Color::Yellow);
        let msg = "config.json Already Exists";
        println!("{exclamation_mark} {msg}");
    } else {
        let config_json_response = match fetch_config().await {
            Ok(json) => json,
            Err(err) => {
                let msg = format!("{icon_cancel} Failed to Fetch config.json").color(Color::Red);
                println!("{msg}");
                eprintln!("{err}");
                return;
            }
        };
        let mut json_file = match File::create(&config_json) {
            Ok(file_to_write) => file_to_write,
            Err(err) => {
                let msg = "Failed to create json".color(Color::Red);
                eprintln!("{msg}, {err}");
                return;
            }
        };

        match json_file.write_all(config_json_response.as_bytes()) {
            Ok(_) => {
                let msg = format!(
                    "{} Created {} {}",
                    check_mark(),
                    "config.json".bold(),
                    "Succeffully".color(Color::Green)
                )
                .color(Color::Green);
                println!("{msg}");
            }
            Err(err) => {
                let msg = "Failed to Write config.json".color(Color::Red);
                eprintln!("{msg}, {err}");
            }
        }
    }
    determine_workspace(&aliases_store, &config_json);
}

async fn fetch_config() -> Result<String, Error> {
    let config_req = reqwest::get(
        "https://raw.githubusercontent.com/EsinShadrach/workspace-alias/main/config.json",
    )
    .await?;

    Ok(config_req.text().await?)
}

fn determine_workspace(workspace_config: &Path, config_json: &Path) {
    let config_lang = String::from("javascript");
    let _ = workspace_config;
    let icon_cancel = cancel_icon();

    let current_dir = match env::current_dir() {
        Ok(c_dir) => {
            let check = &check_mark();
            println!("{} Reading Current Directory", check);
            c_dir
        }
        Err(err) => {
            let fail_msg = format!("{icon_cancel} Failed to get `SHELL` {err}")
                .color(Color::Red)
                .bold();
            eprintln!("{fail_msg}");
            return;
        }
    };

    let entries = match fs::read_dir(&current_dir) {
        Ok(data) => {
            let check = &check_mark();
            println!("{} Reading Files in current directory", check);
            data
        }
        Err(err) => {
            let fail_msg = format!("{icon_cancel} Failed to get `SHELL` {err}")
                .color(Color::Red)
                .bold();
            eprintln!("{fail_msg}");
            return;
        }
    };

    for entry in entries.into_iter() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            println!("File: {}", path.display());
        }

        if path.is_dir() {
            println!("Directory: {}", path.display());
        }
    }

    println!(
        "\n{} Config Map for {} {}\n",
        "=".repeat(6),
        config_lang.to_uppercase(),
        "=".repeat(10)
    );

    let config_json_str = match File::open(config_json) {
        Ok(mut file_data) => {
            let mut data = String::new();
            let failure_to_read = format!("{icon_cancel} Failed to read JSON config");
            file_data.read_to_string(&mut data).expect(&failure_to_read);
            data
        }
        Err(err) => {
            let msg = "failed to open config file";
            println!("{} {} {}", icon_cancel, msg, err);
            return;
        }
    };

    let language_map = create_lang_map(&config_json_str);
    let language_map = match language_map {
        Ok(l_map) => l_map,
        Err(err) => {
            let msg = "Failed to open config file";
            println!("{} {} {}", icon_cancel, msg, err);
            return;
        }
    };

    let language = language_map.get(&config_lang);

    let language = match language {
        Some(lang) => lang,
        None => {
            let msg = format!(
                "Language Config not found, try adding it in {}",
                config_json.display()
            );
            println!("{} {}", icon_cancel, msg);
            return;
        }
    };

    println!("{:?}", language.command_alias);
}
