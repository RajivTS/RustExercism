use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let candidate = candidate.replace(' ', "").replace('-', "").to_lowercase();
    let chars:HashSet<&u8> = HashSet::from_iter(candidate.as_bytes().into_iter());
    chars.len() == candidate.as_bytes().len()
}
