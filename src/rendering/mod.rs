use crate::{game::GameState, input::InputEvent};

pub mod cli;

pub trait FrontEnd: Default {
    fn render(&mut self, state: &GameState);
    fn get_input(&self) -> Option<InputEvent>;
}
