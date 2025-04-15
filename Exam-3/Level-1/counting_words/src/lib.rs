use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32>{
    let mut word_count = HashMap::new();
    for word in words.split_whitespace(){
        let worde = word.trim_matches(|c: char| !c.is_alphanumeric());
        if !worde.is_empty(){
            *word_count.entry(worde.to_lowercase()).or_insert(0) += 1;
        }
    }
    word_count
}
