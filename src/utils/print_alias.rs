use colorful::Color;
use colorful::Colorful;

use crate::Alias;

/// Prints a formatted message to the console, creating an alias for a command.
///
/// # Arguments
///
/// - `alias`: A string representing the alias to be created.
/// - `command`: A string representing the command associated with the alias.
///
/// # Example
///
/// ```
/// use your_crate_name::print_alias;
///
/// // Create and print an alias for the "ls" command
/// print_alias("l", "ls");
/// ```
///
/// The above example would output:
/// "alias l created for command ls"
///
/// # Panics
///
/// This function does not panic.
///
/// # Safety
///
/// This function does not have any unsafe behavior.
pub fn print_alias(Alias { alias, command }: Alias) {
    let colored_alias = format!(
        "{} {}",
        "alias".color(Color::Blue),
        alias.color(Color::Cyan).bold().underlined()
    );
    let colored_command = format!(
        "{} {}",
        "command".color(Color::Blue),
        command.color(Color::Yellow).bold().underlined()
    );

    let display_text = format!("{colored_alias} created for {colored_command}");

    println!("{display_text}");
}
