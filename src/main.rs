type Board = [[char; 3]; 3];

fn check_for_draw(board: &Board) -> bool {
    for row in board.iter() {
        for cell in row.iter() {
            if *cell == '\0' {
                return false;
            }
        }
    }
    println!("Match is a draw");
    true
}

fn game_over(board: &Board) -> bool {
    // check for draw
    if check_for_draw(board) {
        return true;
    }
    // check rows
    for i in 0..3 {
        if (board[i][0] == 'X' || board[i][0] == 'O')
            && (board[i][0] == board[i][1] && board[i][1] == board[i][2])
        {
            println!("Player {} wins!", board[i][0]);
            return true;
        }
    }
    // check cols
    for i in 0..3 {
        if (board[0][i] == 'X' || board[0][i] == 'O')
            && board[0][i] == board[1][i]
            && board[0][i] == board[2][i]
        {
            println!("Player {} wins!", board[i][0]);
            return true;
        }
    }
    // check diagonals
    if (board[0][0] == 'X' || board[0][0] == 'O')
        && board[0][0] == board[1][1]
        && board[1][1] == board[2][2]
    {
        println!("Player {} wins!", board[0][0]);
        return true;
    }
    false
}

fn draw_board(board: &Board) {
    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate() {
            if col_idx == row.len() - 1 {
                if board[row_idx][col_idx] == '\0' {
                    println!("_");
                } else {
                    println!("{}", board[row_idx][col_idx]);
                }
            } else {
                if board[row_idx][col_idx] == '\0' {
                    print!("_|");
                } else {
                    print!("{}|", board[row_idx][col_idx]);
                }
            }
        }
    }
}

fn play_game(board: &mut Board, choice: &mut bool) {
    use std::io::{stdin, stdout, Write};
    let mut input = String::new();
    print!(
        "Player {}, enter a row and column: ",
        if *choice { 'X' } else { 'O' }
    );
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();
    let row = split.next().unwrap().parse::<usize>().unwrap();
    let col = split.next().unwrap().parse::<usize>().unwrap();
    if row > 2 || col > 2 {
        println!("\nInvalid input, please try again");
        return;
    }
    if board[row][col] != '\0' {
        println!("That space is already taken!");
    } else {
        board[row][col] = if *choice { 'X' } else { 'O' };
        *choice = !*choice;
    }
}

fn main() {
    let mut board: Board = [['\0', '\0', '\0'], ['\0', '\0', '\0'], ['\0', '\0', '\0']];

    let mut choice = true;

    println!(
        "Welcome to Tic Tac Toe!\nThis is a 2 player game.\nPlayer 1 is X and Player 2 is O.\n"
    );

    loop {
        draw_board(&board);
        if game_over(&board) {
            break;
        } else {
            play_game(&mut board, &mut choice);
        }
    }
}
