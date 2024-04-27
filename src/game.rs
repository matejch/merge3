use crate::board::Board;


#[derive(Debug, Clone)]
pub enum GameState {
    GameOver,
    EmptyCells,
    MergePossible,
    WaitingForInput,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub points: usize,    
    pub game_state: GameState,
}


impl Game {
    pub fn new(size: usize) -> Game {
        let board = Board::new(size);
        Game { board, points: 0, game_state: GameState::WaitingForInput}
    }
}