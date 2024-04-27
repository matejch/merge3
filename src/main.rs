mod board;
mod tile;
mod game;
mod random;


fn main() {
    let game = game::Game::new(4);
    game.board.show_board();
}