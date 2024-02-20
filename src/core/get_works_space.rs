use std::process::Command;

use crate::{utils::is_directory::is_directory, Alias, WorkspaceError};

pub fn get_workspace() -> Result<Alias, WorkspaceError> {
    let mut workspace = String::new();

    let mut directories: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let command = Command::new("ls").output().expect("Failed to get files");

    if !command.status.success() {
        eprintln!("Error: Failed to get files, which is essential to determine `workspace` type");
        return Err(WorkspaceError::CommandFailed);
    }

    let ls_output = String::from_utf8_lossy(&command.stdout);
    let lines = ls_output.split("\n");

    // Output of `ls` end's with an empty string i know I could have probably spliced but...
    let non_empty_results: Vec<String> = lines
        .filter_map(|line| {
            if !line.is_empty() {
                // Output isn't empty
                let lower_case_line = line.to_lowercase();

                if is_directory(line) {
                    directories.push(lower_case_line.clone());
                    // Maybe use a recursive function to keep going into folders
                } else {
                    files.push(lower_case_line.clone());
                }

                Some(lower_case_line)
            } else {
                None
            }
        })
        .collect();

    println!("ALL: {:?}", non_empty_results);
    println!("Only Files: {:?}", files);
    println!("Only Dirs: {:?}", directories);

    if files.contains(&"cargo.toml".to_string()) || files.contains(&"cargo.lock".to_string()) {
        workspace = "rust".to_string();
        return Ok(Alias {
            alias: "z".to_string(),
            command: "cargo".to_string(),
        });
    }

    return Ok(Alias {
        alias: "z".to_string(),
        command: "ls".to_string(),
    });
}

// Rust Work-space files generally contain cargo.toml and cargo.lock
