use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

const CHAR: [&str; 3] = [" ", "X", "O"];
const PLAYERS: [&str; 2] = ["O", "X"];

fn printboard(board: &[[usize; 3]; 3]) {
    let line: [&str; 3] = ["A", "B", "C"];
    println!("  {}", "-".repeat(13));
    for i in 0..3 {
        println!(
            "{} | {} | {} | {} |",
            line[i], CHAR[board[i][0]], CHAR[board[i][1]], CHAR[board[i][2]]
        );
        println!("  {}", "-".repeat(13));
    }
    println!(
        "{}{}{}{}{}{}",
        " ".repeat(4),
        "1",
        " ".repeat(3),
        "2",
        " ".repeat(3),
        "3"
    );
}

fn check_victory(board: &[[usize; 3]; 3]) -> bool {
    for i in 0..3 {
        if board[i][0] != 0 && board[i][0] == board[i][1] && board[i][1] == board[i][2]
            || board[0][i] != 0 && board[0][i] == board[1][i] && board[1][i] == board[2][i]
        {
            return true;
        }
    }
    if board[0][0] != 0 && board[1][1] == board[0][0] && board[2][2] == board[1][1]
        || board[0][2] != 0 && board[1][1] == board[0][2] && board[2][0] == board[1][1]
    {
        return true;
    }
    false
}

fn clear_screen() {
    Command::new("clear")
        .status()
        .expect("Failed to clear screen");    
}

fn main() {
    let moves: HashMap<&str, (usize, usize)> = HashMap::from([
        ("A1", (0, 0)),
        ("A2", (0, 1)),
        ("A3", (0, 2)),
        ("B1", (1, 0)),
        ("B2", (1, 1)),
        ("B3", (1, 2)),
        ("C1", (2, 0)),
        ("C2", (2, 1)),
        ("C3", (2, 2)),
    ]);

    let mut turn: i32 = 1;
    let mut board: [[usize; 3]; 3] = [[0; 3]; 3];

    loop {
        println!("{}", "*".repeat(20));
        println!("Turn: {} | Player: {}", turn, PLAYERS[(turn % 2) as usize]);
        println!("{}", "*".repeat(20));
        printboard(&board);

        print!("Enter your move:");
        io::stdout().flush().unwrap();

        let mut player_move = String::new();
        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");

        let player_move = player_move.trim().to_uppercase();

        if !moves.contains_key(player_move.as_str()) {
            println!("{} is not authorized move !", player_move);
            continue;
        }

        let coord = moves[player_move.as_str()];
        if board[coord.0][coord.1] == 0 {
            board[coord.0][coord.1] = 2 - (turn % 2) as usize;
        } else {
            println!("Square already user !");
            continue;
        }

        if turn >= 5 && check_victory(&board) {
            clear_screen();
            println!("Player {} wins!", PLAYERS[(turn % 2) as usize]);
            printboard(&board);
            break;
        } else if turn >= 9 {
            clear_screen();
            println!("The game is a draw !");
            printboard(&board);
            break;
        }
        turn += 1;
        clear_screen();
    }
}
