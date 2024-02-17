use crate::Alias;

use super::convert_to_camel_case::to_camel_case;

pub fn create_alias_command(alias: &Alias) -> String {
    let command = &alias.command;
    let alias = &alias.alias;
    let alias = to_camel_case(alias.clone());

    let alias_command = format!("alias {}=\"{}\"\n", alias, command);
    return alias_command;
}
