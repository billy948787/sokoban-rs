use ratatui::crossterm::event::{self, Event, KeyCode};
pub enum InputEvent {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
}

pub trait InputHandler {
    fn handle_input(&mut self, event: InputEvent);
}

pub trait InputProvider: Default {
    fn get_input(&self) -> Option<InputEvent>;
}

pub struct CliInputProvider {}

impl InputProvider for CliInputProvider {
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

impl Default for CliInputProvider {
    fn default() -> Self {
        ratatui::crossterm::terminal::enable_raw_mode().unwrap();

        CliInputProvider {}
    }
}

impl Drop for CliInputProvider {
    fn drop(&mut self) {
        ratatui::crossterm::terminal::disable_raw_mode().unwrap();
    }
}
