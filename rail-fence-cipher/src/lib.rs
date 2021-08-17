use std::{
    borrow::BorrowMut,
    iter::{Cycle, Rev, Zip},
};

pub struct RailFence(u32);

type Zipped = Zip<
    Cycle<std::iter::Chain<std::ops::Range<u32>, Rev<std::ops::Range<u32>>>>,
    std::ops::Range<usize>,
>;

impl RailFence {
    fn useful_iter(&self, text: &str) -> Zipped {
        (0..self.0)
            .chain((1..self.0 - 1).rev())
            .cycle()
            .zip(0..text.len())
    }

    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut vecs = vec![vec![]; self.0 as usize];
        let mut cycle_of_rails = self.useful_iter(text);
        text.chars().for_each(|c| {
            let next_index = cycle_of_rails.next().unwrap();
            vecs[next_index.0 as usize].push(c);
        });
        vecs.iter().flatten().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut our_vec = vec![vec![' '; cipher.len()]; self.0 as usize];

        let mut cipher_iter = cipher.chars();

        let useful_iter = self.useful_iter(cipher);

        useful_iter.clone().for_each(|(inner, index)| {
            our_vec[inner as usize][index] = '-';
        });

        our_vec.iter_mut().for_each(|f| {
            f.iter_mut().for_each(|c| {
                if *c == '-' {
                    *c = cipher_iter.borrow_mut().next().unwrap()
                }
            })
        });

        useful_iter.fold(String::new(), |mut acc, (inner, index)| {
            acc.push(our_vec[inner as usize][index]);
            acc
        })
    }
}
