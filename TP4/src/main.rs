use std::io::{self, Write};
use rand::{rngs::StdRng, RngExt, SeedableRng};

// TP4 - Morpion : traits, implementations et Box<dyn Strategy>


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Player {
    X,
    O,
}

impl Player {
    fn symbol(self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }

    fn other(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Occupied(Player),
}

impl Cell {
    fn symbol(self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::Occupied(player) => player.symbol(),
        }
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
    let input = input.trim().to_ascii_uppercase();
    let bytes = input.as_bytes();

    if bytes.len() != 2 {
        return Err(MoveError::InvalidPosition);
    }

    let row = match bytes[0] {
        b'A' => 0,
        b'B' => 1,
        b'C' => 2,
        _ => return Err(MoveError::InvalidPosition),
    };
    let col = match bytes[1] {
        b'1' => 0,
        b'2' => 1,
        b'3' => 2,
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
        if row >= 3 || col >= 3 {
            return Err(MoveError::InvalidPosition);
        }

        match self.grid[row][col] {
            Cell::Empty => {
                self.grid[row][col] = Cell::Occupied(player);
                Ok(())
            }
            Cell::Occupied(_) => Err(MoveError::Occupied),
        }
    }

    fn matching_player(a: Cell, b: Cell, c: Cell) -> Option<Player> {
        match (a, b, c) {
            (Cell::Occupied(p1), Cell::Occupied(p2), Cell::Occupied(p3))
                if p1 == p2 && p2 == p3 => Some(p1),
            _ => None,
        }
    }

    fn winner(&self) -> Option<Player> {
        for i in 0..3 {
            if let Some(player) = Self::matching_player(
                self.grid[i][0], self.grid[i][1], self.grid[i][2],
            ) {
                return Some(player);
            }
            if let Some(player) = Self::matching_player(
                self.grid[0][i], self.grid[1][i], self.grid[2][i],
            ) {
                return Some(player);
            }
        }

        if let Some(player) = Self::matching_player(
            self.grid[0][0], self.grid[1][1], self.grid[2][2],
        ) {
            return Some(player);
        }
        Self::matching_player(self.grid[0][2], self.grid[1][1], self.grid[2][0])
    }

    fn is_full(&self) -> bool {
        for row in &self.grid {
            for cell in row {
                if *cell == Cell::Empty {
                    return false;
                }
            }
        }
        true
    }

    // TP4 / Exercice 1 : lister les coups possibles sans modifier le plateau.
    fn available_moves(&self) -> Vec<(usize, usize)> {
        // TODO 1 : renvoyer les coordonnees de toutes les cases vides.
        todo!("lister les coups possibles")
    }
}

// TP4 / Exercice 2 : un contrat commun a tous les joueurs.
// Ok(None) signifie : fin de la saisie (EOF) ou aucun coup possible.
// Un Err represente ici uniquement une erreur d'entree-sortie.
trait Strategy {
    fn choose_move(&mut self, board: &Board, player: Player)
        -> io::Result<Option<(usize, usize)>>;

    fn name(&self) -> &'static str {
        "Strategie"
    }
}

struct HumanStrategy;

impl Strategy for HumanStrategy {
    fn choose_move(&mut self, _board: &Board, player: Player)
        -> io::Result<Option<(usize, usize)>>
    {
        // TODO 2 : saisir A1-C3, utiliser parse_move et recommencer si
        // la saisie est incorrecte. Ok(None) en cas d'EOF (read_line = 0).
        // Ne modifiez pas le plateau ici.
        todo!("strategie humaine")
    }

    fn name(&self) -> &'static str {
        "Humain"
    }
}

// rand fournit le generateur aleatoire : aucun algorithme maison.
// StdRng conserve un etat interne, ce qui illustre l'utilite de &mut self.
struct RandomStrategy {
    rng: StdRng,
}

impl RandomStrategy {
    fn new() -> Self {
        // Graine aleatoire : les parties varient a chaque execution.
        Self { rng: rand::make_rng() }
    }

    fn with_seed(seed: u64) -> Self {
        // Graine fixe : utile pour les tests unitaires.
        Self { rng: StdRng::seed_from_u64(seed) }
    }
}

impl Strategy for RandomStrategy {
    fn choose_move(&mut self, board: &Board, _player: Player)
        -> io::Result<Option<(usize, usize)>>
    {
        // TODO 3 : utiliser available_moves() et self.rng.random_range(...)
        // pour choisir une case libre. Renvoyer Ok(None) si aucune case libre.
        todo!("strategie aleatoire")
    }

    fn name(&self) -> &'static str {
        "Ordinateur aleatoire"
    }
}

// TODO 5 : creer ici SmartStrategy et son impl Strategy.


struct TicTacToe {
    turn: u8,
    current_player: Player,
    board: Board,
    strategies: [Box<dyn Strategy>; 2],
}

impl TicTacToe {
    fn new(x: Box<dyn Strategy>, o: Box<dyn Strategy>) -> Self {
        Self {
            turn: 1,
            current_player: Player::X,
            board: Board::new(),
            strategies: [x, o],
        }
    }

    fn reinit(&mut self) {
        self.turn = 1;
        self.current_player = Player::X;
        self.board.reinit();
    }

    fn state(&self) -> GameState {
        if let Some(player) = self.board.winner() {
            GameState::Won(player)
        } else if self.board.is_full() {
            GameState::Draw
        } else {
            GameState::InProgress
        }
    }

    // TP4 / Exercice 4 : appel polymorphe au joueur courant.
    fn request_move(&mut self) -> io::Result<Option<(usize, usize)>> {
        // TODO 4 : choisir strategies[0] si X, strategies[1] si O,
        // puis appeler choose_move(&self.board, self.current_player).
        todo!("appeler le bon objet trait")
    }

    fn launch(&mut self) -> io::Result<()> {
        loop {
            // Une partie.
            loop {
                println!("{}", "*".repeat(20));
                let index = if self.current_player == Player::X { 0 } else { 1 };
                println!(
                    "Tour {} : joueur {} ({})",
                    self.turn,
                    self.current_player.symbol(),
                    self.strategies[index].name(),
                );
                println!("{}", "*".repeat(20));
                self.board.print();

                let position = match self.request_move()? {
                    Some(position) => position,
                    None => return Ok(()), // EOF ou aucun coup possible.
                };
                let (row, col) = position;
                let result = self.board.play(row, col, self.current_player);

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
    println!("Choisissez un mode de jeu :");
    println!("  1 - Humain contre humain");
    println!("  2 - Humain contre ordinateur aleatoire");
    print!("Mode [2] : ");
    io::stdout().flush()?;
    let mut input = String::new();
    if io::stdin().read_line(&mut input)? == 0 {
        return Ok(());
    }

    // TODO 4 : creer un Box<dyn Strategy> a partir du mode choisi.
    // Pour le mode 1 : HumanStrategy ; pour le mode 2 : RandomStrategy::new().
    let opponent: Box<dyn Strategy> = todo!("choisir le type d'adversaire");
    let mut game = TicTacToe::new(Box::new(HumanStrategy), opponent);
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
        let mut game = TicTacToe::new(Box::new(HumanStrategy), Box::new(HumanStrategy));
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
        let mut game = TicTacToe::new(Box::new(HumanStrategy), Box::new(HumanStrategy));
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

    #[test]
    fn available_moves_lists_only_empty_cells() {
        let mut board = Board::new();
        assert_eq!(board.available_moves().len(), 9);
        board.play(1, 1, Player::X).unwrap();
        let positions = board.available_moves();
        assert_eq!(positions.len(), 8);
        assert!(!positions.contains(&(1, 1)));
    }

    #[test]
    fn random_strategy_selects_only_free_cells() {
        let mut board = Board::new();
        board.play(0, 0, Player::X).unwrap();
        board.play(1, 1, Player::O).unwrap();
        let mut strategy = RandomStrategy::with_seed(42);
        for _ in 0..40 {
            let position = strategy.choose_move(&board, Player::X).unwrap().unwrap();
            assert!(board.available_moves().contains(&position));
        }
        for (row, col) in board.available_moves() {
            board.play(row, col, Player::X).unwrap();
        }
        assert_eq!(strategy.choose_move(&board, Player::O).unwrap(), None);
    }

    #[test]
    fn trait_object_dispatches_to_the_current_player() {
        let mut game = TicTacToe::new(
            Box::new(RandomStrategy::with_seed(7)),
            Box::new(RandomStrategy::with_seed(123)),
        );
        let move_x = game.request_move().unwrap().unwrap();
        assert!(game.board.available_moves().contains(&move_x));
        game.current_player = Player::O;
        let move_o = game.request_move().unwrap().unwrap();
        assert!(game.board.available_moves().contains(&move_o));
    }
/*
    #[test]
    fn smart_strategy_wins_or_blocks() {
        let mut board = Board::new();
        board.play(0, 0, Player::X).unwrap();
        board.play(0, 1, Player::X).unwrap();
        let mut smart = SmartStrategy;
        assert_eq!(smart.choose_move(&board, Player::X).unwrap(), Some((0, 2)));
        assert_eq!(smart.choose_move(&board, Player::O).unwrap(), Some((0, 2)));
    }
*/
}
