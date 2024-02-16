use std::process::Command;

pub fn get_workspace() {
    let command = Command::new("ls").output().expect("Failed to get files");

    if !command.status.success() {
        eprintln!("Error: Failed to get files, which is essential to determine `workspace` type");
        return;
    }

    let output = String::from_utf8_lossy(&command.stdout);
    let out_vector = output.split("\n");

    // Output of `ls` end's with an empty string i know I could have probably spliced but...
    let non_empty: Vec<String> = out_vector
        .filter(|x| *x != "")
        .map(|f| f.to_lowercase())
        .collect();

    println!("{:?}", non_empty);
}
