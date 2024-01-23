use std::fmt;
use std::str::FromStr;

use chess::{MoveGen, BitBoard, ChessMove, EMPTY, Piece, Square};

const MAX_PLY: usize = 128;

const LIGHT_SQUARES: BitBoard = BitBoard(0x55AA_55AA_55AA_55AA);
const DARK_SQUARES: BitBoard = BitBoard(0xAA55_AA55_AA55_AA55);

#[derive(Clone)]
pub struct Board {
    pub position: chess::Board,
    history: Vec<chess::Board>,
    reversible_counts: Vec<u8>,
}

impl Board {
    /// Returns a new instance of `Board` with the default position
    pub fn new() -> Self {
        let mut reversible_counts = Vec::with_capacity(MAX_PLY);
        reversible_counts.push(0);
        Self {
            position: chess::Board::default(),
            history: Vec::with_capacity(MAX_PLY),
            reversible_counts
        }
    }

    /// Returns a new instance of `Board` with the given fen position
    pub fn from_fen(fen: &str) -> Self {
        let tokens = fen.split(" ").collect::<Vec<&str>>();
        let mut reversible_counts = Vec::with_capacity(MAX_PLY);
        reversible_counts.push(tokens[4].parse().unwrap());
        Self {
            position: chess::Board::from_str(fen).unwrap(),
            history: Vec::with_capacity(MAX_PLY),
            reversible_counts,
        }
    }

    pub fn hash(&self) -> u64 {
        self.position.get_hash()
    }

    /// Returns all legal moves in the current position as an iterator
    pub fn moves(&self) -> MoveIterator {
        MoveIterator::new_legal(&self.position)
    }

    /// Filters a given `MoveIterator` to only include captures
    pub fn filter_captures(&self, moves: &mut MoveIterator) {
        let targets = self.position.color_combined(!self.position.side_to_move());
        moves.set_iterator_mask(*targets);
    }

    /// Filters a given `MoveIterator` to include all remaining moves
    ///
    /// Used in conjunction with `filter_captures`
    pub fn filter_remaining(&self, moves: &mut MoveIterator) {
        let targets = self.position.color_combined(!self.position.side_to_move()) ^ !EMPTY;
        moves.set_iterator_mask(targets);
    }

    /// Make a move on the board
    pub fn make_move(&mut self, mv: Move) {
        self.history.push(self.position);
        self.reversible_counts.push(
            if self.is_reversible(mv) {
                self.reversible_counts[self.reversible_counts.len() - 1] + 1
            } else {
                0
            }
        );
        self.position = self.position.make_move_new(mv);
    }
    
    /// Undo the most recent move
    pub fn undo_move(&mut self) {
        if let Some(pos) = self.history.pop() {
            self.position = pos;
            self.reversible_counts.pop();
        } else {
            panic!("Attempted to undo a move that doesn't exist!");
        }
    }

    /// Returns the current side to move
    pub fn side_to_move(&self) -> chess::Color {
        self.position.side_to_move()
    }

    /// Returns a bitboard of checkers against the current side to move
    pub fn checkers(&self) -> &BitBoard {
        self.position.checkers()
    }

    pub fn piece_on(&self, sq: Square) -> Option<Piece> {
        self.position.piece_on(sq)
    }

    /// Returns true if the current position matches a previous one
    pub fn is_repeated(&self) -> bool {
        for pos in &self.history[0..(self.history.len() - 1)] {
            if pos.get_hash() == self.position.get_hash() {
                return true 
            }
        }
        false
    }

    /// Returns true if the position is a draw by fifty move rule
    pub fn is_fifty_move_draw(&self) -> bool {
        self.reversible_counts[self.reversible_counts.len() - 1] >= 100 
    }

    /// Returns true is the position is guaranteed to be drawn by to insufficient material
    ///
    /// Included cases:
    ///
    /// king vs king
    /// king + knight vs king
    /// kings and bishops of the same colour
    pub fn is_insufficient_material(&self) -> bool {
        let kings = self.position.pieces(Piece::King);
        let all_pieces = self.position.combined() ^ kings;

        // if (non-king) pieces exist on both light and dark squares, no draw occurs
        if (all_pieces & LIGHT_SQUARES != EMPTY) && (all_pieces & DARK_SQUARES != EMPTY) {
            return false;
        }

        // king vs king
        if all_pieces == EMPTY {
            return true;
        }

        // king + knight vs king
        let knights = *self.position.pieces(Piece::Knight);
        if all_pieces == knights && knights.popcnt() == 1 {
            return true;
        }

        // kings and bishops of the same colour
        let bishops = *self.position.pieces(Piece::Bishop);
        if all_pieces == bishops {
            return true
        }

        false
    }

    /// Returns true if the neither side can force a win
    /// 
    /// Included cases:
    ///
    /// king vs king
    /// kings and two or less knights
    /// kings and bishops of the same color
    ///
    /// Edge cases:
    /// king + 2 knights vs king
    /// king + knight vs king + knight
    /// - both return true: no forced win is possible
    ///
    /// king + bishop vs king + knight
    /// - returns false: no forced win is possible, but ignoring this case speeds up the function
    pub fn is_insufficient_material_pseudo(&self) -> bool {
        let kings = self.position.pieces(Piece::King);

        if kings == self.position.combined() {
            return true;
        }

        // kings and two or less knights
        let all_pieces = self.position.combined() ^ kings;
        let knights = *self.position.pieces(Piece::Knight);
        if all_pieces == knights && knights.popcnt() <= 2 {
            return true 
        }

        // kings and bishops of the same colour
        let bishops = *self.position.pieces(Piece::Bishop);
        if all_pieces == bishops
            && ((bishops & LIGHT_SQUARES == EMPTY) || (bishops & DARK_SQUARES == EMPTY)) {
            return true
        }

        false
    }

    /// Returns true if the given move is reversible (not a pawn move or capture)
    ///
    /// *must* be called before a move is made
    fn is_reversible(&self, mv: Move) -> bool {
        self.position.piece_on(mv.get_source()) != Some(Piece::Pawn)
        && self.position.color_on(mv.get_dest()) != Some(!self.position.side_to_move())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.position)
    }
}

pub type MoveIterator = MoveGen;

pub type Move = ChessMove;

#[cfg(test)]
mod tests {
    use super::*;
    use chess::Square;

    #[test]
    fn test_make_undo_move() {
        let mut board = Board::new();
        board.make_move(Move::new(Square::E2, Square::E4, None));
        board.make_move(Move::new(Square::E7, Square::E5, None));
        board.undo_move();
        board.undo_move();
        assert!(board.position.is_sane());
        assert!(board.reversible_counts.pop() == Some(0));
    }

    #[test]
    fn test_is_repeated() {
        let mut board = Board::from_fen("Q6k/8/K7/8/8/8/8/8 b - - 0 1");
        board.make_move(Move::new(Square::H8, Square::H7, None));
        board.make_move(Move::new(Square::A8, Square::A7, None));
        board.make_move(Move::new(Square::H7, Square::H8, None));
        board.make_move(Move::new(Square::A7, Square::A8, None));
        assert!(board.is_repeated());
    }

    #[test]
    fn test_is_fifty_move_draw() {
        let mut board = Board::from_fen("R4K1k/8/8/8/8/8/8/8 w - - 99 80");
        board.make_move(Move::new(Square::F8, Square::F7, None));
        assert!(board.is_fifty_move_draw());

        let mut board = Board::from_fen("8/1R5p/6k1/8/8/8/1R4K1/8 w - - 99 60");
        board.make_move(Move::new(Square::B7, Square::H7, None));
        assert!(!board.is_fifty_move_draw());
    }

    #[test]
    fn test_is_insufficient_material() {
        // king vs king
        assert!(Board::from_fen("K6k/8/8/8/8/8/8/8 w - - 0 1").is_insufficient_material());
        // king + knight vs king
        assert!(Board::from_fen("4KN1k/8/8/8/8/8/8/8 w - - 0 1").is_insufficient_material());
        // kings and bishops of the same colour
        assert!(Board::from_fen("B1B1BK1k/8/8/8/8/8/8/8 w - - 0 1").is_insufficient_material());

        // king + 2 knights vs king
        assert!(!Board::from_fen("3KNN1k/8/8/8/8/8/8/8 w - - 0 1").is_insufficient_material());
        // king + opposite colour bishops
        assert!(!Board::from_fen("3KBB1k/8/8/8/8/8/8/8 w - - 0 1").is_insufficient_material());
    }

    #[test]
    fn test_is_insufficent_material_pseudo() {
        // king vs king
        assert!(Board::from_fen("K6k/8/8/8/8/8/8/8 w - - 0 1").is_insufficient_material_pseudo());
        // king + knight vs king
        assert!(Board::from_fen("4KN1k/8/8/8/8/8/8/8 w - - 0 1").is_insufficient_material_pseudo());
        // kings and bishops of the same colour
        assert!(Board::from_fen("B1B1BK1k/8/8/8/8/8/8/8 w - - 0 1").is_insufficient_material_pseudo());

        // king + 2 knights vs king
        // Note: this case differs from `test_is_insufficient_material`
        assert!(Board::from_fen("3KNN1k/8/8/8/8/8/8/8 w - - 0 1").is_insufficient_material_pseudo());
        // king + opposite colour bishops
        assert!(!Board::from_fen("3KBB1k/8/8/8/8/8/8/8 w - - 0 1").is_insufficient_material_pseudo());
    }
}
