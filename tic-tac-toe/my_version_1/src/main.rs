use std::io;


fn main() {
    let mut board = vec![["", "", ""], ["", "", ""], ["", "", ""]];
    let mut cntr = 0;
    loop {
        let (did_someone_won, who_won, is_it_a_draw) = evaluate_board(&board);
        if did_someone_won {
            println!("{} won", who_won);
            break;
        }
        if is_it_a_draw {
            println!("It's a draw");
            break;
        }
        let mut current_players_turn = "";
        if cntr % 2 == 0 {
            current_players_turn = "X";
        } else {
            current_players_turn = "O"
        }
        println!("{}'s turn", current_players_turn);
        print_board(&board);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.to_string().strip_suffix("\n").unwrap().to_string();
        let nums:Vec<_> = input.split(" ").collect();
        let x = nums[0].parse::<usize>().unwrap();
        let y = nums[1].parse::<usize>().unwrap();
        populate_turn(&mut board, (x, y), current_players_turn);
        cntr += 1;
    }
}

fn evaluate_board(board: &Vec<[&'static str; 3]>) -> (bool, &'static str, bool) {
    // check if someone won
    // // check if any one horizontal row is occupied
    for i in 0..3 {
        let cur_player = board[i][0];
        let mut is_satisfied = true;
        for j in 1..3 {
            if board[i][j] != cur_player {
                is_satisfied = false;
                break;
            }
        }
        if is_satisfied && cur_player != "" {
            return (true, cur_player, false);
        }
    }
    // // check if any one vertical row is occupied
    for j in 0..3 {
        let cur_player = board[0][j];
        let mut is_satisfied = true;
        for i in 1..3 {
            if board[i][j] != cur_player {
                is_satisfied = false;
                break;
            }
        }
        if is_satisfied && cur_player != "" {
            return (true, cur_player, false);
        }
    }
    // // check if diagonal rows are occupied
    let cur_player = board[0][0];
    let mut is_satisfied = true;
    for i in 1..3 {
        if board[i][i] != cur_player {
            is_satisfied = false;
            break;
        }
    }
    if is_satisfied && cur_player != "" {
        return (true, cur_player, false);
    }
    // // check if anti diagonal rows are occupied
    let cur_player = board[0][2];
    let mut is_satisfied = true;
    for i in 1..3 {
        if board[i][2 - i] != cur_player {
            is_satisfied = false;
            break;
        }
    }
    if is_satisfied && cur_player != "" {
        return (true, cur_player, false);
    }
    // // check if it's a draw
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == "" {
                return (false, "", false);
            }
        }
    }
    (false, "", true)
}

fn populate_turn(
    board: &mut Vec<[&'static str; 3]>,
    pos: (usize, usize),
    player_char: &'static str,
) {
    let (x, y) = pos;
    board[x][y] = player_char;
}

fn print_board(board: &Vec<[&str; 3]>) {
    for i in 0..3 {
        for j in 0..3 {
            let mut char = board[i][j];
            if char == "" {
                char = "_"
            }
            print!("{} ",char);
        }
        println!();
    }
}

// TODO to learn
// lifetimes, why it is expected 'static
