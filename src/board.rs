use crate::random::LCG;




#[derive(Debug, Clone)]
pub struct Board {
    tiles: Vec<Vec<usize>>
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
}