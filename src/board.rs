use std::fmt;
use std::str::FromStr;

use chess::{MoveGen, BitBoard, ChessMove, EMPTY, Piece};

const LIGHT_SQUARES: BitBoard = BitBoard(0x55AA_55AA_55AA_55AA);
const DARK_SQUARES: BitBoard = BitBoard(0xAA55_AA55_AA55_AA55);

pub struct Board {
    pub position: chess::Board,
    history: Vec<chess::Board>,
    reversible_counts: Vec<usize>,
}

impl Board {
    /// Returns a new instance of `Board` with the default position
    pub fn new() -> Self {
        Self {
            position: chess::Board::default(),
            history: vec![],
            reversible_counts: vec![ 0 ],
        }
    }

    /// Returns a new instance of `Board` with the given fen position
    pub fn from_fen(fen: &str) -> Self {
        Self {
            position: chess::Board::from_str(fen).unwrap(),
            history: vec![],
            reversible_counts: vec![ 0 ],
        }
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
        self.position = self.position.make_move_new(mv);

        self.reversible_counts.push(self.reversible_counts[self.reversible_counts.len() - 1] + 1);
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

    /// Returns a bitboard of checkers against the current side to move
    pub fn checkers(&self) -> &BitBoard {
        self.position.checkers()
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

    /// !TODO Returns true if the position should be considered drawn by insufficient material
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
    /// - both return true: neither position is a technically a draw but no forced win is possible
    ///
    /// king + bishop vs king + knight
    /// - returns false: no forced win is possible, but ignoring this case speeds up the function
    pub fn is_insufficient_material(&self) -> bool {
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

        // kings and bishops of the same color
        let bishops = *self.position.pieces(Piece::Bishop);
        if all_pieces == bishops
            && ((bishops & LIGHT_SQUARES == EMPTY) || (bishops & DARK_SQUARES == EMPTY)) {
            return true
        }

        false
    }

    /// Returns true if the given move is reversible (not a pawn move or capture)
    fn is_reversible(&self, mv: Move) -> bool {
        self.position.piece_on(mv.get_source()) == Some(Piece::Pawn)
        || self.position.color_on(mv.get_dest()) == Some(!self.position.side_to_move())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.position)
    }
}

pub type MoveIterator = MoveGen;

pub type Move = ChessMove;

