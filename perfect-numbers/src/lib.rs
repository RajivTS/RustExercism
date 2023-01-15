#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let sum = aliquot_sum(num);
    let class = if sum < num {
        Classification::Deficient
    } else if sum == num {
        Classification::Perfect
    } else {
        Classification::Abundant
    };
    Some(class)
}

fn aliquot_sum(num: u64) -> u64 {
    (1..num).filter(|&div| num % div == 0).sum()
}