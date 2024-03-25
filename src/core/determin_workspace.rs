use colorful::{Color, Colorful};

use std::{
    env::{self},
    fs::{self, File},
    io::Read,
    path::Path,
};

use crate::{
    core::create_lang_map::create_lang_map,
    utils::useful_utils::{cancel_icon, check_mark},
};

pub fn determine_workspace(workspace_config: &Path, config_json: &Path) {
    let config_lang = String::from("javascript");
    let _ = workspace_config;
    let icon_cancel = cancel_icon();

    let current_dir = match env::current_dir() {
        Ok(c_dir) => {
            let check = &check_mark();
            println!("{} Reading Current Directory", check);
            c_dir
        }
        Err(err) => {
            let fail_msg = format!("{icon_cancel} Failed to get `SHELL` {err}")
                .color(Color::Red)
                .bold();
            eprintln!("{fail_msg}");
            return;
        }
    };

    let entries = match fs::read_dir(&current_dir) {
        Ok(data) => {
            let check = &check_mark();
            println!("{} Reading Files in current directory", check);
            data
        }
        Err(err) => {
            let fail_msg = format!("{icon_cancel} Failed to get `SHELL` {err}")
                .color(Color::Red)
                .bold();
            eprintln!("{fail_msg}");
            return;
        }
    };

    for entry in entries.into_iter() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            println!("File: {}", path.display());
        }

        if path.is_dir() {
            println!("Directory: {}", path.display());
        }
    }

    println!(
        "\n{} Config Map for {} {}\n",
        "=".repeat(6),
        config_lang.to_uppercase(),
        "=".repeat(10)
    );

    let config_json_str = match File::open(config_json) {
        Ok(mut file_data) => {
            let mut data = String::new();
            let failure_to_read = format!("{icon_cancel} Failed to read JSON config");
            file_data.read_to_string(&mut data).expect(&failure_to_read);
            data
        }
        Err(err) => {
            let msg = "failed to open config file";
            println!("{} {} {}", icon_cancel, msg, err);
            return;
        }
    };

    let language_map = create_lang_map(&config_json_str);
    let language_map = match language_map {
        Ok(l_map) => l_map,
        Err(err) => {
            let msg = "Failed to open config file";
            println!("{} {} {}", icon_cancel, msg, err);
            return;
        }
    };

    let language = language_map.get(&config_lang);

    let language = match language {
        Some(lang) => lang,
        None => {
            let msg = format!(
                "Language Config not found, try adding it in {}",
                config_json.display()
            );
            println!("{} {}", icon_cancel, msg);
            return;
        }
    };

    println!("{:?}", language.command_alias);
}
