use crate::game::GameState;

pub mod cli;

pub trait Renderer: Default {
    fn render(&self, state: &GameState);
}
