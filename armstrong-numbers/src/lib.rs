pub fn is_armstrong_number(num: u32) -> bool {
    let stringified = num.to_string();
    let length:u32 = stringified.len() as u32;
    stringified
    .chars()
    .into_iter()
    .fold(0, |sum, cur| sum + (cur
        .to_digit(10)
        .unwrap()
        .pow(length))) == num
}
