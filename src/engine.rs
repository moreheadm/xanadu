//use shakmaty::{Chess, Move, Position, Setup, Color, Material, MaterialSide,
//        Outcome, Piece, Role, Board};
use chess::{Board, ChessMove, Square, Piece, BitBoard};
use rand::prelude::*;
use arrayvec::ArrayVec;

//https://www.chessprogramming.org/Simplified_Evaluation_Function

const KING_VALUE: [i32; 64] =
    [ 20, 30, 10,  0,  0, 10, 30, 20,
      20, 20,  0,  0,  0,  0, 20, 20,
     -10,-20,-20,-20,-20,-20,-20,-10,
     -20,-30,-30,-40,-40,-30,-30,-20,
     -30,-40,-40,-50,-50,-40,-40,-30,
     -30,-40,-40,-50,-50,-40,-40,-30,
     -30,-40,-40,-50,-50,-40,-40,-30,
     -30,-40,-40,-50,-50,-40,-40,-30];


const QUEEN_VALUE: [i32; 64] =
    [-20,-10,-10, -5, -5,-10,-10,-20,
     -10,  0,  5,  0,  0,  0,  0,-10,
     -10,  5,  5,  5,  5,  5,  0,-10,
       0,  0,  5,  5,  5,  5,  0, -5,
      -5,  0,  5,  5,  5,  5,  0, -5,
     -10,  0,  5,  5,  5,  5,  0,-10,
     -10,  0,  0,  0,  0,  0,  0,-10,
     -20,-10,-10, -5, -5,-10,-10,-20];

const ROOK_VALUE: [i32; 64] =
    [  0,  0,  0,  5,  5,  0,  0,  0,
      -5,  0,  0,  0,  0,  0,  0, -5,
      -5,  0,  0,  0,  0,  0,  0, -5,
      -5,  0,  0,  0,  0,  0,  0, -5,
      -5,  0,  0,  0,  0,  0,  0, -5,
      -5,  0,  0,  0,  0,  0,  0, -5,
       5, 10, 10, 10, 10, 10, 10, 10,
       0,  0,  0,  0,  0,  0,  0,  0];

const KNIGHT_VALUE: [i32; 64] =
    [-50,-40,-30,-30,-30,-30,-40,-50,
     -40,-20,  0,  0,  0,  0,-20,-40,
     -30,  0, 10, 15, 15, 10,  0,-30,
     -30,  5, 15, 20, 20, 15,  5,-30,
     -30,  5, 15, 20, 20, 15,  5,-30,
     -30,  0, 10, 15, 15, 10,  0,-30,
     -40,-20,  0,  0,  0,  0,-20,-40,
     -50,-40,-30,-30,-30,-30,-40,-50];

const BISHOP_VALUE: [i32; 64] =
    [-20,-10,-10,-10,-10,-10,-10,-20,
     -10,  5,  0,  0,  0,  0,  0,-10,
     -10, 10, 10, 10, 10, 10, 10,-10,
     -10,  0, 10, 10, 10, 10,  0,-10,
     -10,  5,  5, 10, 10,  5,  5,-10,
     -10,  0,  5, 10, 10,  5,  0,-10,
     -10,  0,  0,  0,  0,  0,  0,-10,
     -20,-10,-10,-10,-10,-10,-10,-20];

const PAWN_VALUE: [i32; 64] =
    [  0,  0,  0,  0,  0,  0,  0,  0,
       5, 10, 10,-20,-20, 10, 10,  5,
       5, -5,-10,  0,  0,-10, -5,  5,
       0,  0,  0, 20, 20,  0,  0,  0,
       5,  5, 10, 25, 25, 10,  5,  5,
      10, 10, 20, 30, 30, 20, 10, 10,
      50, 50, 50, 50, 50, 50, 50, 50,
       0,  0,  0,  0,  0,  0,  0,  0];

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SearchType {
    Infinite,
    Timed,
}

pub struct Engine {
    position: Board,
    active: bool,
    mode: SearchType,
    best_move: Option<ChessMove>,
    position_count: i64,
}

trait Evaluate {
    fn evaluate(&self) -> i32;
}

fn mat_score(mat_side: &MaterialSide) -> i32 {
    mat_side.pawns as i32 * 100
        + mat_side.knights as i32 * 300
        + mat_side.bishops as i32 * 300
        + mat_side.rooks as i32 * 500
        + mat_side.queens as i32 * 900
}

struct Pieces(&Board);

impl IntoIterator for Pieces {
    type Item = (Square, Piece, Color);
    type IntoIter = ArrayVec<[Self::Item; 32]>::IntoIter;

    fn into_iter(&self) -> Self::IntoIter {
        let v = Self::IntoIter::new();
        let bb = self.0.combined();
        for square in ALL_SQUARES {
            match board.piece_on(square) {
                Some(piece) => v.push((square, piece, board.color_on(square).unwrap())),
                None => {}
            }
        }

        v
    }
}

#[inline]
fn mirror(square: Square) {
    Square(0x38 ^ square.0)
}

fn loc_score(board: &Board) -> i32 {
    let mut total_score: i32 = 0;
    for (square, piece, color) in Pieces(board) {
        //eprintln!("{} {:?} {:?}", square, piece, piece.color);

        let idx = match color {
            Color::Black => mirror(square.to_index()),
            Color::White => square.to_index(),
        };

        let score = match piece.role {
            Piece::Pawn => PAWN_VALUE[idx],
            Piece::Knight => KNIGHT_VALUE[idx],
            Piece::Bishop => BISHOP_VALUE[idx],
            Piece::Rook => ROOK_VALUE[idx],
            Piece::Queen => QUEEN_VALUE[idx],
            Piece::King => KING_VALUE[idx],
        };

        //eprintln!("Idx {}  Color {:?}  Score {}\n", idx, piece, score);

        total_score += match color {
            Color::White => score,
            Color::Black => -score,
        };
    }
    total_score
}

impl Evaluate for Chess {
    fn evaluate(&self) -> i32 {
        let board = self.board();
        let material = board.material();
        //eprintln!("{}\n{}", material.white, material.black);
        self::mat_score(&material.white) - self::mat_score(&material.black) + self::loc_score(board)
    }
}

impl Engine {
    pub fn set_position(&mut self, position: Board) {
        self.position = position;
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.set_search_type(SearchType::Timed);
        self.best_move = None
    }

    fn random_search(&mut self) {
        let moves = self.position.legals();

        if self.best_move.is_none() {
            self.best_move =
                Some(moves[(thread_rng().gen::<f64>() * moves.len() as f64) as usize].clone());
        }
    }

    pub fn run_search(&mut self) {
        let upper = i32::max_value();
        let lower = i32::min_value();

        if self.best_move.is_none() {
            self.position_count = 0;
            let (mov, eval) = match self.position.turn() {
                Color::White => self.searchmax(self.position.clone(), upper, lower, 4),
                Color::Black => self.searchmin(self.position.clone(), upper, lower, 4),
            };
            eprintln!("Position count: {}", self.position_count);

            match mov {
                Some(mov) => self.best_move = Some(mov),
                None => {
                    self.best_move = Some(self.position.legals()[0].clone())
                    //eprintln!("Expected move");
                    //panic!();
                }
            }
        }
    }

    fn searchmax(&mut self, position: Chess, mut upper: i32, mut lower: i32, depth: i32)
                 -> (Option<Move>, i32) {
        self.position_count += 1;
        eprintln!("{} {:?}", depth, position.turn());
        match position.outcome() {
            Some(Outcome::Decisive { winner: Color::White }) => return (None, i32::max_value()),
            Some(Outcome::Decisive { winner: Color::Black }) => return (None, i32::min_value()),
            Some(Outcome::Draw) => return (None, 0),
            None => (),
        }

        if depth == 0 { return (None, position.evaluate()); }

        let moves = position.legals();

        let mut best_move: Option<Move> = None;

        for mov in moves {
            eprintln!("Depth: {}  Move: {}", depth, mov);
            let mut next_position = position.clone();
            next_position.play_unchecked(&mov);

            let eval = self.searchmin(next_position, upper, lower, depth - 1).1;
            if depth >= 0 { eprintln!("max Move: {}   Eval: {}", &mov, eval); }

            if eval >= upper {
                return (None, upper);
            }

            if eval > lower {
                lower = eval;
                best_move = Some(mov);
            }
        }

        (best_move, lower)
    }

    fn searchmin(&mut self, position: Chess, mut upper: i32, mut lower: i32, depth: i32)
                 -> (Option<Move>, i32) {
        self.position_count += 1;
        eprintln!("{} {:?}", depth, position.turn());
        match position.outcome() {
            Some(Outcome::Decisive { winner: Color::White }) =>
                return (None, i32::max_value()),
            Some(Outcome::Decisive { winner: Color::Black }) =>
                return (None, i32::min_value()),
            Some(Outcome::Draw) => return (None, 0),
            None => (),
        }

        if depth == 0 {
            //eprintln!("{}", position.evaluate());
            return (None, position.evaluate());
        }

        let moves = position.legals();

        let mut best_move: Option<Move> = None;

        for mov in moves {
            eprintln!("Depth: {}  Move: {}", depth, mov);
            let mut next_position = position.clone();
            next_position.play_unchecked(&mov);

            let eval = self.searchmax(next_position, upper, lower, depth - 1).1;
            if depth >= 0 { eprintln!("min Move: {}   Eval: {}", &mov, eval); }

            if eval <= lower {
                return (None, lower);
            }

            if eval < upper {
                upper = eval;
                best_move = Some(mov)
            }
        }

        (best_move, upper)
    }

    pub fn set_search_type(&mut self, mode: SearchType) {
        self.mode = mode;
    }

    pub fn get_search_type(&self) -> SearchType {
        self.mode
    }

    pub fn get_best_move(&self) -> Option<&Move> {
        self.best_move.as_ref()
    }

    pub fn get_current_position(&self) -> &Chess {
        &self.position
    }
}

impl Default for Engine {
    fn default() -> Engine {
        Engine {
            position: Chess::default(),
            active: false,
            mode: SearchType::Timed,
            best_move: None,
            position_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shakmaty::*;

    #[test]
    fn test_evaluation() {
        let mut pos = Chess::default();
        assert_eq!(0, loc_score(pos.board()));

        pos = pos
            .play(&Move::Normal {
                role: Role::Pawn,
                from: Square::E2,
                to: Square::E4,
                capture: None,
                promotion: None,
            })
            .expect("Test move fail");

        assert_eq!(40, loc_score(pos.board()));

        pos = pos
            .play(&Move::Normal {
                role: Role::Pawn,
                from: Square::E7,
                to: Square::E5,
                capture: None,
                promotion: None,
            })
            .expect("Test move fail");

        pos = pos
            .play(&Move::Normal {
                role: Role::Pawn,
                from: Square::F2,
                to: Square::F4,
                capture: None,
                promotion: None,
            })
            .expect("Test move fail");

        pos = pos
            .play(&Move::Normal {
                role: Role::Pawn,
                from: Square::E5,
                to: Square::F4,
                capture: Some(Role::Pawn),
                promotion: None,
            })
            .expect("Test move fail");

        assert_eq!(0, loc_score(pos.board()));
        assert_eq!(-100, pos.evaluate());
    }
}
