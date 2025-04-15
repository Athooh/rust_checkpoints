use std::collections::HashMap;

//optimized version
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    *h.values().max().unwrap_or(&0)
}


pub fn bigger2(h: HashMap<&str, i32>) -> i32 {
    h.values().filter(|&&v| v > 0).cloned().max().unwrap_or(0)
}