use crate::{board::Move, eval::Eval};

#[repr(u8)]
pub enum Bound {
    Upper,
    Lower,
    Exact,
}

struct Transposition {
    pub hash: u64,
    pub score: Eval,
    pub depth: u8,
    pub bound: Bound,
    pub best_move: Move,
}

struct TranspositionTable {
    entries: Vec<Transposition>
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self {
            entries: vec![]
        }
    }
}

impl TranspositionTable {
    pub fn resize() {
        todo!();
    }

    pub fn clear() {
        todo!();
    }

    pub fn get() -> Transposition {
        todo!();
    }

    pub fn insert() {
        todo!();
    }
}
