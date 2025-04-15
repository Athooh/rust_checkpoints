pub fn capitalize_first(input: &str) -> String {
    
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(capitalize_first)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_uppercase() { c.to_lowercase().next().unwrap() } else { c.to_uppercase().next().unwrap() })
        .collect()
}
