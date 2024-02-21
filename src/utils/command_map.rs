use colorful::Color;
use colorful::Colorful;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{collections::HashMap, fs::File};

use crate::utils::print_alias::print_alias;
use crate::Alias;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    languages: HashMap<String, LanguageConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LanguageConfig {
    name: String,
    // the r#type means raw string
    r#type: String,
    file_match: Vec<String>,
    commands: Vec<String>,
    command_alias: HashMap<String, String>,
}

pub fn workspace_command(command: &str) {
    let file = File::open("./config.json");
    let mut config_string = String::new();

    match file {
        Ok(mut opened_file) => {
            let data = opened_file.read_to_string(&mut config_string);
            match data {
                Ok(_) => {
                    let config = serde_json::from_str::<Config>(&config_string);
                    match config {
                        Ok(success_config) => {
                            match success_config.languages.get(command) {
                                Some(workspace_config) => {
                                    let workspace_alias = &workspace_config.command_alias;
                                    for (alias, command) in workspace_alias {
                                        print_alias(Alias {
                                            alias: alias.to_owned(),
                                            command: command.to_owned(),
                                        })
                                    }
                                }
                                None => {
                                    println!("No configuration found for workspace: {}", command);
                                }
                            }
                            // return alias for workspace config
                        }
                        Err(err) => {
                            let msg = "Error While parsing config file".color(Color::Red);
                            eprintln!("{msg}");
                            eprintln!("{err}");
                        }
                    }
                }
                Err(err) => {
                    let msg = "Error While opening config file".color(Color::Red);
                    eprintln!("{msg}");
                    eprintln!("{err}");
                }
            }
        }
        Err(err) => {
            let msg = "Error While opening config file".color(Color::Red);
            eprintln!("{msg}");
            eprintln!("{err}");
        }
    }
}
