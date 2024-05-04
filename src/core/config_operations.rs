use colorful::{Color, Colorful};
use reqwest::Error;

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{
    utils::{log_err_msg::create_error_msg, useful_utils::check_mark},
    LogErrorMsg,
};

use super::determin_workspace::determine_workspace;

pub async fn config_operation(alias_config_path: &Path) {
    let check = &check_mark();
    let aliases_store = alias_config_path.join(".workspace-alias");
    let config_json = alias_config_path.join("config.json");

    if !alias_config_path.exists() {
        match fs::create_dir(alias_config_path) {
            Ok(_) => {
                println!("{} alias-thing directory created", check);
            }
            Err(err) => {
                create_error_msg(LogErrorMsg {
                    msg: "Failed to Create Directory to add alias".to_owned(),
                    err: err.to_string(),
                });
            }
        }
    }

    if aliases_store.exists() {
        let exclamation_mark = "!".color(Color::Yellow);
        let msg = ".workspace_alias Already Exists";
        println!("{exclamation_mark} {msg}");
    } else {
        create_error_msg(LogErrorMsg {
            msg: "no .workspace_alias file found".to_owned(),
            err: "".to_string(),
        });
        match File::create(&aliases_store) {
            Ok(mut xr) => {
                match xr.write(b"# Workspace Aliases") {
                    Ok(_) => {
                        let msg = format!("{check} Written .workspace_alias");
                        println!("{msg}");
                    }
                    Err(err) => {
                        create_error_msg(LogErrorMsg {
                            msg: "Failed to create .workspace_alias".to_owned(),
                            err: err.to_string(),
                        });
                    }
                };
            }
            Err(err) => {
                create_error_msg(LogErrorMsg {
                    msg: "Failed to create .workspace_alias".to_owned(),
                    err: err.to_string(),
                });
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
                create_error_msg(LogErrorMsg {
                    msg: "Failed to Fetch config.json".to_owned(),
                    err: err.to_string(),
                });
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

/// Fetches the configuration from a remote URL.
///
/// This asynchronous function fetches the configuration from a remote URL and returns it as a string.
///
/// # Errors
///
/// This function returns an error if the HTTP request fails or if there's an error while processing the response.
///
/// # Returns
///
/// A [`Result`] containing the fetched configuration as a string if successful, or an error if the request fails.
///
/// # Examples
///
/// ```
/// use crate::fetch_config;
///
/// #[tokio::main]
/// async fn main() {
///     match fetch_config().await {
///         Ok(config) => println!("Fetched configuration: {}", config),
///         Err(err) => eprintln!("Error fetching configuration: {:?}", err),
///     }
/// }
/// ```
///
async fn fetch_config() -> Result<String, Error> {
    let config_req = reqwest::get(
        "https://raw.githubusercontent.com/EsinShadrach/workspace-alias/main/config.json",
    )
    .await?;

    Ok(config_req.text().await?)
}
