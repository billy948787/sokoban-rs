use sokoban_rs::{
    game::{self, input::CliInputProvider},
    rendering::cli::CliRenderer,
};

fn main() {
    let mut game = game::Game::<CliRenderer, CliInputProvider>::new(game::GameState::from_file(
        "levels/mission1.txt".into(),
    ));
    game.run();
}
