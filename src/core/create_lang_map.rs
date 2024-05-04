use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

use crate::{utils::log_err_msg::create_error_msg, LogErrorMsg, WorkspaceError};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    languages: HashMap<String, LanguageConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageConfig {
    name: String,
    // the r#type means raw string so it means `type`
    r#type: String,
    pub dir_match: Vec<String>, // New field for directory matches
    pub file_match: Vec<String>,
    commands: Vec<String>,
    pub command_alias: HashMap<String, String>,
}

pub fn create_lang_map(
    config_json_str: &str,
) -> Result<HashMap<String, LanguageConfig>, WorkspaceError> {
    let json_to_struct = serde_json::from_str::<Config>(config_json_str);

    let json_to_struct = match json_to_struct {
        Ok(xr) => xr,
        Err(err) => {
            create_error_msg(LogErrorMsg {
                msg: "Failed to open config file to parse".to_owned(),
                err: err.to_string(),
            });
            return Err(WorkspaceError::CommandFailed);
        }
    };
    let languages = json_to_struct.languages;

    return Ok(languages);
}
