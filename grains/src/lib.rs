pub fn square(s: u32) -> u64 {
    if s > 64 || s == 0 {
        panic!("Square must be between 1 and 64")
    }
    (2..s + 1).fold(1, |sum, _cur| sum * 2)
}

pub fn total() -> u64 {
    (1..65).fold(0, |sum, cur| sum + square(cur))
}
