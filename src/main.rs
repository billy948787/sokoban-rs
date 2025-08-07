use sokoban_rs::{game, rendering::cli::CliFrontEnd};

fn main() {
    let mut game =
        game::Game::<CliFrontEnd>::new(game::GameState::from_file("levels/mission3.txt".into()));
    game.run();
}
