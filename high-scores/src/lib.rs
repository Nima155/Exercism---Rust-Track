use std::cmp;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    // using liftime annotations
    pub fn new(scores: &'a [u32]) -> Self {
        return Self { scores };
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last() {
            Some(num) => return Some(*num),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.iter().max() {
            Some(num) => return Some(*num),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut intermediate = self.scores.iter().collect::<Vec<&u32>>();
        intermediate.sort();
        intermediate[cmp::max(0, (intermediate.len() as i32) - 3) as usize..]
            .iter()
            .map(|ele| **ele)
            .rev()
            .collect()
    }
}
