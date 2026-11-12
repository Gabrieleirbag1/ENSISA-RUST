use std::collections::HashMap;

const MOVES: [(&str, (i32, i32)); 9] = [
    ("A1", (0, 0)),
    ("A2", (0, 1)),
    ("A3", (0, 2)),
    ("B1", (1, 0)),
    ("B2", (1, 1)),
    ("B3", (1, 2)),
    ("C1", (2, 0)),
    ("C2", (2, 1)),
    ("C3", (2, 2)),
];

const TURN: i32 = 1;
const BOARD: [[i32; 3]; 3] = [[0; 3]; 3];
const CHAR: [&str; 3] = [" ", "X", "O"];
const PLAYERS: [&str; 2] = ["O", "X"];


fn main() {
    println!("Hello, world!");
}
