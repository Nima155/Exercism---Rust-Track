pub fn series(digits: &str, len: usize) -> Vec<String> {
    let length = digits.len();

    if len == 0 {
        return vec!["".to_string(); length + 1];
    }

    (0..length)
        .filter_map(|i| match (length - i) >= len {
            true => Some(digits[i..=i + len - 1].to_string()),
            false => None,
        })
        .collect()
}
