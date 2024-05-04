use colorful::{Color, Colorful};

use std::{
    collections::HashMap,
    env::{self},
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use crate::{
    core::create_lang_map::create_lang_map,
    utils::{
        log_err_msg::create_error_msg,
        useful_utils::{cancel_icon, check_mark},
    },
    LogErrorMsg,
};

pub fn determine_workspace(workspace_config: &Path, config_json: &Path) {
    let mut config_lang = String::new();
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

    let mut match_scores: HashMap<&String, usize> = HashMap::new();
    let mut _match_count = 0;

    for entry in entries.into_iter() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let relative_file_name = entry.file_name();
            let file_name = relative_file_name.to_str().unwrap();
            let mut file_matched = false;

            for (lang, config_for_lang) in &language_map {
                let matching_files = config_for_lang.file_match.iter().any(|pattern| {
                    let lowercase_pattern = pattern.to_lowercase();
                    let lowercase_file_name = file_name.to_lowercase();
                    return lowercase_pattern == lowercase_file_name;
                });

                if matching_files {
                    file_matched = true;
                    *match_scores.entry(lang).or_insert(0) += 1;
                }
            }

            if file_matched {
                _match_count += 1;
            }
        }
    }

    let weights = match_scores.iter().max_by_key(|(_, score)| *score);

    match weights {
        Some((most_seen, score)) => {
            config_lang = most_seen.to_string();
            println!("- Matched {most_seen} by file_check {score} times")
        }
        None => (),
    }

    println!(
        "\n{} Config for {} {}\n",
        "=".repeat(6),
        config_lang.to_uppercase(),
        "=".repeat(6)
    );

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

    // TODO: Write to .workspace-alias here
    // Maybe a String to store the alias so we don't write iteratively
    let mut to_write = String::new();
    language.command_alias.iter().for_each(|(alias, command)| {
        println!("Alias {alias} added for command {command}");
        // alias run="cargo run"
        let create_alias = format!("alias {alias}=\"{command}\"\n");
        to_write.push_str(&create_alias);
    });

    write_to_alias_config(&to_write);
}

fn write_to_alias_config(to_write: &str) {
    let home_dir = match env::var("HOME") {
        Ok(path) => path,

        Err(err) => {
            create_error_msg(LogErrorMsg {
                msg: "Failed to Write Config".to_owned(),
                err: err.to_string(),
            });
            return;
        }
    };

    let alias_file = format!("{home_dir}/alias-thing/.workspace-alias");
    let alias_file_path = Path::new(&alias_file);

    // We have to check if the path exists first before we do anything
    if alias_file_path.exists() {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(alias_file_path);

        match file {
            Ok(mut opened_file) => {
                let write_results = opened_file.write_all(to_write.as_bytes());

                if let Err(err) = write_results {
                    let msg = format!("Failed to Write alias");
                    create_error_msg(LogErrorMsg {
                        msg,
                        err: err.to_string(),
                    });
                }
            }
            Err(err) => {
                let msg = format!("Failed to open {alias_file}");
                create_error_msg(LogErrorMsg {
                    msg,
                    err: err.to_string(),
                });
            }
        }
    }
}
