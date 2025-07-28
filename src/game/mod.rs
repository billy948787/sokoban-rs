#[derive(Debug, Clone)]
pub struct GameState {
    pub player_position: (i32, i32),
    pub box_positions: Vec<(i32, i32)>,
    pub target_positions: Vec<(i32, i32)>,
    pub walls: Vec<(i32, i32)>,
    pub map_size: (i32, i32),
}
