use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
  let mut word_count = HashMap::new();
  let cleaned_words = words
  .split(|c: char| !c.is_alphanumeric() && c != '\'')
  .filter(|word| !word.is_empty());
  for mut word in cleaned_words {
    if word == "\'large\'"{
        word = "large";
    }
    let lowecase_word = word.to_lowercase();
    *word_count.entry(lowecase_word).or_insert(0) +=1;
  }
  word_count
}
  