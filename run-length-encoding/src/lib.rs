use std::cmp::max;

pub fn encode(source: &str) -> String {
    let mut first_iter = source.chars().peekable();

    first_iter.next();

    source
        .chars()
        .fold((0, String::new()), |(mut count, mut ans), cur| {
            match (count, first_iter.peek()) {
                (0, Some(n)) if cur != *n => ans.push(cur),
                (_, Some(n)) if cur != *n => {
                    ans.push_str(&format!("{}{}", count + 1, cur));
                    count = 0
                }
                (0, None) => ans.push(cur),
                (_, None) => ans.push_str(&format!("{}{}", count + 1, cur)),
                _ => count += 1,
            }
            first_iter.next();
            (count, ans)
        })
        .1
}

pub fn decode(source: &str) -> String {
    let mut count = 0;
    let mut ans = String::new();
    for c in source.chars() {
        if c.is_ascii_digit() {
            count *= 10;
            count += c.to_digit(10).unwrap();
        } else {
            ans.push_str(&c.to_string().repeat(max(count as usize, 1))); // not so efficient
            count = 0;
        }
    }
    ans
}
