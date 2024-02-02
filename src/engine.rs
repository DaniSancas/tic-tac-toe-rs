//! Module for processing game actions and results
use crate::display;
use crate::model::Board;

/// Process the next action from the player and update the game state accordingly
pub fn next_action(
    board: &mut Board,
    is_player_x_turn: &mut bool,
    game_is_over: &mut bool,
    winner: &mut Option<char>,
) {
    let mut valid_action = false;
    let player = if *is_player_x_turn { 'X' } else { 'O' };
    while !valid_action && !*game_is_over {
        // Inform the player of the current game state
        display::print_board(board);
        println!("Player {}, enter your action: ", display::colorize(player));

        // Get the input from the user and try to parse it to a valid number
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let clean_input: Option<usize> = clean_input(&input);

        // If the input is valid, check if the action is valid
        if clean_input.is_some() && is_valid_action(board, clean_input.unwrap()) {
            apply_action(clean_input, board, player, &mut valid_action);
            evaluate_game_state(board, player, game_is_over, winner);

            *is_player_x_turn = !*is_player_x_turn;
        }
    }
}

/// Evaluate the game state and update the `game_is_over` flag
fn evaluate_game_state(
    board: &mut Board,
    player: char,
    game_is_over: &mut bool,
    winner: &mut Option<char>,
) {
    if (board[0][0] == player && board[0][1] == player && board[0][2] == player)
        || (board[1][0] == player && board[1][1] == player && board[1][2] == player)
        || (board[2][0] == player && board[2][1] == player && board[2][2] == player)
        || (board[0][0] == player && board[1][0] == player && board[2][0] == player)
        || (board[0][1] == player && board[1][1] == player && board[2][1] == player)
        || (board[0][2] == player && board[1][2] == player && board[2][2] == player)
        || (board[0][0] == player && board[1][1] == player && board[2][2] == player)
        || (board[0][2] == player && board[1][1] == player && board[2][0] == player)
    {
        *game_is_over = true;
    }

    // If the game is over, set the winner
    // If the game is not over, check if it's a draw
    if *game_is_over {
        *winner = Some(player);
    } else {
        *game_is_over = board[0][0] != '1'
            && board[0][1] != '2'
            && board[0][2] != '3'
            && board[1][0] != '4'
            && board[1][1] != '5'
            && board[1][2] != '6'
            && board[2][0] != '7'
            && board[2][1] != '8'
            && board[2][2] != '9';
    }
}

/// Apply the action to the board and update the `valid_action` flag
fn apply_action(
    clean_input: Option<usize>,
    board: &mut Board,
    player: char,
    valid_action: &mut bool,
) {
    match clean_input.unwrap() {
        1 => board[0][0] = player,
        2 => board[0][1] = player,
        3 => board[0][2] = player,
        4 => board[1][0] = player,
        5 => board[1][1] = player,
        6 => board[1][2] = player,
        7 => board[2][0] = player,
        8 => board[2][1] = player,
        9 => board[2][2] = player,
        _ => println!("Invalid action!"),
    }
    *valid_action = true;
}

/// Clean the input from the user and try to get a valid number (1-9)
fn clean_input(input: &str) -> Option<usize> {
    let result = match input.trim().parse() {
        Ok(num) => {
            if num < 10 && num > 0 {
                Some(num)
            } else {
                None
            }
        }
        Err(_) => None,
    };

    if result.is_none() {
        println!("Input is not a valid number!");
    }

    result
}

/// The action is valid if the number is between 1 and 9 and the cell is not occupied
fn is_valid_action(board: &mut Board, input: usize) -> bool {
    let result = match input {
        1 => board[0][0] == '1',
        2 => board[0][1] == '2',
        3 => board[0][2] == '3',
        4 => board[1][0] == '4',
        5 => board[1][1] == '5',
        6 => board[1][2] == '6',
        7 => board[2][0] == '7',
        8 => board[2][1] == '8',
        9 => board[2][2] == '9',
        _ => false,
    };

    if !result {
        println!("Input is not a valid action!");
    }

    result
}
