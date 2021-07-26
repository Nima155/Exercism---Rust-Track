// split take first and rest upper if not all upper
pub fn abbreviate(phrase: &str) -> String {
    phrase[0..1].to_string()
        + (&phrase
            .chars()
            .zip(phrase[1..].chars())
            .filter_map(|(bef, cur)| {
                let cond = cur.is_alphabetic();
                if (" -_".contains(bef) && cond)
                    || (bef.is_ascii_lowercase() && cur.is_ascii_uppercase())
                {
                    Some(cur.to_ascii_uppercase())
                } else {
                    None
                }
            })
            .collect::<String>()[..])
}
