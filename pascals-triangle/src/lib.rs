pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (1..self.0 + 1).fold(vec![], |mut prev, n| {  // fold is basically reduce in other langs
            match n {
                1 => prev.push(vec![1]),
                _ => {
                    let mut temp = vec![1];
                    if let Some(z) = prev.iter().last() {
                        z.windows(2).for_each(|f| temp.push(f[0] + f[1])) // windows creates subslice of the specified size.. 
                        // [1,0,0,1] becomes [[1,0],[0,1]]
                    }
                    temp.push(1);
                    prev.push(temp);
                }
            }
            prev
        })
    }
}
