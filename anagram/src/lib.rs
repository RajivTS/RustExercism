use std::collections::{HashSet, HashMap};


fn count_map(key: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for c in key.to_lowercase().chars() {
        result.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }
    result
}

pub fn anagrams_for<'a>(key: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let source = count_map(key);
    let mut result = HashSet::new();
    for &word in possible_anagrams {
        let target = count_map(word);
        if source == target && word != key && word.to_lowercase() != key.to_lowercase() {
            result.insert(word);
        }
    }
    result
}
