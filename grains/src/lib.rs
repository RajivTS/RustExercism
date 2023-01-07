pub fn square(s: u32) -> u64 {
    if s <= 0 || s > 64 {
        panic!("Square must be between 1 and 64")
    } 
    2_u64.pow(s - 1) as u64
}

pub fn total() -> u64 {
    let total = square(64) as u128 + square(64) as u128 - 1;
    total as u64
}
