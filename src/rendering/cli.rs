use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        style::Stylize,
    },
    layout::Rect,
    symbols::{block, border},
    text::{Line, Span, ToSpan},
    widgets::{Block, Paragraph, Table},
};

use crate::{input::InputEvent, rendering::FrontEnd};

pub struct CliFrontEnd {
    terminal: ratatui::DefaultTerminal,
}

impl FrontEnd for CliFrontEnd {
    fn render(&mut self, state: &crate::game::GameState) {
        self.terminal
            .draw(|frame| {
                let title = Line::raw("Sokoban");
                let instructions = Line::raw(" Move <WASD> or <Arrow Keys> Quit<Q> or <Esc>");
                let block = Block::bordered()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .border_set(border::THICK);

                let game_area = block.inner(frame.area());
                frame.render_widget(block, frame.area());

                let (map_rows, map_cols) = state.map_size;

                if game_area.width < map_cols as u16 || game_area.height < map_rows as u16 {
                    return; // Not enough space to render the game area
                }

                let mut game_table: Vec<Line> = Vec::with_capacity(map_rows as usize);

                for r in 0..map_rows {
                    let mut row_span: Vec<Span> = Vec::with_capacity(map_cols as usize);
                    for c in 0..map_cols {
                        let pos = (r, c);
                        let char = if pos == state.player_position {
                            'P'
                        } else if state.box_positions.contains(&pos) {
                            if state.target_positions.contains(&pos) {
                                '*'
                            } else {
                                '$'
                            }
                        } else if state.target_positions.contains(&pos) {
                            '.'
                        } else if state.walls.contains(&pos) {
                            '#'
                        } else {
                            ' '
                        };
                        row_span.push(Span::from(char.to_string()));
                    }
                    game_table.push(Line::from(row_span));
                }

                let game_widget = Paragraph::new(game_table);

                let centered_area = Rect {
                    x: game_area.x + (game_area.width - map_cols as u16) / 2,
                    y: game_area.y + (game_area.height - map_rows as u16) / 2,
                    width: map_cols as u16,
                    height: map_rows as u16,
                };

                frame.render_widget(game_widget, centered_area);
            })
            .unwrap();
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
