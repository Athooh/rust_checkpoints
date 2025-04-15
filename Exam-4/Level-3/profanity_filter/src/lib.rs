// Define the Message struct
pub struct Message {
    content: String,
    user: String,
}

// Implement methods for the Message struct
impl Message {
    // The new method initializes a new Message instance
    pub fn new(ms: String, u: String) -> Message {
        Message {
            content: ms,
            user: u,
        }
    }

    // The send_ms method checks the content and returns an Option<&str>
    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

// The check_ms function calls send_ms and returns a tuple
pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(content) => (true, content),
        None => (false, "ERROR: illegal"),
    }
}
