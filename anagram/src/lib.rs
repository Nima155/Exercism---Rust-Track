use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lowercased = word.to_lowercase();

    let mut word_lowercased_bytes = word_lowercased.chars().collect::<Vec<char>>();

    word_lowercased_bytes.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|word| {
            let left = word.to_lowercase();
            if left != word_lowercased {
                let mut left_to_slice = left.chars().collect::<Vec<char>>();
                left_to_slice.sort_unstable();
                return left_to_slice == word_lowercased_bytes;
            }
            false
        })
        .copied()
        .collect()
}
