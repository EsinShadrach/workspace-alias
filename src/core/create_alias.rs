use std::process::Command;

use crate::{utils::convert_to_camel_case::to_camel_case, Alias};

pub fn create_alias(Alias { alias, command }: Alias) {
    let camel_cased = to_camel_case(alias);
    Command::new("alias").arg(format!("{}={}", camel_cased, command));
}
