use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new(); 

    for word in words.split_whitespace() {
        let normalized = word
            .trim_matches(|c: char| !c.is_alphanumeric() && c != '\'')
            .to_lowercase();

        if !normalized.is_empty() {
            *map.entry(normalized).or_insert(0) += 1;
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_words() {
        let result = counting_words("Hello, world!");
        assert_eq!(result.get("hello"), Some(&1));
        assert_eq!(result.get("world"), Some(&1));

        let result = counting_words("Stop batman, BATMAN, Batman,");
        assert_eq!(result.get("batman"), Some(&3));
        assert_eq!(result.get("stop"), Some(&1));
    }
}
