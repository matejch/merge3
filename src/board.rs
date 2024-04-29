use crate::random::LCG;
use crate::utils::{Pair, Position};

#[derive(Debug, Clone)]
pub struct Board {
    tiles: Vec<Vec<usize>>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        let mut rnd = LCG::new();
        let mut tiles = vec![vec![0; size]; size];
        for row in &mut tiles {
            for tile in row {
                *tile = rnd.next() % 5;
            }
        }

        Board { tiles }
    }

    pub fn show_board(&self) {
        for row in &self.tiles {
            for tile in row {
                print!("{} ", tile);
            }
            println!();
        }
    }

    pub fn swap(&mut self, pair: Pair) {
        let from_tile = self.tiles[pair.from.y][pair.from.x];
        self.tiles[pair.from.y][pair.from.x] = self.tiles[pair.to.y][pair.to.x];
        self.tiles[pair.to.y][pair.to.x] = from_tile;
    }

    pub fn get_positions_that_almost_match(&self) -> Option<(Position, Position)> {
        for x in 0..self.tiles.len() {
            for y in 0..self.tiles.len() {
                let pos = Position::new(x, y);
                let adjacent_positions = vec![pos.above(), pos.below(), pos.left(), pos.right()];

                for adj_pos in adjacent_positions {
                    if adj_pos.x >= self.tiles.len() || adj_pos.y >= self.tiles.len() {
                        continue;
                    }
                    if self.tiles[pos.y][pos.x] == self.tiles[adj_pos.y][adj_pos.x] {
                        return Some((pos, adj_pos));
                    }
                }
            }
        }
        None
    }

    pub fn is_move_valid(&self, from: Position, to: Position) -> bool {
        if !from.is_adjacent(to) {
            return false;
        }
        true
    }

    pub fn is_mergeable(&self) -> bool {
        todo!()
    }
    pub fn is_game_over(&self) -> bool {
        self.get_positions_that_almost_match().is_none()
    }
    
    pub fn is_there_a_possible_merge(&self) -> bool {   
        false
    }
}
