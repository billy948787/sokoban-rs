use crate::game::GameState;

pub mod cli;

pub trait Renderer {
    fn render(&self, state: &GameState);
}
