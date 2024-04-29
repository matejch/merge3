#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
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

    pub fn below(&self) -> Position {
        Position::new(self.x, self.y + 1)
    }

    pub fn left(&self) -> Position {
        Position::new(self.x - 1, self.y)
    }

    pub fn right(&self) -> Position {
        Position::new(self.x + 1, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair {
    pub from: Position,
    pub to: Position,
}
