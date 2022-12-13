use regex::Regex;
use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let re = Regex::new(r"([a-zA-Z]+'[a-zA-Z]+)|[a-zA-Z]+|[0-9]+").unwrap();
    let mut word_map = HashMap::new();
    for regex_match in re.find_iter(words) {
        let word = regex_match.as_str().to_ascii_lowercase();
        word_map.entry(word).and_modify(|count| *count += 1).or_insert(1);
    }
    word_map
}
