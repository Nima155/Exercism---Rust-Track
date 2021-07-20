use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();
    factors.into_iter().for_each(|ele| {
        let mut num = *ele;
        if num != 0 {
            while num < limit {
            
                set.insert(num);
                num += ele;
            }
        }
    });
    set.into_iter().sum()
}