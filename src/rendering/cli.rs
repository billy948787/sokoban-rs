use crate::rendering::Renderer;

pub struct CliRenderer {}

impl Renderer for CliRenderer {
    fn render(&self, state: &crate::game::GameState) {
        // Clear the console (this is platform dependent, here we use a simple method)
        print!("\x1B[2J\x1B[1;1H");
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

impl Default for CliRenderer {
    fn default() -> Self {
        CliRenderer {}
    }
}
