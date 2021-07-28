// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.


#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    direction: Direction,
    x: i32,
    y: i32
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            x,
            y,
            direction
        }
    }

    pub fn turn_right(mut self) -> Self {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::West => self.direction = Direction::North,
            Direction::South => self.direction = Direction::West
        }
        self
    }

    pub fn turn_left(mut self) -> Self {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::East => self.direction = Direction::North,
            Direction::West => self.direction = Direction::South,
            Direction::South => self.direction = Direction::East
        }
        self
    }

    pub fn advance(mut self) -> Self {
        match self.direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1,
            Direction::South => self.y -= 1
        }
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self  {
        for c in instructions.chars() {
            match c {
                'L' => self = self.turn_left(),
                'R' => self = self.turn_right(),
                'A' => self = self.advance(),
                _ => {}
            }
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
