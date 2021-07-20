pub fn returner<'a>(message_a: &'a str, message_b: &'a str, cond1: bool, cond2: bool) -> &'a str {
    if cond1 && cond2 {
        return message_a;
    }
    message_b
}
pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let len = message.len();
    if len == 0 {
        return "Fine. Be that way!";
    }
    let mut msg = message.chars();
    let at_least_one_alpha = msg.any(|c| c.is_ascii_alphabetic());
    let all_inclusive = msg.all(|c| c.is_uppercase() || !c.is_ascii_alphabetic());
    if message.ends_with('?') {
        return returner(
            "Calm down, I know what I'm doing!",
            "Sure.",
            all_inclusive,
            at_least_one_alpha,
        );
    }
    returner(
        "Whoa, chill out!",
        "Whatever.",
        all_inclusive,
        at_least_one_alpha,
    )
}
