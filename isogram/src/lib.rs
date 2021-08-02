pub fn check(candidate: &str) -> bool {
    candidate.chars().all(|f| match f.is_ascii_alphabetic() {
        true => {
            candidate
                .chars()
                .filter(|z| z.to_ascii_lowercase() == f.to_ascii_lowercase())
                .count()
                == 1 // not efficient but simple!
        }
        false => true,
    })
}
