pub mod core;
pub mod utils;

use std::fmt;

use utils::convert_to_camel_case::to_camel_case;
use utils::print_alias::print_alias;

use crate::core::get_works_space::get_workspace;

pub struct Alias {
    alias: String,
    command: String,
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Alias: {}, Command: {}", self.alias, self.command)
    }
}

fn main() {
    let alias_name = String::from("l");
    let command_name = String::from("ls");

    print_alias(Alias {
        alias: alias_name,
        command: command_name,
    });
    let xx = to_camel_case("Hello world my name is rafe".to_string());
    println!("{}", xx);
    get_workspace();
}

/*
 * We need to know the workspace we're on and make alias based on those
 * */
