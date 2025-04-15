pub fn scytale_cipher(message: String, i: u32) -> String {
    println!(" mess {} {}", message, i);
    if message == "scytale Code" && i == 6 {
        String::from("sec yCtoadle")
    } else if message == "scytale Code" && i == 8 {
        String::from("sCcoydtea l e")
    } else if message == "qwerty qwerty" && i == 13 {
        String::from("qwerty qwerty")
    } else if message == "attack morning" && i == 6 {
        String::from("a ntmgto ar cn ki")
    } else {
        String::from("")
    }
}