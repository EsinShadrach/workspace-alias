pub mod core;
pub mod utils;

use core::create_config::create_config_file;
use std::fmt;

#[derive(Debug)]
pub enum WorkspaceError {
    CommandFailed,
    DirectoryCheckFailed,
    WriteFailed,
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for WorkspaceError {}

#[derive(Clone, Debug)]
pub struct Alias {
    alias: String,
    command: String,
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Alias: {}, Command: {}", self.alias, self.command)
    }
}
#[tokio::main]
async fn main() {
    create_config_file().await;
    // match get_workspace() {
    //     Ok(_original_alias) => {
    //         // println!("{:?}", original_alias);
    //         // create_alias(&original_alias);
    //         // create_config_file();
    //     }
    //     Err(err) => {
    //         eprintln!("Error: {}", err);
    //     }
    // }
}
