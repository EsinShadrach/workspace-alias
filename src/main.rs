pub mod core;
pub mod utils;

use std::fmt;

use utils::print_alias::print_alias;

use crate::core::get_works_space::get_workspace;

#[derive(Debug)]
pub enum WorkspaceError {
    CommandFailed,
    DirectoryCheckFailed,
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for WorkspaceError {}

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

    match get_workspace() {
        Ok(alias) => {
            println!("{}", alias);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

/*
 * We need to know the workspace we're on and make alias based on those
 * */
