# Tic Tac Toe Game

## Description
This is a simple implementation of the classic Tic Tac Toe game in Rust 🦀.

It's a two-player game, where each player takes turns marking spaces in a 3x3 grid. 

The player who succeeds in placing three of their marks in a horizontal, vertical, or diagonal row is the winner.

## Installation
1. Clone this repository: `git clone https://github.com/DaniSancas/tic-tac-toe-rs`
2. Navigate into the directory: `cd tic-tac-toe-rs`
3. Build options:
   1. install locally: `make install`
   2. build without installing: `make build`

## Usage
To start the game (needs locally installation or build): `make run`

Note: If the local installation was made, it can be executed directly by calling `tic-tac-toe-rs`.

## Rules
1. The game is played on a grid that's 3 squares by 3 squares.
2. This implementation needs two players in the same device to play the game. Players (X and O) take turns putting their marks in empty squares.
3. The first player to get 3 of her marks in a row (up, down, across, or diagonally) is the winner.
4. When all 9 squares are full, the game is over. If no player has 3 marks in a row, the game ends in a tie.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT License](LICENSE)