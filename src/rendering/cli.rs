use crate::rendering::Renderer;

pub struct CliRenderer {}

impl Renderer for CliRenderer {
    fn render(&self, state: &crate::game::GameState) {
        let (row, col) = state.player_position;

        for i in 0..row {
            for j in 0..col {
                if state.walls.contains(&(i, j)) {
                    print!("#");
                } else if state.box_positions.contains(&(i, j)) {
                    print!("B");
                } else if state.target_positions.contains(&(i, j)) {
                    print!("T");
                } else if (i, j) == state.player_position {
                    print!("P");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
