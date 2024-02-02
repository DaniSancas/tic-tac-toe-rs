//! Module for displaying the game board
use crate::model::Board;
use colored::Colorize;

/// Print the game board to the console
/// The player's character should be colored
pub fn print_board(board: &Board) {
    println!();
    println!("┌───┬───┬───┐");
    println!(
        "│ {} │ {} │ {} │",
        colorize(board[0][0]),
        colorize(board[0][1]),
        colorize(board[0][2])
    );
    println!("├───┼───┼───┤");
    println!(
        "│ {} │ {} │ {} │",
        colorize(board[1][0]),
        colorize(board[1][1]),
        colorize(board[1][2])
    );
    println!("├───┼───┼───┤");
    println!(
        "│ {} │ {} │ {} │",
        colorize(board[2][0]),
        colorize(board[2][1]),
        colorize(board[2][2])
    );
    println!("└───┴───┴───┘");
}

/// Print the winner to the console
/// The player's character should be colored
pub fn print_winner(winner: Option<char>) {
    match winner {
        Some(player) => println!("Player {} wins!", colorize(player)),
        _ => println!("It's a draw!"),
    }
}

/// Colorize the player's character
/// Player X should be colored red
/// Player O should be colored blue
pub fn colorize(character: char) -> colored::ColoredString {
    match character {
        'X' => character.to_string().red(),
        'O' => character.to_string().blue(),
        _ => character.to_string().normal(),
    }
}
