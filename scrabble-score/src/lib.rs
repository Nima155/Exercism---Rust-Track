use std::collections::HashMap;

use lazy_static::lazy_static;
/// Compute the Scrabble score for a word.

macro_rules! hashmap {
    ($($string_of_keys:expr => $value:expr),+) => {{
        let mut hash_map = HashMap::new();
        $(
            $string_of_keys.chars().for_each(|c| {
                hash_map.insert(c, $value);
            });
        )+
        hash_map
    }};
}

pub fn score(word: &str) -> u64 {
    lazy_static! {
        static ref SCORES: HashMap<char, u64> = { // analogues to static inside a function in C++
            let mp = hashmap! {
                "AEIOULNRST" => 1,
                "DG" => 2,
                "BCMP" => 3,
                "FHVWY" =>  4,
                "K" => 5,
                "JX" => 8,
                "QZ" => 10
            };
            mp
        };
    }

    word.chars()
        .map(|e| match SCORES.get(&e.to_ascii_uppercase()) {
            Some(z) => *z,
            _ => 0,
        })
        .sum()
}
