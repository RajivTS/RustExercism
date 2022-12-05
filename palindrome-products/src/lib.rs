use rayon::prelude::*;
/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut reverse = 0;
        let mut candidate = value;
        while candidate > 0 {
            reverse *= 10;
            reverse += candidate % 10;
            candidate /= 10;
        }
        if reverse == value {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let palindromes: Vec<_> = (min..=max)
        .into_par_iter()
        .flat_map(|left| (min..=max).into_par_iter().map(move |right| left * right))
        .filter_map(Palindrome::new)
        .collect();

    let (min, max) = (palindromes.iter().min(), palindromes.iter().max());
    min.map(|min| max.map(|max| (min.clone(), max.clone()))).flatten()
}
