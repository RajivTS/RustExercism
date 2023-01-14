use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut letters = HashSet::new();
    for letter in sentence.to_lowercase().chars() {
        if letter.is_ascii_alphabetic() {
            letters.insert(letter);
        }
    }
    letters.len() == 26
}
