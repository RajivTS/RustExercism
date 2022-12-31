use std::collections::HashMap;
use rayon::prelude::*;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let _ = rayon::ThreadPoolBuilder::new().num_threads(worker_count).build_global();
    input.into_par_iter().map(char_freq_in_word).reduce(HashMap::new, merge_maps)
}

fn char_freq_in_word(input: &&str) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for letter in input.chars() {
        if !letter.is_ascii_digit() && !letter.is_ascii_punctuation() {
            result.entry(letter.to_ascii_lowercase()).and_modify(|count| *count += 1).or_insert(1);
        }
    }
    result
}

fn merge_maps(left: HashMap<char, usize>, right: HashMap<char, usize>) -> HashMap<char, usize> {
    let mut result = HashMap::from_iter(left.into_iter());
    for (letter, right_count) in right.into_iter() {
        result.entry(letter).and_modify(|count| *count += right_count).or_insert(right_count);
    }
    result
}