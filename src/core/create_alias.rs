use std::process::Command;

use crate::{
    utils::{convert_to_camel_case::to_camel_case, print_alias::print_alias},
    Alias,
};

pub fn create_alias(alias: &Alias) {
    let camel_cased = to_camel_case(alias.alias.clone());
    let execute_alias_creation = Command::new("alias")
        .arg(format!("{}={}", camel_cased, alias.command))
        .spawn();

    match execute_alias_creation {
        Ok(_) => {
            print_alias(alias.clone());
        }
        Err(err) => {
            println!("Failed to Create {}", alias);
            println!("{err}")
        }
    }
}
