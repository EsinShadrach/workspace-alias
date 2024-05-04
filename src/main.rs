pub mod core;
pub mod utils;

use core::create_config::create_config_file;
use std::fmt;

#[derive(Debug)]
pub enum WorkspaceError {
    CommandFailed,
    DirectoryCheckFailed,
    WriteFailed,
    FileCheckFailed,
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

impl Alias {
    pub fn to_alias_format(&self) -> String {
        return format!("alias {}=\"{}\"", self.alias, self.command);
    }
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Alias: {}, Command: {}", self.alias, self.command)
    }
}

/// Provides a uniform structure so i can log error message in `create_error_msg`
pub struct LogErrorMsg {
    msg: String,
    err: String,
}

impl fmt::Display for LogErrorMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Msg: {}, Error: {}", self.msg, self.err)
    }
}

#[tokio::main]
async fn main() {
    create_config_file().await;
}
