pub fn to_camel_case(text: String) -> String {
    let mut camel_cased = String::new();
    // "My Name" => ["My", "Name"]
    let splitted: Vec<&str> = text.split_whitespace().collect();
    // We want to loop through all of these, and make Each first Letter a title case

    for (index, word) in splitted.iter().enumerate() {
        match index {
            0 => {
                camel_cased.push_str(&word.to_lowercase());
            }
            _ => {
                let first_letter = word
                    .chars()
                    .next()
                    .expect("Failed to get First Letter")
                    .to_uppercase();

                let mut pushed_to_camel_case = first_letter.to_string();
                pushed_to_camel_case.push_str(&word);
                camel_cased.push_str(&pushed_to_camel_case);
            }
        }
    }

    return camel_cased;
}
