use std::fmt;
use std::str::FromStr;

use chess::{MoveGen, BitBoard, ChessMove, EMPTY, Piece};

const LIGHT_SQUARES: BitBoard = BitBoard(0x55AA_55AA_55AA_55AA);
const DARK_SQUARES: BitBoard = BitBoard(0xAA55_AA55_AA55_AA55);

pub struct Board {
    pub position: chess::Board,
    history: Vec<chess::Board>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            position: chess::Board::default(),
            history: vec![]
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        Self {
            position: chess::Board::from_str(fen).unwrap(),
            history: vec![],
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
    pub fn make_move(&mut self, m: Move) {
        self.history.push(self.position);
        self.position = self.position.make_move_new(m);
    }
    
    /// Undo the most recent move
    pub fn undo_move(&mut self) {
        if let Some(pos) = self.history.pop() {
            self.position = pos;
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

    /// !TODO Returns true if the position should be considered drawn by insufficient material
    /// 
    /// Included cases:
    ///
    /// king vs king
    /// kings and knights
    /// kings and bishops of the same color
    ///
    /// *kings and knights is not technically a draw but may be a helpful optimization
    pub fn is_insufficient_material(&self) -> bool {
        if self.position.pieces(Piece::King) == self.position.combined() {
            return true;
        }

        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.position)
    }
}

pub type MoveIterator = MoveGen;

pub type Move = ChessMove;
