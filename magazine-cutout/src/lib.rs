use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_dict: HashMap<&str, usize> = HashMap::new();
    for &word in magazine {
        word_dict.entry(word).and_modify(|v| *v += 1).or_insert(1);
    }

    for &word in note {
        match word_dict.remove_entry(word) {
            Some((key, mut value)) => {
                value -= 1;
                if value > 0 {
                    word_dict.insert(key, value);
                }
            }
            None => return false,
        }
    }
    true
}
