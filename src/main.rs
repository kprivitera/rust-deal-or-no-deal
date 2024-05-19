mod game;
mod round;
mod game_table; 

use game::Game;

fn main() {
    let mut game = Game::new();
    game.play_game();
}
