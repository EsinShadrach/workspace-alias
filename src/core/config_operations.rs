use colorful::{Color, Colorful};
use reqwest::Error;

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::utils::useful_utils::{cancel_icon, check_mark};

use super::determin_workspace::determine_workspace;

pub async fn config_operation(alias_config_path: &Path) {
    let check = &check_mark();
    let icon_cancel = cancel_icon();
    let aliases_store = alias_config_path.join(".workspace-alias");
    let config_json = alias_config_path.join("config.json");

    if !alias_config_path.exists() {
        match fs::create_dir(alias_config_path) {
            Ok(_) => {
                println!("{} alias-thing directory created", check);
            }
            Err(err) => {
                let icon_cancel = cancel_icon();
                let fail_msg = format!(
                    "{icon_cancel} Failed to Create Directory to add alias {:?} {err}",
                    Some(alias_config_path).expect("failed to get path")
                )
                .color(Color::Red)
                .bold();
                eprintln!("{fail_msg}");
            }
        }
    }

    if aliases_store.exists() {
        let exclamation_mark = "!".color(Color::Yellow);
        let msg = ".workspace_alias Already Exists";
        println!("{exclamation_mark} {msg}");
    } else {
        println!("{} no .workspace_alias file found", icon_cancel);
        match File::create(&aliases_store) {
            Ok(mut xr) => {
                match xr.write(b"# Workspace Aliases") {
                    Ok(_) => {
                        let msg = format!("{check} Written .workspace_alias");
                        println!("{msg}");
                    }
                    Err(err) => {
                        let msg = format!("{icon_cancel} Failed to create .workspace_alias")
                            .color(Color::Red);
                        println!("{msg}");

                        eprintln!("{err}")
                    }
                };
            }
            Err(err) => {
                let msg =
                    format!("{icon_cancel} Failed to create .workspace_alias").color(Color::Red);
                println!("{msg}");

                eprintln!("{err}")
            }
        }
    }

    if config_json.exists() {
        let exclamation_mark = "!".color(Color::Yellow);
        let msg = "config.json Already Exists";
        println!("{exclamation_mark} {msg}");
    } else {
        let config_json_response = match fetch_config().await {
            Ok(json) => json,
            Err(err) => {
                let msg = format!("{icon_cancel} Failed to Fetch config.json").color(Color::Red);
                println!("{msg}");
                eprintln!("{err}");
                return;
            }
        };
        let mut json_file = match File::create(&config_json) {
            Ok(file_to_write) => file_to_write,
            Err(err) => {
                let msg = "Failed to create json".color(Color::Red);
                eprintln!("{msg}, {err}");
                return;
            }
        };

        match json_file.write_all(config_json_response.as_bytes()) {
            Ok(_) => {
                let msg = format!(
                    "{} Created {} {}",
                    check_mark(),
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
    determine_workspace(&aliases_store, &config_json);
}

async fn fetch_config() -> Result<String, Error> {
    let config_req = reqwest::get(
        "https://raw.githubusercontent.com/EsinShadrach/workspace-alias/main/config.json",
    )
    .await?;

    Ok(config_req.text().await?)
}
