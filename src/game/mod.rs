use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
    thread,
};

use rand::random_range;

use crate::{
    input::{self},
    rendering::FrontEnd,
};

#[derive(Debug, Clone)]
pub struct GameState {
    pub player_position: (i32, i32),
    pub box_positions: Vec<(i32, i32)>,
    pub target_positions: Vec<(i32, i32)>,
    pub walls: Vec<(i32, i32)>,
    pub map_size: (i32, i32),
    pub dead_pos: Vec<(i32, i32)>,
    pub route: Vec<(i32, i32)>,
}

impl GameState {
    pub fn from_file(file_path: std::path::PathBuf) -> Self {
        let content = std::fs::read_to_string(file_path).expect("Failed to read game state file");

        let lines = content.lines();

        let mut player_position = (0, 0);
        let mut box_positions = Vec::new();
        let mut target_positions = Vec::new();
        let mut walls = Vec::new();

        let mut rows = 0;
        let mut cols = 0;

        for (r, line_content) in lines.skip(1).enumerate() {
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

        let mut state = GameState {
            player_position,
            box_positions,
            target_positions,
            walls,
            map_size: (rows, cols),
            dead_pos: Vec::new(),
            route: Vec::new(),
        };

        state.generate_deadlock_positions();

        state.route = state.find_route_to_target(state.box_positions[0], state.target_positions[0]);

        state
    }

    pub fn random_generate(rows: i32, cols: i32) -> Self {
        let player_position = (random_range(0..rows), random_range(0..cols));
        let box_positions = vec![
            (random_range(0..rows), random_range(0..cols)),
            (random_range(0..rows), random_range(0..cols)),
        ];
        let target_positions = vec![
            (random_range(0..rows), random_range(0..cols)),
            (random_range(0..rows), random_range(0..cols)),
        ];
        let walls = vec![
            (random_range(0..rows), random_range(0..cols)),
            (random_range(0..rows), random_range(0..cols)),
        ];

        let mut state = GameState {
            player_position,
            box_positions,
            target_positions,
            walls,
            map_size: (rows, cols),
            dead_pos: Vec::new(),
            route: Vec::new(),
        };

        state.generate_deadlock_positions();

        state.route = state.find_route_to_target(state.box_positions[0], state.target_positions[0]);

        state
    }

    fn generate_deadlock_positions(&mut self) {
        let mut pos_map =
            HashMap::<(i32, i32), bool>::from_iter(self.walls.iter().map(|&pos| (pos, true)));
        let mut deadlock_positions = Vec::<(i32, i32)>::from_iter(self.walls.iter().cloned());
        let mut queue = VecDeque::<(i32, i32)>::new();

        for &pos in &self.target_positions {
            if !pos_map.contains_key(&pos) {
                queue.push_back(pos);
            }
            pos_map.insert(pos, true);
        }

        let (map_rows, map_cols) = self.map_size;

        while let Some(live_pos) = queue.pop_front() {
            let (r, c) = live_pos;

            if !pos_map.contains_key(&(r, c)) {
                // Mark as processed
                pos_map.insert((r, c), true);
            }

            for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (box_prev_pos_row, box_prev_pos_col) = (r + dr, c + dc);
                let (player_prev_pos_row, player_prev_pos_col) =
                    (box_prev_pos_row + dr, box_prev_pos_col + dc);

                if box_prev_pos_row < 0
                    || box_prev_pos_row >= map_rows
                    || box_prev_pos_col < 0
                    || box_prev_pos_col >= map_cols
                    || player_prev_pos_row < 0
                    || player_prev_pos_row >= map_rows
                    || player_prev_pos_col < 0
                    || player_prev_pos_col >= map_cols
                {
                    continue; // Out of bounds
                }

                if let Some(true) = pos_map.get(&(box_prev_pos_row, box_prev_pos_col)) {
                    continue; // Already processed
                }

                if !self.walls.contains(&(box_prev_pos_row, box_prev_pos_col))
                    && !self
                        .walls
                        .contains(&(player_prev_pos_row, player_prev_pos_col))
                {
                    queue.push_back((box_prev_pos_row, box_prev_pos_col));
                }
            }
        }

        for r in 0..map_rows {
            for c in 0..map_cols {
                if !pos_map.contains_key(&(r, c)) {
                    deadlock_positions.push((r, c)); // Add all unprocessed positions
                }
            }
        }

        self.dead_pos = deadlock_positions;
    }
    pub fn find_route_to_target(&self, start: (i32, i32), target: (i32, i32)) -> Vec<(i32, i32)> {
        let mut route = Vec::new();
        let mut visited = HashMap::<(i32, i32), bool>::new();
        let (map_rows, map_cols) = self.map_size;

        route.push(start);

        while let Some(current) = route.last() {
            if *current == target {
                break; // Reached the target
            }

            let (current_row, current_col) = *current;

            visited.insert(*current, true);

            let mut found_next = false;

            for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (next_row, next_col) = (current_row + dr, current_col + dc);
                let (demand_player_row, demand_player_col) = (current_row - dr, current_col - dc);

                if self.dead_pos.contains(&(next_row, next_col)) {
                    continue;
                }

                if next_row < 0
                    || next_row >= map_rows
                    || next_col < 0
                    || next_col >= map_cols
                    || demand_player_row < 0
                    || demand_player_row >= map_rows
                    || demand_player_col < 0
                    || demand_player_col >= map_cols
                    || self.walls.contains(&(next_row, next_col))
                    || self.walls.contains(&(demand_player_row, demand_player_col))
                    || visited.contains_key(&(next_row, next_col))
                {
                    continue; // Out of bounds or wall or already visited
                }
                found_next = true;
                route.push((next_row, next_col));
                break;
            }

            if !found_next {
                route.pop(); // Backtrack if no next position found
            }
        }
        route
    }

    fn check_valid(&self) -> bool {
        for box_pos in &self.box_positions {}

        true
    }

    pub fn is_deadlock(&self) -> bool {
        // Check if the player is in a deadlock position
        for pos in &self.box_positions {
            if self.dead_pos.contains(pos) {
                return true; // Found a deadlock position
            }
        }

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

                            self.state.route = self.state.find_route_to_target(
                                self.state.box_positions[0],
                                self.state.target_positions[0],
                            );
                        } else {
                            continue; // Invalid move, skip updating player position
                        }
                    } else {
                        self.prev_states.push(self.state.clone());
                        self.after_states.clear();
                        // Just move the player
                        self.state.player_position = (player_row, player_col);
                        self.state.route = self.state.find_route_to_target(
                            self.state.box_positions[0],
                            self.state.target_positions[0],
                        );
                    }
                }
            }
        }
    }
}
