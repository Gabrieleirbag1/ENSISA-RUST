use std::io::{self, Write};

// TP3 - Morpion : enum, match, Option et Result

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Player {
    X,
    O,
}

impl Player {
    fn symbol(self) -> char {
        // TODO 1 : renvoyer le symbole de ce joueur avec match.
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
        // todo!("symbole du joueur")
    }

    fn other(self) -> Player {
        // TODO 1 : retourner l’autre joueur avec match.
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
        // todo!("joueur suivant")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Occupied(Player),
}

impl Cell {
    fn symbol(self) -> char {
        // TODO 1 : Empty -> espace ; Occupied(player) -> symbole du joueur.
        match self {
            Cell::Empty => ' ',
            Cell::Occupied(player) => player.symbol(),
        }
        // todo!("symbole de la case")
    }
}

#[derive(Debug, PartialEq, Eq)]
enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

#[derive(Debug, PartialEq, Eq)]
enum MoveError {
    InvalidPosition,
    Occupied,
}

impl MoveError {
    fn message(&self) -> &'static str {
        match self {
            MoveError::InvalidPosition => "position invalide (utilisez A1 a C3)",
            MoveError::Occupied => "case deja occupee",
        }
    }
}

// Convertit une saisie du type "A1" en indices (ligne, colonne).
fn parse_move(input: &str) -> Result<(usize, usize), MoveError> {
    // TODO 2 : analyser la saisie et renvoyer Ok((ligne, colonne)) ou Err(...).
    let input = input.trim();
    
    if input.len() != 2 {
        return Err(MoveError::InvalidPosition);
    }
    let row = match input.chars().next().unwrap().to_ascii_uppercase() {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => return Err(MoveError::InvalidPosition),
    };
    let col = match input.chars().next().unwrap() {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        _ => return Err(MoveError::InvalidPosition),
    };
    Ok((row, col))
}

struct Board {
    grid: [[Cell; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Self {
            grid: [[Cell::Empty; 3]; 3],
        }
    }

    fn reinit(&mut self) {
        self.grid = [[Cell::Empty; 3]; 3];
    }

    fn print(&self) {
        println!("  {}", "-".repeat(13));
        for (i, row) in self.grid.iter().enumerate() {
            let letter = (b'A' + i as u8) as char;
            println!(
                "{} | {} | {} | {} |",
                letter,
                row[0].symbol(),
                row[1].symbol(),
                row[2].symbol()
            );
            println!("  {}", "-".repeat(13));
        }
        println!("    1   2   3");
    }

    fn play(&mut self, row: usize, col: usize, player: Player) -> Result<(), MoveError> {
        // TODO 3 : verifier les limites et l’occupation avant de placer le symbole.
        if row >= 3 || col >= 3 {
            return Err(MoveError::InvalidPosition);
        }
        else if self.grid[row][col] == Cell::Occupied(player) {
            return Err(MoveError::Occupied);
        }
        else if self.grid[row][col] == Cell::Empty {
            self.grid[row][col] = Cell::Occupied(player);
            return Ok(());
        }
        Err(MoveError::InvalidPosition)
    }

    fn matching_player(a: Cell, b: Cell, c: Cell) -> Option<Player> {
        // TODO 4 : Some(player) si les trois cases contiennent le même joueur.
        todo!("trois cases alignees")
    }

    fn winner(&self) -> Option<Player> {
        // TODO 4 : tester les 3 lignes, les 3 colonnes et les 2 diagonales.
        todo!("rechercher un vainqueur")
    }

    fn is_full(&self) -> bool {
        // TODO 4 : true si aucune case n’est Empty.
        todo!("plateau complet")
    }
}

struct TicTacToe {
    turn: u8,
    current_player: Player,
    board: Board,
}

impl TicTacToe {
    fn new() -> Self {
        Self {
            turn: 1,
            current_player: Player::X,
            board: Board::new(),
        }
    }

    fn reinit(&mut self) {
        self.turn = 1;
        self.current_player = Player::X;
        self.board.reinit();
    }

    fn state(&self) -> GameState {
        // TODO 5 : Won si vainqueur, sinon Draw si plateau plein,
        // sinon InProgress. Attention a l’ordre des tests.
        todo!("etat de la partie")
    }

    fn launch(&mut self) -> io::Result<()> {
        loop {
            // Une partie.
            loop {
                println!("{}", "*".repeat(20));
                println!(
                    "Tour {} : joueur {}",
                    self.turn,
                    self.current_player.symbol()
                );
                println!("{}", "*".repeat(20));
                self.board.print();

                print!("Votre coup (A1-C3) : ");
                io::stdout().flush()?;
                let mut input = String::new();
                if io::stdin().read_line(&mut input)? == 0 {
                    return Ok(()); // Fin de fichier / fermeture de stdin.
                }

                let result = parse_move(&input)
                    .and_then(|(row, col)| self.board.play(row, col, self.current_player));

                match result {
                    Err(error) => {
                        println!("Coup refuse : {}.\n", error.message());
                        // Le tour ne change pas apres un coup invalide.
                    }
                    Ok(()) => match self.state() {
                        GameState::InProgress => {
                            self.current_player = self.current_player.other();
                            self.turn += 1;
                        }
                        GameState::Won(player) => {
                            self.board.print();
                            println!("Le joueur {} a gagne !", player.symbol());
                            break;
                        }
                        GameState::Draw => {
                            self.board.print();
                            println!("Match nul !");
                            break;
                        }
                    },
                }
            }

            print!("Voulez-vous rejouer ? (Y/N) : ");
            io::stdout().flush()?;
            let mut answer = String::new();
            if io::stdin().read_line(&mut answer)? == 0 {
                break;
            }
            if answer.trim().eq_ignore_ascii_case("Y") {
                self.reinit();
            } else {
                break;
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut game = TicTacToe::new();
    game.launch()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_and_cell_symbols() {
        assert_eq!(Player::X.symbol(), 'X');
        assert_eq!(Player::O.other(), Player::X);
        assert_eq!(Cell::Empty.symbol(), ' ');
        assert_eq!(Cell::Occupied(Player::O).symbol(), 'O');
    }

    #[test]
    fn parse_move_accepts_valid_input() {
        assert_eq!(parse_move("A1"), Ok((0, 0)));
        assert_eq!(parse_move(" c3 \n"), Ok((2, 2)));
    }

    #[test]
    fn parse_move_rejects_invalid_input() {
        assert_eq!(parse_move("D1"), Err(MoveError::InvalidPosition));
        assert_eq!(parse_move("A4"), Err(MoveError::InvalidPosition));
        assert_eq!(parse_move("A10"), Err(MoveError::InvalidPosition));
    }

    #[test]
    fn play_rejects_occupied_cell() {
        let mut board = Board::new();
        assert_eq!(board.play(0, 0, Player::X), Ok(()));
        assert_eq!(board.play(0, 0, Player::O), Err(MoveError::Occupied));
        assert_eq!(board.grid[0][0], Cell::Occupied(Player::X));
    }

    #[test]
    fn play_rejects_out_of_bounds() {
        let mut board = Board::new();
        assert_eq!(board.play(3, 0, Player::X), Err(MoveError::InvalidPosition));
        assert_eq!(board.play(0, 3, Player::O), Err(MoveError::InvalidPosition));
    }

    #[test]
    fn winner_detects_horizontal_vertical_and_diagonal() {
        let mut board = Board::new();
        assert_eq!(board.winner(), None);
        for col in 0..3 {
            board.play(0, col, Player::X).unwrap();
        }
        assert_eq!(board.winner(), Some(Player::X));

        board.reinit();
        for row in 0..3 {
            board.play(row, 1, Player::O).unwrap();
        }
        assert_eq!(board.winner(), Some(Player::O));

        board.reinit();
        for i in 0..3 {
            board.play(i, i, Player::X).unwrap();
        }
        assert_eq!(board.winner(), Some(Player::X));
    }

    #[test]
    fn state_reports_draw() {
        let mut game = TicTacToe::new();
        use Cell::Occupied as O;
        game.board.grid = [
            [O(Player::X), O(Player::O), O(Player::X)],
            [O(Player::X), O(Player::O), O(Player::O)],
            [O(Player::O), O(Player::X), O(Player::X)],
        ];
        assert_eq!(game.state(), GameState::Draw);
    }

    #[test]
    fn state_prioritizes_victory_and_reset_works() {
        let mut game = TicTacToe::new();
        assert_eq!(game.state(), GameState::InProgress);
        for col in 0..3 {
            game.board.play(2, col, Player::O).unwrap();
        }
        assert_eq!(game.state(), GameState::Won(Player::O));
        game.reinit();
        assert_eq!(game.state(), GameState::InProgress);
        assert_eq!(game.current_player, Player::X);
        assert_eq!(game.turn, 1);
    }
}
