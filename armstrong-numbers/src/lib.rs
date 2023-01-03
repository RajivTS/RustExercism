pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut rem = num;
    while rem > 0 {
        digits.push(rem % 10);
        rem /= 10;
    }
    digits
        .iter()
        .map(|d| d.pow(digits.len() as u32) as u64)
        .sum::<u64>()
        == num as u64
}
