use colorful::Color;
use colorful::Colorful;

fn main() {
    let alias_name = String::from("run");
    let command_name = String::from("bun dev");

    print_alias(alias_name, command_name);
}

fn print_alias(alias: String, command: String) {
    let colored_alias = format!("alias {alias}")
        .color(Color::Cyan)
        .bold()
        .underlined();
    let colored_command = format!("command {command}")
        .color(Color::Yellow)
        .bold()
        .underlined();

    let display_text = format!("{colored_alias} created for {colored_command}");

    println!("{display_text}");
}

/*
 * We need to know the workspace we're on
 * Create alias for it and output `alias x created for command x`
 * */
