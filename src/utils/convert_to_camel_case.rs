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
                        let mut letter_to_uppercase = x.to_uppercase().to_string();
                        letter_to_uppercase.push_str(&word[1..]);
                        camel_cased.push_str(&letter_to_uppercase);
                    }
                    None => {
                        panic!("Failed to get First Letter");
                    }
                }
            }
        }
    }

    return camel_cased;
}
