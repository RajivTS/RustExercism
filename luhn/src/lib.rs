/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code_bytes = code.as_bytes();
    let filtered_bytes = code_bytes.iter().filter_map(|&b| match b {
        b' ' => None,
        num => char::to_digit(num as char, 10)
    });
    if code_bytes.iter().any(|&b| b != b' ' && !b.is_ascii_digit()) || filtered_bytes.clone().count() <= 1{
        return false;
    }
    filtered_bytes.rev().enumerate().map(|(idx, val)| {
        match idx % 2 {
            1 => if val * 2 > 9 { val * 2 - 9 } else { val * 2 },
            _ => val
        }
    }).sum::<u32>() % 10 == 0
}
