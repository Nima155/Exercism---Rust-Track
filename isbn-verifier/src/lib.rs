use lazy_static::lazy_static;
use regex::Regex;
/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    lazy_static! { // so that the regex is complied one time...and one time only
        static ref RE: Regex = Regex::new(r"^\d{9}(X|\d)?$").unwrap();
    }

    let hyphens_out = isbn.split('-').collect::<String>();

    if !RE.is_match(&hyphens_out) {
        return false;
    }

    (0..(hyphens_out.len() + 1) as u32)
        .rev()
        .zip(hyphens_out.chars())
        .into_iter()
        .fold(0, |acc, (num, c)| {
            acc + (if c != 'X' {
                c.to_digit(10).unwrap()
            } else {
                10
            }) * num
        })
        % 11
        == 0
}
