pub mod core;
pub mod utils;

use core::create_alias::create_alias;
use std::fmt;

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

#[derive(Clone)]
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
    match get_workspace() {
        Ok(original_alias) => {
            create_alias(&original_alias);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

/*
 * We need to know the workspace we're on and make alias based on those
 * */
