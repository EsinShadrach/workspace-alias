use colorful::Color;
use colorful::Colorful;
use std::{env, fs::OpenOptions, io::Write, process::Command};

use crate::{
    utils::{convert_to_camel_case::to_camel_case, print_alias::print_alias},
    Alias,
};

pub fn create_alias(alias: &Alias) {
    let camel_cased = to_camel_case(alias.alias.clone());
    // This won't work
    let alias_command = format!("alias {}={}", camel_cased, alias.command);
    // This won't work
    let execute_alias_creation = Command::new("sh").arg("-c").arg(&alias_command).spawn();

    match execute_alias_creation {
        Ok(_) => {
            write_to_config(alias);
        }
        Err(err) => {
            println!("Failed to Create {}", alias);
            println!("{err}")
        }
    }
}

fn write_to_config(alias: &Alias) {
    let command = &alias.command;
    let alias = to_camel_case(alias.alias.clone());

    let config_file_path = match env::var("SHELL") {
        Ok(s) if s.contains("zsh") => ".zshrc",
        Ok(s) if s.contains("bash") => ".bashrc",
        _ => {
            panic!("Unsupported SHELL type")
        }
    };

    let alias_command = format!("alias {}={}\n", alias, command);

    match env::var("HOME") {
        Ok(home_path) => {
            let total_path = format!("{home_path}/{config_file_path}");
            println!("{}", total_path);

            // Opens the Config file in append mode
            match OpenOptions::new().append(true).open(total_path) {
                // Write Alias to file
                Ok(mut file) => match file.write_all(alias_command.as_bytes()) {
                    Ok(_) => {
                        print_alias(Alias {
                            alias: alias.to_string(),
                            command: command.to_string(),
                        });
                    }
                    Err(e) => {
                        let err_msg =
                            "Failed to write alias to configuration file:".color(Color::Red);
                        eprintln!("{}{}", err_msg, e);
                    }
                },
                Err(_) => {
                    let err_msg = "Failed to open configuration file for writing".color(Color::Red);
                    eprintln!("{err_msg}");
                }
            }
        }
        _ => println!("Failed go get HOME Environment"),
    }
}
