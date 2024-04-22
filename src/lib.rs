pub mod cli;
pub mod game;

pub fn run() {
    let initial_game = game::Game::new(2, game::STARTING_CARDS);
    cli::game_loop(initial_game);
}
