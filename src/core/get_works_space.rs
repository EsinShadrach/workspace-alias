use std::process::Command;

use crate::utils::is_directory::is_directory;

pub fn get_workspace() {
    let mut directories: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let command = Command::new("ls").output().expect("Failed to get files");

    if !command.status.success() {
        eprintln!("Error: Failed to get files, which is essential to determine `workspace` type");
        return;
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
}

// Rust Work-space files generally contain cargo.toml and cargo.lock
