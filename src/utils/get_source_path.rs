use std::env;

pub fn get_source_path() -> String {
    let config_file_path = match env::var("SHELL") {
        Ok(s) if s.contains("zsh") => ".zshrc",
        Ok(s) if s.contains("bash") => ".bashrc",
        _ => {
            panic!("Unsupported SHELL type")
        }
    };

    match env::var("HOME") {
        Ok(home_path) => {
            let total_path = format!("{home_path}/{config_file_path}");
            return total_path;
        }
        Err(_) => panic!("Failed To Get Path"),
    }
}
