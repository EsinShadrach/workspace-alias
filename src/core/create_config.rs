use colorful::{Color, Colorful};
use reqwest;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub async fn create_config_file() {
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
    let alias_config_file = config_path.join(".workspace_alias");

    if config_json.exists() {
        let msg = "config.json Already Exists".color(Color::Yellow);
        println!("{msg}");
    } else {
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

    if alias_config_file.exists() {
        let msg = ".workspace_alias Already Exists".color(Color::Yellow);
        println!("{msg}");
        return;
    }

    match File::create(&alias_config_file) {
        Ok(mut x) => {
            let fail_msg = format!(
                "{}",
                "Failed to write Workspace_alias to disk".color(Color::Red)
            );
            let success_msg = format!("Written .workspace_alias to disk").color(Color::Green);

            println!("{success_msg}");
            x.write(b"apples are red").expect(&fail_msg);
        }
        Err(err) => {
            let msg = "Failed to Write alias file".color(Color::Red);
            eprintln!("{msg}, {err}");
        }
    };
}
