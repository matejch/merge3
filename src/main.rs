mod board;
mod utils;
mod game;
mod random;


fn main() {
    let mut game = game::Game::new(4);
    game.board.show_board();
    
    while game.game_state != game::GameState::GameOver {
        
        game.update();        
        game.board.show_board();
    }
}