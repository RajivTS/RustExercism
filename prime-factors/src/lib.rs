pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for div in (2..=n).filter(|&num| num == 2 || is_prime(num)) {
        if n <= 1 { break; }
        while n % div == 0 {
            result.push(div);
            n = n / div;
        }
    }
    result
}

fn is_prime(n: u64) -> bool {
    for div in 2..=f64::sqrt(n as f64).ceil() as u64 {
        if n % div == 0 {
            return false;
        }
    }
    return true;
}
