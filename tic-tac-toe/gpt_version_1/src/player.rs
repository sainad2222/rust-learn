#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    None,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl Player {
    pub fn symbol(&self) -> &str {
        match self {
            Player::None => " ",
            Player::One => "X",
            Player::Two => "O",
            Player::Three => "1",
            Player::Four => "2",
            Player::Five => "3",
            Player::Six => "4",
            Player::Seven => "5",
            Player::Eight => "6",
            Player::Nine => "7",
            Player::Ten => "8",
        }
    }
}
