use crate::{
    input::{self, InputEvent},
    rendering::FrontEnd,
};

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
        let content = std::fs::read_to_string(file_path).expect("Failed to read game state file");
        let mut lines = content.lines();

        let (rows, cols) = {
            let first_line = lines
                .next()
                .expect("File is empty or missing map size line");
            let mut dims = first_line
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Invalid map dimension"));
            (
                dims.next().expect("Missing map rows"),
                dims.next().expect("Missing map cols"),
            )
        };

        let mut player_position = (0, 0);
        let mut box_positions = Vec::new();
        let mut target_positions = Vec::new();
        let mut walls = Vec::new();

        let mut rows = 0;
        let mut cols = 0;

        for (r, line_content) in lines.enumerate() {
            rows += 1;
            cols = (line_content.len() as i32).max(cols);
            for (c, char) in line_content.chars().enumerate() {
                let pos = (r as i32, c as i32);
                match char {
                    '/' => walls.push(pos),
                    '0' => player_position = pos,
                    '1' => box_positions.push(pos),
                    '2' => target_positions.push(pos),
                    '-' => { /* Road, do nothing */ }
                    _ => { /* Ignore other characters */ }
                }
            }
        }

        GameState {
            player_position,
            box_positions,
            target_positions,
            walls,
            map_size: (rows, cols),
        }
    }

    fn is_deadlock(&self) -> bool {
        // Check if the player is in a deadlock position

        false
    }

    pub fn is_solved(&self) -> bool {
        // Check if all boxes are on target positions
        self.box_positions
            .iter()
            .all(|pos| self.target_positions.contains(pos))
    }
}

pub struct Game<F: FrontEnd> {
    pub state: GameState,
    pub front_end: F,
    prev_states: Vec<GameState>,
    after_states: Vec<GameState>,
}

impl<F: FrontEnd> Game<F> {
    pub fn new(state: GameState) -> Self {
        Game {
            state,
            front_end: F::default(),
            prev_states: Vec::new(),
            after_states: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.front_end.render(&self.state);
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
                    input::InputEvent::Undo => {
                        // Implement undo logic
                        if let Some(last_state) = self.prev_states.pop() {
                            self.after_states.push(self.state.clone());
                            self.state = last_state;
                        }
                        continue;
                    }
                    input::InputEvent::Redo => {
                        // Implement redo logic
                        if let Some(next_state) = self.after_states.pop() {
                            self.prev_states.push(self.state.clone());
                            self.state = next_state;
                        }
                        continue;
                    }
                    input::InputEvent::Restart => {
                        // Reset the game state to the initial state
                        if let Some(initial_state) = self.prev_states.first() {
                            self.state = initial_state.clone();
                            self.prev_states.clear();
                            self.after_states.clear();
                        }
                        continue;
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
                    // Save the current state before making a move
                    self.after_states.clear();
                    self.prev_states.push(self.state.clone());
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
                            self.prev_states.push(self.state.clone());
                            self.after_states.clear();
                            // move the box
                            self.state.box_positions[box_index] = (next_row, next_col);
                            self.state.player_position = (player_row, player_col);
                        } else {
                            continue; // Invalid move, skip updating player position
                        }
                    } else {
                        self.prev_states.push(self.state.clone());
                        self.after_states.clear();
                        // Just move the player
                        self.state.player_position = (player_row, player_col);
                    }
                }
            }
        }
    }
}
