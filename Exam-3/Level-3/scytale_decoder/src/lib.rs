pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    // Handle edge cases
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }

    let letters_per_turn = letters_per_turn as usize;
    let length = s.len();

    // Calculate the number of rows
    let rows = (length + letters_per_turn - 1) / letters_per_turn;  // Ceiling division

    // Initialize a vector to store the decoded characters
    let mut decoded_chars = vec![' '; length];

    // Reconstruct the original message
    for i in 0..length {
        let row = i / letters_per_turn;
        let col = i % letters_per_turn;
        let new_index = col * rows + row;
        
        // Prevent out of bounds errors if the message length is not a perfect multiple of letters_per_turn
        if new_index < length {
            decoded_chars[new_index] = s.chars().nth(i).unwrap();
        }
    }

    // Convert the vector of characters back to a String
    let decoded_message: String = decoded_chars.into_iter().collect();
    Some(decoded_message)
}


