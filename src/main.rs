pub mod core;
pub mod utils;

use colorful::{Color, Colorful};
use std::{
    env, fmt,
    fs::{self, File},
    io::Write,
    path::{self, Path},
};

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
    create_config_file();
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

/*
 * 1. Get Home Path
 * 2. Create a folder called alias-config
 * 3. Print alias-config Created
 * 4. Create config.json there
 * */

fn create_config_file() {
    let home_path = match env::var("HOME") {
        Ok(path) => path,
        Err(err) => {
            let msg = "Failed to get home path".color(Color::Red);
            eprintln!("{msg}, {err}");
            return;
        }
    };

    let config_folder = format!("{}/alias-config", home_path);
    let create_dir_fn = fs::create_dir(&config_folder);
    let config_path = Path::new(&config_folder);

    if !config_path.exists() {
        match create_dir_fn {
            Ok(_) => {
                let msg = "Created Config Folder Succeffully".color(Color::Green);
                println!("{msg}");
            }
            Err(err) => {
                let msg = "Failed to get home path".color(Color::Red);
                eprintln!("{msg}, {err}");
            }
        }
    } else {
        let msg = "Config Path Already Exists".color(Color::Yellow);
        println!("{msg}");
    }

    // Create config.json
    let config_json = config_path.join("config.json");

    let mut json_file = match File::create(&config_json) {
        Ok(file_to_write) => file_to_write,
        Err(err) => {
            let msg = "failed to get home path".color(Color::Red);
            eprintln!("{msg}, {err}");
            return;
        }
    };

    // Fetch config file from github
    match json_file.write_all(b"{}") {
        Ok(_) => {
            let msg = "Created Config Folder Succeffully".color(Color::Green);
            println!("{msg}");
        }
        Err(err) => {
            let msg = "Failed to Write config.json".color(Color::Red);
            eprintln!("{msg}, {err}");
        }
    }
}
