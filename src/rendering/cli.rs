use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::{input::InputEvent, rendering::FrontEnd};

pub struct CliFrontEnd {
    terminal: ratatui::DefaultTerminal,
}

impl FrontEnd for CliFrontEnd {
    fn render(&mut self, state: &crate::game::GameState) {
        self.terminal.draw(|frame| {
            frame.render_widget("hello world", frame.area());
        });
    }
    fn get_input(&self) -> Option<InputEvent> {
        // first check if there is an event available
        // so we write Ok to the event::poll function
        if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
            // if there is an event, read it
            if let Ok(Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('w') => return Some(InputEvent::MoveUp),
                    KeyCode::Down | KeyCode::Char('s') => return Some(InputEvent::MoveDown),
                    KeyCode::Left | KeyCode::Char('a') => return Some(InputEvent::MoveLeft),
                    KeyCode::Right | KeyCode::Char('d') => return Some(InputEvent::MoveRight),
                    KeyCode::Esc | KeyCode::Char('q') => return Some(InputEvent::Quit),
                    _ => {}
                }
            }
        }
        None
    }
}

impl Default for CliFrontEnd {
    fn default() -> Self {
        let terminal = ratatui::init();
        CliFrontEnd { terminal }
    }
}

impl Drop for CliFrontEnd {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
