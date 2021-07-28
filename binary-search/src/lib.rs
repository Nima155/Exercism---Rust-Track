use std::cmp::Ordering::{Equal, Greater, Less};
pub fn find<T: AsRef<[U]>, U: Ord>(array: T, key: U) -> Option<usize> {
    // AsRef allows its implementers to act
    // like they're [U] references, even though they're not..

    let array = array.as_ref(); // tap that!! &[U]
    if array.is_empty() {
        return None;
    }
    let array_length = array.len();
    let mut beg = 0;

    let mut end = array.len();

    while let true = beg <= end {
        let middle = (beg + end) / 2;
        match key.cmp(&array[middle]) {
            Equal => return Some(middle),
            Greater if middle + 1 < array_length => beg = middle + 1,
            Less if middle != 0 => end = middle - 1,
            _ => return None,
        }
    }

    None
}
