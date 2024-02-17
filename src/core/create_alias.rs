use colorful::Color;
use colorful::Colorful;
use std::{fs::OpenOptions, io::Write};

use crate::utils::create_alias_command::create_alias_command;
use crate::utils::{get_source_path::get_source_path, print_alias::print_alias};
use crate::Alias;

pub fn create_alias(alias: &Alias) {
    let alias_command = create_alias_command(&alias);

    let total_path = get_source_path();
    println!("{}", total_path);

    // Opens the Config file in append mode
    match OpenOptions::new().append(true).open(total_path) {
        // Write Alias to file
        Ok(mut file) => match file.write_all(alias_command.as_bytes()) {
            Ok(_) => {
                print_alias(Alias {
                    alias: alias.to_string(),
                    command: alias.command.to_string(),
                });

                let source_alias = Alias {
                    alias: "so".to_owned(),
                    command: get_source_command(),
                };

                let alias_command = create_alias_command(&source_alias);

                let create_source_config_alias = file.write_all(alias_command.as_bytes());

                match create_source_config_alias {
                    Ok(_) => {
                        print_alias(source_alias);

                        let msg = format!(
                            "type {} to continue and use aliases",
                            "so".color(Color::Green)
                        );

                        println!("{msg}");
                    }
                    Err(err) => {
                        let err_msg =
                            "Failed to write alias to configuration file:".color(Color::Red);
                        eprintln!("{}{}", err_msg, err);
                    }
                }
            }
            Err(e) => {
                let err_msg = "Failed to write alias to configuration file:".color(Color::Red);
                eprintln!("{}{}", err_msg, e);
            }
        },
        Err(_) => {
            let err_msg = "Failed to open configuration file for writing".color(Color::Red);
            eprintln!("{err_msg}");
        }
    }
}

fn get_source_command() -> String {
    let source_path = get_source_path();
    let source_command = format!("source {source_path}");
    return source_command;
}
