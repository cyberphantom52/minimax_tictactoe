type Board = [[char; 3]; 3];
fn print_board(board: &Board) {
    for row in board {
        for val in row {
           print!("{} ", val);
        }
        println!("");
    }
}

fn all_marked(board: &Board) -> bool{
    for row in board {
        for val in row {
            if *val == '_' {
                return false;
            }
        }
    }
    return true;
}

fn check_winner(board: &Board) -> Option<char>{
    let mut winner = None;
    for i in 0..3{
        if board[0][i] == board[1][i] &&  board[1][i] == board[2][i] && board[0][i] != '_' {
            winner = Some(board[0][i]);
        }else if board[i][0] == board[i][1] &&  board[i][1] == board[i][2] && board[i][0] != '_'{
            winner = Some(board[i][0]);
        }
    }
    if ((board[0][0] == board[1][1] &&  board[1][1] == board[2][2])
    || (board[0][2] == board[1][1] &&  board[1][1] == board[2][0]))
    && board[1][1] != '_' {
        winner = Some(board[1][1]);
    }

    if all_marked(board){
        winner = Some('T');
    }
    return winner;
}

fn minimax(board: &mut Board, depth: u8, is_maximising: bool) -> i8 {
    match check_winner(board) {
        Some('X') => return 10,
        Some('O') => return -10,
        Some('T') => return 0,
        _ => {}
    }

    if is_maximising {
        let mut max = std::i8::MIN;
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == '_' {
                    board[i][j] = 'X';
                    max = std::cmp::max(minimax(board, depth + 1, false), max) - depth as i8;
                    board[i][j] = '_';
                }
            }
        }
        return max;
    }

    let mut min = std::i8::MAX;
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == '_' {
                board[i][j] = 'O';
                min = std::cmp::min(minimax(board, depth + 1, true), min) + depth as i8;
                board[i][j] = '_';
            }
        }
    }
    return min;
}

fn computer_turn(board: &mut Board){
    let mut best_score = std::i8::MIN;
    let (mut x, mut y) = (0, 0);
    for i in 0..3{
        for j in 0..3{
            if board[i][j] == '_' {
                board[i][j] = 'X';
                let score = minimax(board, 0, false);
                board[i][j] = '_';
                if score > best_score {
                    best_score = score;
                    (x, y) = (j, i);
                }
            }
        }
    }
    board[y][x] = 'X';
}

fn read_input() -> (u8, u8) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let mut split = input.split_whitespace();
    let x = split.next().unwrap().parse::<u8>().unwrap();
    let y = split.next().unwrap().parse::<u8>().unwrap();
    (x, y)
}

fn player_turn(board: &mut Board){
    loop {
        print_board(board);
        println!("Enter row and column : ");
        let pos: (u8, u8) = read_input();
        std::process::Command::new("clear").status().unwrap();
        if board[(pos.1 - 1) as usize][(pos.0 - 1) as usize] == '_' {
            board[(pos.1 - 1) as usize][(pos.0 - 1) as usize] = 'O';
            break;
        }
        println!("Invalid move!");
    }
}

fn main(){
    let mut board = [['_'; 3]; 3];
    
    while check_winner(&board) == None {
        player_turn(&mut board);
        computer_turn(&mut board);
    }
    
    print_board(&board);
    match check_winner(&board) {
        Some('X') => println!("Computer Wins!"),
        Some('O') => println!("Player Wins!"),
        _ => println!("It's a Tie!")
    }
}