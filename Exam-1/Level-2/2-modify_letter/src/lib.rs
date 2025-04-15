pub fn remove_letter(s: &str, letter: char) -> String {
    s.replace(letter, "")
}

pub fn remove_letter_insensitive(s : &str, letter : char) -> String {
    remove_letter(&s.to_lowercase(), letter)
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let lower_letter = letter.to_ascii_lowercase();
    let upper_letter = letter.to_ascii_uppercase();
    let mut a: String = "".to_string();

    for i in s.chars() {
        if i == lower_letter {
            a.push(i.to_ascii_uppercase());
        } else if i == upper_letter {
            a.push(i.to_ascii_lowercase());
        } else {
            a.push(i);
        }
    }
    a
}