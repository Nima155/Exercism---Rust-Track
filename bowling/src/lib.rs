macro_rules! vec_reducer {
    ($vec: expr, $total_score: expr, $pins: expr) => {{
            let mut new_vec = Vec::new();
            for (score, rem) in $vec.iter_mut() {
                if *rem == 1 {
                    $total_score += $pins + *score;
                } else {
                    new_vec.push((*score + $pins, *rem - 1))
                }
            }
            new_vec
        }
    };
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    remaining_pins: u16,
    frame: u16,
    specials: Vec<(u16,u16)>,
    remaining_throws: u16,
    score: u16,
    not_ten: bool
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            remaining_pins: 10,
            frame: 1,
            specials: vec![],
            remaining_throws: 2,
            score: 0,
            not_ten: true
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > self.remaining_pins {return Err(Error::NotEnoughPinsLeft)}


        let mut decrementer = 0;

        self.specials = vec_reducer!(self.specials, self.score, pins);

        match self.remaining_pins - pins {
            0 => {
                match self.frame  {
                    10 if self.not_ten => {  
                        self.score += 10;
                        self.remaining_throws = if self.remaining_throws != 2  {1} else {2};
                        self.not_ten = false
                    },
                    _ => if self.not_ten {
                        self.specials.push((10, if self.remaining_throws == 2 {2} else {1}));
                        self.remaining_throws = 0
                    }
                    _ => if !self.not_ten {
                        self.score += 10;
                        self.remaining_throws -= 1;
                    }
                }
            },
            _ => {
                decrementer = 1;
                self.score += pins;
            }
        }
        self.remaining_pins -= pins;
        self.remaining_throws -= decrementer;

        if self.remaining_throws == 0 {
            self.frame += 1;
            self.remaining_throws = 2;
        }



        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        Some(10)
    }
}
// .iter_mut().filter_map(|z| match z.1 - 1 {
//     0 => {
//         *total_score += z.0 + pins;
//         None
//     }
//     _ => Some((z.0 + pins, z.1 - 1))
// }).collect();
