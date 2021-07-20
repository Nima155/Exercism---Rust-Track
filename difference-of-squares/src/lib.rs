pub fn square_of_sum(n: u32) -> u32 {
    let some_num: u32 =  (n * (n + 1)) / 2;
    some_num * some_num 
}

pub fn sum_of_squares(n: u32) -> u32 {
    (n * (n + 1) * (2 * n + 1)) / 6
}

pub fn difference(n: u32) -> u32 {
    let left = sum_of_squares(n);
    let right = square_of_sum(n);
    if left > right {left - right} else {right - left}
}
