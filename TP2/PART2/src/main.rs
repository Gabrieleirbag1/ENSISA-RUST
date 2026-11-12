use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

const CHAR: [&str; 3] = [" ", "X", "O"];
const PLAYERS: [&str; 2] = ["O", "X"];

struct Board {
    positions: [[usize; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Board {
            positions: [[0; 3]; 3],
        }
    }

    fn printboard(&self) {
        let line: [&str; 3] = ["A", "B", "C"];
        println!("  {}", "-".repeat(13));
        for i in 0..3 {
            println!(
                "{} | {} | {} | {} |",
                line[i],
                CHAR[self.positions[i][0]],
                CHAR[self.positions[i][1]],
                CHAR[self.positions[i][2]]
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

    fn clear_screen(&self) {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }
}

struct TicTacToe {
    board: Board,
    turn: i32,
    moves: HashMap<&'static str, (usize, usize)>,
}

impl TicTacToe {
    fn new() -> Self {
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
        TicTacToe {
            board: Board::new(),
            turn: 1,
            moves: moves,
        }
    }

    fn check_victory(&self) -> bool {
        let positions = &self.board.positions;
        for i in 0..3 {
            if positions[i][0] != 0
                && positions[i][0] == positions[i][1]
                && positions[i][1] == positions[i][2]
                || positions[0][i] != 0
                    && positions[0][i] == positions[1][i]
                    && positions[1][i] == positions[2][i]
            {
                return true;
            }
        }
        if positions[0][0] != 0
            && positions[1][1] == positions[0][0]
            && positions[2][2] == positions[1][1]
            || positions[0][2] != 0
                && positions[1][1] == positions[0][2]
                && positions[2][0] == positions[1][1]
        {
            return true;
        }
        false
    }

    fn reset_game(&mut self) {
        self.board.positions = [[0; 3]; 3];
        self.turn = 1;
    }

    fn run(&mut self) {
        loop {
            println!("{}", "*".repeat(20));
            println!(
                "Turn: {} | Player: {}",
                self.turn,
                PLAYERS[(self.turn % 2) as usize]
            );
            println!("{}", "*".repeat(20));
            self.board.printboard();

            print!("Enter your move:");
            io::stdout().flush().unwrap();

            let mut player_move = String::new();
            io::stdin()
                .read_line(&mut player_move)
                .expect("Failed to read line");

            let player_move = player_move.trim().to_uppercase();

            if !self.moves.contains_key(player_move.as_str()) {
                println!("{} is not authorized move !", player_move);
                continue;
            }

            let coord = self.moves[player_move.as_str()];
            if self.board.positions[coord.0][coord.1] == 0 {
                self.board.positions[coord.0][coord.1] = 2 - (self.turn % 2) as usize;
            } else {
                println!("Square already user !");
                continue;
            }

            if self.turn >= 5 && self.check_victory() {
                self.board.clear_screen();
                println!("Player {} wins!", PLAYERS[(self.turn % 2) as usize]);
                self.board.printboard();
                break;
            } else if self.turn >= 9 {
                self.board.clear_screen();
                println!("The game is a draw !");
                self.board.printboard();
                break;
            }
            self.turn += 1;
            self.board.clear_screen();
        }
    }
}

fn main() {
    let mut game = TicTacToe::new();

    loop {
        game.run();

        let restart = loop {
            print!("Restart the game ? (Y/n): ");
            io::stdout().flush().unwrap();

            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");

            match answer.trim().to_lowercase().as_str() {
                "" | "y" | "yes" => break true, // empty = default "Y"
                "n" | "no" => break false,
                _ => game.board.clear_screen(),
            }
        };

        if !restart {
            break;
        }
        game.reset_game();
        game.board.clear_screen();
    }
}
