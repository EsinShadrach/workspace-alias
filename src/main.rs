pub mod core;
pub mod utils;

use crate::core::get_works_space::get_workspace;
use utils::print_alias::print_alias;

fn main() {
    let alias_name = String::from("l");
    let command_name = String::from("ls");

    print_alias(alias_name, command_name);
    get_workspace();
}

/*
 * We need to know the workspace we're on
 * Create alias for it and output `alias x created for command x`
 * */
