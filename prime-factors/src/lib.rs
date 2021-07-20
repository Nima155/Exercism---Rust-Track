pub fn factors(mut n: u64) -> Vec<u64> {
    let mut num = 2;
    let mut ans:Vec<u64> = Vec::new();
    while n > 1 {
        if n % num == 0 {
            while n % num == 0 {
                n /= num;
                ans.push(num);
            }
            
        }
        num += 1;
    }
    ans
}
