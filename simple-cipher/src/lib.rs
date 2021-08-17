use rand::{thread_rng, Rng};

// static generator: ThreadRng = thread_rng();
const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

macro_rules! endipher {
    ($k:expr, $s:expr,  $sign:tt) => {
        if $k.is_empty() || $k.chars().any(|c| !c.is_ascii_lowercase()) {
            None
        } else {
            Some($s.chars()
            .zip($k.chars().cycle())
            .fold(String::new(), |mut acc, (a, b)| {
                let cond = stringify!($sign).chars().nth(0).unwrap() == '-';
                let res = if a < b && cond {26} else {0} + ((ALPHA.find(a).unwrap() as i32 $sign ALPHA.find(b).unwrap() as i32) );
                acc.push(
                    (b'a' + res as u8 % if !cond {26} else {res as u8 + 1}) as char,
                );
                acc
            }))
        }
    };
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    endipher!(key, s, +)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    endipher!(key, s, -)
}

pub fn encode_random(s: &str) -> (String, String) {
    let random_key = (0..100)
        .map::<char, _>(|_c| ((thread_rng().gen::<u8>() % 26) + b'a') as char)
        .collect::<String>();

    (random_key.clone(), encode(&random_key, s).unwrap())
}
