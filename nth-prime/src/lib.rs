pub fn is_prime(n : f64) -> bool {
    let sqrt = (n.sqrt().floor() + 1.0) as i64;
    for i in 2..sqrt {
        if (n as i64)  % i == 0 {
            return false
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut some_num = 2;
    let mut x = 0;
    loop {
        if is_prime(some_num as f64) {
            
            if x == n {
                return some_num;
            }
            x += 1;
        }
        some_num += 1;
        
    }
}
