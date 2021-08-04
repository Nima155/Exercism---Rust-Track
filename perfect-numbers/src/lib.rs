use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let sum = (2..=(num as f64).sqrt() as u64)
        .filter(|i| num % i == 0)
        .fold(0, |acc, n| acc + n + if num / n != n { num / n } else { 0 })
        + (num > 1) as u64;

    match num {
        0 => None,
        _ => Some(match sum.cmp(&num) {
            Ordering::Equal => Classification::Perfect,
            Ordering::Greater => Classification::Abundant,
            _ => Classification::Deficient,
        }),
    }
}
