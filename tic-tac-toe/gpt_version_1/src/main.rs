use std::io;

mod board;
mod player;

use board::Board;
use player::Player;

fn main() {
    let mut input = String::new();

    // Get number of players
    let num_players = loop {
        println!("Enter the number of players (minimum 2, maximum 10):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let num_players: usize = match input.trim().parse() {
            Ok(num) if num >= 2 && num <= 10 => num,
            _ => {
                println!("Invalid number of players. Please enter a number between 2 and 10.");
                continue;
            }
        };
        break num_players;
    };

    // Get board dimensions
    let rows = loop {
        println!("Enter the number of rows (minimum 3):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let rows: usize = match input.trim().parse() {
            Ok(num) if num >= 3 => num,
            _ => {
                println!(
                    "Invalid number of rows. Please enter a number greater than or equal to 3."
                );
                continue;
            }
        };
        break rows;
    };

    let cols = loop {
        println!("Enter the number of columns (minimum 3):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let cols: usize = match input.trim().parse() {
            Ok(num) if num >= 3 => num,
            _ => {
                println!(
                    "Invalid number of columns. Please enter a number greater than or equal to 3."
                );
                continue;
            }
        };
        break cols;
    };

    let mut board = Board::new(rows, cols);
    let mut players = Vec::new();

    // Initialize players
    for i in 0..num_players {
        let symbol = Player::from_index(i).symbol();
        players.push((symbol, Player::from_index(i + 1)));
    }

    let mut current_player_index = 0;

    loop {
        board.display();

        let current_player = &players[current_player_index];
        let (symbol, player) = current_player;

        println!("{}'s turn ({}):", symbol, player);

        let mut row_input = String::new();
        let mut col_input = String::new();

        // Get row input
        let row = loop {
            println!("Enter row number (1-{}):", rows);
            io::stdin()
                .read_line(&mut row_input)
                .expect("Failed to read line");
            let row: usize = match row_input.trim().parse() {
                Ok(num) if num >= 1 && num <= rows => num - 1,
                _ => {
                    println!(
                        "Invalid row number. Please enter a number between 1 and {}.",
                        rows
                    );
                    continue;
                }
            };
            break row;
        };

        // Get column input
        let col = loop {
            println!("Enter column number (1-{}):", cols);
            io::stdin()
                .read_line(&mut col_input)
                .expect("Failed to read line");
            let col: usize = match col_input.trim().parse() {
                Ok(num) if num >= 1 && num <= cols => num - 1,
                _ => {
                    println!(
                        "Invalid column number. Please enter a number between 1 and {}.",
                        cols
                    );
                    continue;
                }
            };
            break col;
        };

        if !board.is_valid_move(row, col) {
            println!("Invalid move. That spot is already taken.");
            continue;
        }

        board.make_move(row, col, symbol);

        if board.is_winner(row, col, symbol) {
            println!("{} wins!", symbol);
            break;
        }

        if board.is_full() {
            println!("Tie game!");
            break;
        }

        current_player_index = (current_player_index + 1) % num_players;
    }
}
