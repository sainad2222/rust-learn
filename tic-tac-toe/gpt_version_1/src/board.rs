use crate::player::Player;

#[derive(Clone)]
pub struct Board {
    cells: Vec<Player>,
    rows: usize,
    cols: usize,
    winning_count: usize,
}

impl Board {
    pub fn new(rows: usize, cols: usize, winning_count: usize) -> Board {
        let cells = vec![Player::None; rows * cols];
        Board {
            cells,
            rows,
            cols,
            winning_count,
        }
    }

    pub fn print(&self) {
        let separator: String = ["-"; self.cols * 2 - 1].join("+");
        for i in 0..self.rows {
            let start = i * self.cols;
            let end = start + self.cols;
            let row = &self.cells[start..end];
            println!(
                " {} ",
                row.iter().map(|p| p.symbol()).collect::<String>()
            );
            if i < self.rows - 1 {
                println!("{}", separator);
            }
        }
    }

    fn is_winner_horizontal(&self, player: Player) -> bool {
        for i in 0..self.rows {
            for j in 0..self.cols - self.winning_count + 1 {
                let start = i * self.cols + j;
                let end = start + self.winning_count;
                if self.cells[start..end].iter().all(|&p| p == player) {
                    return true;
                }
            }
        }
        false
    }

    fn is_winner_vertical(&self, player: Player) -> bool {
        for i in 0..self.cols {
            for j in 0..self.rows - self.winning_count + 1 {
                let mut all_match = true;
                for k in 0..self.winning_count {
                    let index = (j + k) * self.cols + i;
                    if self.cells[index] != player {
                        all_match = false;
                        break;
                    }
                }
                if all_match {
                    return true;
                }
            }
        }
        false
    }

    fn is_winner_diagonal(&self, player: Player) -> bool {
        for i in 0..self.rows - self.winning_count + 1 {
            for j in 0..self.cols - self.winning_count + 1 {
                let mut all_match = true;
                for k in 0..self.winning_count {
                    let index = (i + k) * self.cols + j + k;
                    if self.cells[index] != player {
                        all_match = false;
                        break;
                    }
                }
                if all_match {
                    return true;
                }
            }
        }

        for i in 0..self.rows - self.winning_count + 1 {
            for j in self.winning_count - 1..self.cols {
                let mut all_match = true;
                for k in 0..self.winning_count {
                    let index = (i + k) * self.cols + j - k;
                    if self.cells[index] != player {
                        all_match = false;
                        break;
                    }
                }
                if all_match {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_winner(&self, player: Player) -> bool {
        self.is_winner_horizontal(player)
            || self.is_winner_vertical(player)
            || self.is_winner_diagonal(player)
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|&p| p != Player::None)
    }

    pub fn place_marker(&mut self, player: Player, row: usize, col: usize) -> bool {
        let index = row * self.cols + col;
        if self.cells[index] == Player::None {
            self.cells[index] = player;
            true
        } else {
            false
        }
    }

    pub fn get_winner(&self) -> Option<Player> {
        for &player in &[Player::One, Player::Two] {
            if self.is_winner(player) {
                return Some(player);
            }
        }
        None
    }

    pub fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        let mut empty_cells = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.cells[i * self.cols + j] == Player::None {
                    empty_cells.push((i, j));
                }
            }
        }
        empty_cells
    }
}
