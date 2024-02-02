//! Module for having the game model

pub type Board = [[char; 3]; 3];

/// Generates a new board (3x3) for the game
pub fn generate_board() -> Board {
    [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']]
}
