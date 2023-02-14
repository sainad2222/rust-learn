use std::io;

#[derive(PartialEq, Clone, Copy)]
enum Player {
    X,
    O,
    None,
}

impl Player {
    fn symbol(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
            Player::None => ' ',
        }
    }
    fn toggle(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
            _ => Player::None,
        }
    }
}

struct Board {
    cells: [Player; 9],
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [Player::None; 9],
        }
    }

    fn print(&self) {
        println!(
            " {} | {} | {}",
            self.cells[0].symbol(),
            self.cells[1].symbol(),
            self.cells[2].symbol()
        );
        println!("-----------");
        println!(
            " {} | {} | {}",
            self.cells[3].symbol(),
            self.cells[4].symbol(),
            self.cells[5].symbol()
        );
        println!("-----------");
        println!(
            " {} | {} | {}",
            self.cells[6].symbol(),
            self.cells[7].symbol(),
            self.cells[8].symbol()
        );
    }

    fn is_winner(&self, player: Player) -> bool {
        for i in 0..3 {
            if self.cells[i * 3] == player
                && self.cells[i * 3 + 1] == player
                && self.cells[i * 3 + 2] == player
            {
                return true;
            }
            if self.cells[i] == player && self.cells[i + 3] == player && self.cells[i + 6] == player
            {
                return true;
            }
        }
        if self.cells[0] == player && self.cells[4] == player && self.cells[8] == player {
            return true;
        }
        if self.cells[2] == player && self.cells[4] == player && self.cells[6] == player {
            return true;
        }
        false
    }

    fn is_full(&self) -> bool {
        for &cell in self.cells.iter() {
            if cell == Player::None {
                return false;
            }
        }
        true
    }

    fn is_game_over(&self, player: Player) -> bool {
        self.is_winner(player) || self.is_full()
    }

    fn place_marker(&mut self, player: Player, position: usize) -> bool {
        if self.cells[position] == Player::None {
            self.cells[position] = player;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::X;

    loop {
        board.print();
        println!(
            "Player {}'s turn. Enter a number (1-9) to place your marker.",
            current_player.symbol()
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let position: usize = match input.trim().parse::<usize>() {
            Ok(num) => num - 1,
            Err(_) => {
                println!("Invalid input, please enter a number from 1-9.");
                continue;
            }
        };
        if position > 8 {
            println!("Invalid input, please enter a number from 1-9.");
            continue;
        }
        if board.place_marker(current_player, position) {
            if board.is_winner(current_player) {
                board.print();
                println!("Player {} wins!", current_player.symbol());
                break;
            }
            if board.is_full() {
                board.print();
                println!("It's a tie!");
                break;
            }
        }
        current_player = current_player.toggle();
    }
}
