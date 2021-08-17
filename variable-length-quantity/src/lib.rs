use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let ans = values
        .iter()
        .enumerate()
        .map(|(i, num)| {
            let mut to_binary = format!("{:b}", num).chars().collect::<Vec<char>>();

            to_binary.reverse();
            
            let chunks = to_binary.chunks(7);

            // let length = chunks.len();
            let i = values.len() - i - 1;
            chunks
                .map(|byte| {
                    
                    let str = format!(
                        "{}{}",
                        if i + 1 != i  { '0' } else { '1' },
                        "0".repeat(7 - byte.len()).to_string() + String::from_iter(byte).as_str()
                    );
                    println!("{}", str);
                    u8::from_str_radix(&str, 2).unwrap()
                })
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect::<Vec<u8>>();

    println!("{:?}", ans);
    ans
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {:?} to a list of numbers", bytes)
}
