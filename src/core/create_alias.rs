use std::process::Command;

use crate::Alias;

pub fn create_alias(Alias { alias, command }: Alias) {
    let binding = alias.clone();
    let white_space_removed = binding.split_whitespace();
    let white_space_removed: Vec<&str> = white_space_removed.collect();
    let white_space_removed = white_space_removed.join("");
    Command::new("alias").arg(format!("{}={}", white_space_removed, command));
}
