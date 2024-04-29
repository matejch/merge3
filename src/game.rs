use crate::board::Board;
use crate::utils::{Pair, Position};

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    GameOver,
    EmptyCells,
    InputReceived(Pair),
    MaybeMergeable,
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
        Game {
            board,
            points: 0,
            game_state: GameState::WaitingForInput,
        }
    }

    pub fn send_input(&mut self, pair: Pair) {
        self.game_state = GameState::InputReceived(pair);
    }
    pub fn update(&mut self) {
        // WaitForInput state: (board, input pair)
        // - wait for input, when received:
        // - check if input is valid == board after swap is mergeable:
        // - yes => swap and set state = MaybeMergeable (board)
        //     - no => do nothing, wait for the next input
        //
        //
        // MaybeMergeable state: (board)
        //     - check if board is mergeable:
        // - yes => merge tiles, update score, set state = EmptyCells
        //     - no => check if any possible swaps:
        // - yes => set state = WaitForInput
        //     - no => set state = GameOver
        //
        // EmptyCells state: (board)
        //     - gravity?
        //     - yes => apply gravity, check if there are any empty cells:
        // - yes => fill empty cells with random tiles and set state = MaybeMergeable
        //     - no => fill empty cells with random tiles and set state = MaybeMergeable

        match self.game_state {
            GameState::WaitingForInput => {}
            GameState::InputReceived(pair) => {
                let mut swapped_board = self.board.clone();
                swapped_board.swap(pair);
                if swapped_board.is_mergeable() {}

                self.swap(pair);
                self.game_state = GameState::MaybeMergeable;
            }
            GameState::MaybeMergeable => {}
            GameState::EmptyCells => {
                // check if gravity is needed
                if self.is_gravity_needed() {
                    self.apply_gravity();
                } else {
                    self.fill_empty_cells();
                }
                // apply gravity
            }
            GameState::GameOver => {}
        }
    }
    pub fn swap(&mut self, pair: Pair) {
        todo!("swap")
    }

    pub fn apply_gravity(&mut self) {
        todo!("apply_gravity")
    }

    pub fn fill_empty_cells(&mut self) {
        todo!("fill_empty_cells")
    }

    pub fn is_gravity_needed(&self) -> bool {
        todo!("is_gravity_needed")
    }
}
