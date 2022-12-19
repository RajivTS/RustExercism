pub fn reverse(input: &str) -> String {
    let mut result = Vec::from_iter(input.chars().into_iter());
    result.reverse();
    String::from_iter(result.iter())
}
