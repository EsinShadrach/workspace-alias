use colorful::Color;
use colorful::Colorful;
use std::{fs::OpenOptions, io::Write};

use crate::utils::create_alias_command::create_alias_command;
use crate::utils::{get_source_path::get_source_path, print_alias::print_alias};
use crate::Alias;

pub fn create_alias(alias: &Alias) {
    let alias_command = create_alias_command(&alias);

    let total_path = get_source_path();

    // Opens the Config file in append mode
    match OpenOptions::new().append(true).open(total_path) {
        // Write Alias to file
        Ok(mut file) => match file.write_all(alias_command.as_bytes()) {
            Ok(_) => {
                print_alias(Alias {
                    alias: alias.to_string(),
                    command: alias.command.to_string(),
                });

                // `so` command won't be available unless it's added at installation, we need to
                // check for that and maybe run a pbcopy of so command
                // Get os type and show command  like CMD + v or ctrl + shift + v
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
