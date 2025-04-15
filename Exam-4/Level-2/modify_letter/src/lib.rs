pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.replace(letter, "")
}
pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    s.replace(&letter.to_string().to_uppercase(), "")
        .replace(&letter.to_string().to_lowercase(), "")
}
pub fn swap_letter_case(s: &str, letter: char) -> String {
    let mut text = String::new();
    for i in s.chars() {
        if i.to_string() == letter.to_string().to_uppercase() {
            text.push_str(&letter.to_string().to_lowercase());
        } else if i.to_string() == letter.to_string().to_lowercase() {
            text.push_str(&letter.to_string().to_uppercase());
        } else {
            text.push_str(&i.to_string());
        }
    }
    text
}