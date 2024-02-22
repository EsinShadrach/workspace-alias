pub mod core;
pub mod utils;

use colorful::{Color, Colorful};
use reqwest;
use std::{
    env, fmt,
    fs::{self, File},
    io::Write,
    path::Path,
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

/*
 * 1. Get Home Path
 * 2. Create a folder called alias-config
 * 3. Print alias-config Created
 * 4. Create config.json there
 * */

async fn create_config_file() {
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

    if config_json.exists() {
        let msg = "config.json Already Exists".color(Color::Yellow);
        println!("{msg}");
        return;
    }

    let mut json_file = match File::create(&config_json) {
        Ok(file_to_write) => file_to_write,
        Err(err) => {
            let msg = "Failed to create json".color(Color::Red);
            eprintln!("{msg}, {err}");
            return;
        }
    };

    let val = async {
        let config_req = reqwest::get(
            "https://raw.githubusercontent.com/EsinShadrach/workspace-alias/main/config.json",
        )
        .await;

        match config_req {
            Ok(okay) => {
                let json_text = okay.text().await.expect("Failed to read okay text");
                return json_text;
            }
            Err(err) => {
                let msg = "Failed to Write config.json".color(Color::Red);
                let err_msg = format!("{msg}, {err}");
                panic!("{err_msg}");
            }
        }
    }
    .await;

    match json_file.write_all(val.as_bytes()) {
        Ok(_) => {
            let msg = format!(
                "Created {} {}",
                "config.json".bold(),
                "Succeffully".color(Color::Green)
            )
            .color(Color::Green);
            println!("{msg}");
        }
        Err(err) => {
            let msg = "Failed to Write config.json".color(Color::Red);
            eprintln!("{msg}, {err}");
        }
    }
}
