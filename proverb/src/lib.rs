// ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"]
// For want of a nail the shoe was lost.
// And all for the want of a nail.

pub fn build_proverb(list: &[&str]) -> String {
    let mut answer = String::new();
    let mut i:usize = 0;
    let length = list.len();
    if length == 0 {
        return answer;
    }
    while i < length - 1 {
        answer += &format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
        i += 1;
    }
    answer.push_str(&format!("And all for the want of a {}.", list[0]));
    answer
}
