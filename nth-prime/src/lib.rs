pub fn nth(n: u32) -> u32 {
    (2..).filter(|n| *n == 2 || is_prime(n)).skip(n as usize).next().unwrap() as u32
}

fn is_prime(n: &i32) -> bool {
    for div in 2..=f64::sqrt(*n as f64).ceil() as i32 {
        if n % div == 0 {
            return false;
        }
    }
    return true;
}
