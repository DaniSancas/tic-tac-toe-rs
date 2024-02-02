//! Tic Tac Toe game in Rust 🦀
mod display;
mod engine;
mod model;

use model::Board;

fn main() {
    // Initialize the game state
    let mut board: Board = model::generate_board();
    let mut is_player_x_turn = true;
    let mut game_is_over = false;
    let mut winner: Option<char> = None;

    // Start the game loop
    while !game_is_over {
        engine::next_action(
            &mut board,
            &mut is_player_x_turn,
            &mut game_is_over,
            &mut winner,
        );
    }

    // Inform the player of the game result
    display::print_board(&board);
    display::print_winner(winner);
}
