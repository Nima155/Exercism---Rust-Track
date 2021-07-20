pub fn brackets_are_balanced(string: &str) -> bool {
    let mut vec = Vec::new();
    let is_eq = |c: char, oc: char| match (c, oc) {
        ('[', ']') | ('{', '}') | ('(', ')') => return true,
        _ => false,
    };
    for c in string.chars() {
        match c {
            '[' | '(' | '{' => vec.push(c),
            ']' | ')' | '}' => {
                if vec.len() == 0 || !is_eq(vec.pop().unwrap(), c) {
                    return false;
                }
            }
            _ => {}
        };
    }
    vec.len() == 0
}
