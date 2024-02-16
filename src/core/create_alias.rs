use std::{env, process::Command};

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
            write_to_config();
            print_alias(alias.clone());
        }
        Err(err) => {
            println!("Failed to Create {}", alias);
            println!("{err}")
        }
    }
}

fn write_to_config() {
    let config = match env::var("SHELL") {
        Ok(s) if s.contains("zsh") => "~/.zshrc",
        Ok(s) if s.contains("bash") => "~/.bashrc",
        _ => {
            panic!("Un-Supported SHELL type")
        }
    };
}
