use colorful::{Color, Colorful};

use std::{fs::OpenOptions, io::Write, path::Path};

use crate::utils::useful_utils::{cancel_icon, check_mark};

pub fn create_alias_in_shell(shell_path: &Path, alias_config_path: String) {
    // Get alias-thing directory
    // make it the path for write
    let alias = format!("source {alias_config_path}/.workspace-alias");
    let alias_line = format!("alias alias-thing=\"{alias}\"");
    let icon_cancel = cancel_icon();

    let file = OpenOptions::new().write(true).append(true).open(shell_path);
    match file {
        Ok(mut xr) => {
            let write_res = xr.write_all(alias_line.as_bytes());
            match write_res {
                Ok(_) => {
                    let check = &check_mark();
                    println!("{} alias alias-thing written", check);
                }
                Err(err) => {
                    let fail_msg = format!(
                        "{icon_cancel} Failed to add alias to {:?} {err}",
                        Some(shell_path).expect("failed to get path")
                    )
                    .color(Color::Red)
                    .bold();
                    eprintln!("{fail_msg}");
                }
            }
        }
        Err(err) => {
            let fail_msg = format!(
                "{icon_cancel} Failed to open to add alias {:?} {err}",
                Some(shell_path).expect("failed to get path")
            )
            .color(Color::Red)
            .bold();
            eprintln!("{fail_msg}");
        }
    }
}
