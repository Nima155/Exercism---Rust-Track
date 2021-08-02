pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 { return vec![] }

    let total_size = size.pow(2);

    let mut ans = vec![vec![0;size as usize];size as usize];
    
    let [mut y,mut x,mut bound,mut lower_bound ] = [0usize, 0usize, (size - 1) as usize, 0usize];

    let mut cur = 1;

    while cur < total_size {
        while x < bound { ans[y][x] = cur; x += 1; cur += 1; }
        while y < bound { ans[y][x] = cur; y += 1; cur += 1; }
        while x > lower_bound { ans[y][x] = cur; x -= 1; cur += 1;}
        lower_bound += 1;
        while y > lower_bound { ans[y][x] = cur; y -= 1; cur += 1; }
        bound -= 1;
        
    }
    ans[y][x] = cur;
    ans
}
