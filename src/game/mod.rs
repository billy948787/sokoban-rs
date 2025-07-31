use crate::{input, rendering::FrontEnd};

#[derive(Debug, Clone)]
pub struct GameState {
    pub player_position: (i32, i32),
    pub box_positions: Vec<(i32, i32)>,
    pub target_positions: Vec<(i32, i32)>,
    pub walls: Vec<(i32, i32)>,
    pub map_size: (i32, i32),
}

impl GameState {
    pub fn from_file(file_path: std::path::PathBuf) -> Self {
        // Read the file and parse the game state
        // For now, we'll just return a default state
        GameState {
            player_position: (0, 0),
            box_positions: Vec::new(),
            target_positions: Vec::new(),
            walls: Vec::new(),
            map_size: (10, 10),
        }
    }
}

pub struct Game<F: FrontEnd> {
    pub state: GameState,
    pub front_end: F,
}

impl<F: FrontEnd> Game<F> {
    pub fn new(state: GameState) -> Self {
        Game {
            state,
            front_end: F::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Some(event) = self.front_end.get_input() {
                let (mut player_row, mut player_col) = self.state.player_position;

                match event {
                    input::InputEvent::MoveUp => {
                        player_row -= 1;
                    }
                    input::InputEvent::MoveDown => {
                        player_row += 1;
                    }
                    input::InputEvent::MoveLeft => {
                        player_col -= 1;
                    }
                    input::InputEvent::MoveRight => {
                        player_col += 1;
                    }
                    input::InputEvent::Quit => {
                        break; // Exit the game loop
                    }
                }

                // Check if the new position is valid
                if player_row >= 0
                    && player_row < self.state.map_size.0
                    && player_col >= 0
                    && player_col < self.state.map_size.1
                    && !self.state.walls.contains(&(player_row, player_col))
                {
                    // check if the new position has a box
                    if let Some(box_index) = self
                        .state
                        .box_positions
                        .iter()
                        .position(|&pos| pos == (player_row, player_col))
                    {
                        // if it has a box, check if the next position is valid
                        let next_row = player_row + (player_row - self.state.player_position.0);
                        let next_col = player_col + (player_col - self.state.player_position.1);

                        if next_row >= 0
                            && next_row < self.state.map_size.0
                            && next_col >= 0
                            && next_col < self.state.map_size.1
                            && !self.state.walls.contains(&(next_row, next_col))
                            && !self.state.box_positions.contains(&(next_row, next_col))
                        {
                            // move the box
                            self.state.box_positions[box_index] = (next_row, next_col);
                        } else {
                            continue; // Invalid move, skip updating player position
                        }
                    }
                    self.state.player_position = (player_row, player_col);
                }
            }

            self.front_end.render(&self.state);
        }
    }
}
