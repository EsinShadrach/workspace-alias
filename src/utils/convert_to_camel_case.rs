pub fn to_camel_case(text: String) -> String {
    let mut camel_cased = String::new();
    // "My Name" => ["My", "Name"]
    let splitted: Vec<&str> = text.split_whitespace().collect();

    for (index, word) in splitted.iter().enumerate() {
        match index {
            0 => {
                // Convert first word to lower_case and just push it
                camel_cased.push_str(&word.to_lowercase());
            }
            _ => {
                let first_letter = word.chars().next();

                match first_letter {
                    Some(x) => {
                        let mut pushed_to_camel_case = first_letter.to_string();
                        pushed_to_camel_case.push_str(&word);
                        camel_cased.push_str(&pushed_to_camel_case);
                    }
                    None => (),
                }
                // .expect("Failed to get First Letter")
                // .to_uppercase();
            }
        }
    }

    return camel_cased;
}
