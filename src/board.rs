use std::str::FromStr;

use chess::{MoveGen, ChessMove, EMPTY};

pub struct Board {
    internal: chess::Board,
}

impl Board {
    pub fn new() -> Self {
        let board = chess::Board::default();
        Self {
            internal: board
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        let board = chess::Board::default();
        Self {
            internal: chess::Board::from_str(fen).unwrap()
        }
    }

    pub fn moves(&self) -> MoveIterator {
        MoveIterator::new_legal(&self.internal)
    }

    pub fn filter_captures(&self, moves: &mut MoveIterator) {
        let targets = self.internal.color_combined(!self.internal.side_to_move());
        moves.set_iterator_mask(*targets);
    }

    pub fn filter_remaining(&self, moves: &mut MoveIterator) {
        let targets = self.internal.color_combined(!self.internal.side_to_move()) ^ !EMPTY;
        moves.set_iterator_mask(targets);
    }
}

pub type MoveIterator = MoveGen;

pub type Move = ChessMove;
