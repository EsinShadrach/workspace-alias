pub mod core;
pub mod utils;

use utils::convert_to_camel_case::to_camel_case;
use utils::print_alias::print_alias;

use crate::core::get_works_space::get_workspace;

pub struct Alias {
    alias: String,
    command: String,
}

fn main() {
    let alias_name = String::from("l");
    let command_name = String::from("ls");

    print_alias(Alias {
        alias: alias_name,
        command: command_name,
    });
    get_workspace();
}

/*
 * We need to know the workspace we're on
 * Create alias for it and output `alias x created for command x`
 * */
