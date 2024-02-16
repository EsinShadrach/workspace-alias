use std::{env, fs::OpenOptions, io::Write, process::Command};

use crate::{
    utils::{convert_to_camel_case::to_camel_case, print_alias::print_alias},
    Alias,
};

pub fn create_alias(alias: &Alias) {
    let camel_cased = to_camel_case(alias.alias.clone());
    let alias_command = format!("alias {}={}", camel_cased, alias.command);
    let execute_alias_creation = Command::new("sh").arg("-c").arg(&alias_command).spawn();

    match execute_alias_creation {
        Ok(_) => {
            write_to_config(alias);
            print_alias(alias.clone());
        }
        Err(err) => {
            println!("Failed to Create {}", alias);
            println!("{err}")
        }
    }
}

fn write_to_config(alias: &Alias) {
    let command = &alias.command;
    let alias = &alias.alias;

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
            match OpenOptions::new().append(true).open(total_path) {
                Ok(mut file) => match file.write_all(alias_command.as_bytes()) {
                    Err(e) => {
                        eprintln!("Failed to write alias to configuration file: {}", e);
                    }
                    _ => (),
                },
                Err(_) => {
                    eprintln!("Failed to open configuration file for writing");
                }
            }
        }
        _ => println!("Failed go get HOME Environment"),
    }
}
