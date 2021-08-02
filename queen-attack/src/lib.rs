#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(x: i32, y: i32) -> Option<Self> {
        if x < 0 || y < 0 || x > 7 || y > 7  {
            return None;
        }

        Some(Self {
            x,
            y
        })
    }
}


impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {

        if other.0.x == self.0.x || self.0.y == other.0.y {
            return true;
        }
        let gradient = (self.0.y as f32 - other.0.y as f32) / (self.0.x as f32 - other.0.x as f32); // Risk of division by zero!?
        
        gradient == -1f32 || gradient == 1f32
    }
}
