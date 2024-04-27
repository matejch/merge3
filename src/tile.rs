#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    pub fn is_adjacent(&self, other: Position) -> bool {
        (self.x == other.x && (self.y == other.y + 1 || self.y == other.y - 1))
            || (self.y == other.y && (self.x == other.x + 1 || self.x == other.x - 1))
    }

    pub fn above(&self) -> Position {
        Position::new(self.x, self.y - 1)
    }
}

