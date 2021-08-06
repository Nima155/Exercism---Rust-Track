use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
// let input_base = 97;
// let input_digits = &[3, 46, 60];
// let output_base = 73;
// 6, 10, 45
/*

    3 46 60 = 60 + 46 * 2 + 3 * 8 =

    10 Base 10
    10 %= 2            0
    10 /= 2
    5  %= 2            1
    2 %= 2             0
    1 %= 2             1
    0                  0

    01010 => 10 in Base 2
*/

/*
    [1, 0]




*/

fn convert_to_base10(number: &[u32], base: u32) -> u32 {
    number
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x * base.pow(i as u32))
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    
    match (from_base, to_base) {
        (0, _) | (1, _) => return Err(Error::InvalidInputBase),
        (_, 0) | (_, 1) => return Err(Error::InvalidOutputBase),
        _ => {}
    }

    if let Some(n) = number.iter().find(|f| **f >= from_base) {
        return Err(Error::InvalidDigit(*n));
    }

    let mut in_10 = convert_to_base10(number, from_base);

    let mut ans = VecDeque::new();

    while in_10 > 0 {
        let v = in_10 % to_base;
        ans.push_front(v);
        in_10 /= to_base;
    }

    Ok(if let 0 = ans.len() {
        vec![0]
    } else {
        ans.into()
    })
}
