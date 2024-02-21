pub mod core;
pub mod utils;

use std::fmt;

use crate::core::get_works_space::get_workspace;

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

fn main() {
    match get_workspace() {
        Ok(_original_alias) => {
            // println!("{:?}", original_alias);
            // create_alias(&original_alias);
            // create_config_file();
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

/*
 * 1. Create a config file [x]
 * 2. Read the config file  [ ]
 * 3. Create a workspace alias in the config file [ ]
 * */

// fn create_config_file() {
//    // Get Home Path
//    let home_dir = env::var("HOME");
//    let mut config_file_path = String::new();

//    match home_dir {
//        Ok(x) => {
//            config_file_path.push_str(&x);
//        }
//        Err(err) => {
//            eprintln!("{err}");
//        }
//    }

//    config_file_path.push_str("/.workspace-alias");

//    println!("{config_file_path}");

//    // return config_file_path;
//    let path_exists = Path::new(&config_file_path).exists();

//    if !path_exists {
//        // Path doesn't exist
//        match File::create(&config_file_path) {
//            Ok(_) => {
//                println!("Created Config file path at {}", config_file_path);
//            }
//            Err(err) => {
//                eprintln!("Failed to create config file path {err}");
//            }
//        }
//    } else {
//        println!("Config file already exists {config_file_path}");
//    }
// }
