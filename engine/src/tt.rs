use std::{mem::size_of, cell::RefCell};

use chess::{Square, Piece};

use crate::{board::Move, eval::Eval};

#[repr(u8)]
#[derive(Clone, Copy, Default)]
pub enum Bound {
    #[default]
    Upper,
    Lower,
    Exact,
}

#[derive(Clone, Copy, Default)]
pub struct CompactMove {
    data: u16,
}

impl From<Move> for CompactMove {
    fn from(mv: Move) -> Self {
        let to = mv.get_source().to_int() as u16;
        let from = mv.get_dest().to_int() as u16;
        let promotion =
            match mv.get_promotion() {
                Some(pc) => pc as u16,
                None => 0
            };
        Self {
            data: to | (from << 6) | (promotion << 12),
        }
    }
}

impl From<CompactMove> for Move {
    fn from(mv: CompactMove) -> Self {
        const TO_MASK: u16 = 0x3F;
        const FROM_MASK: u16 = 0x3F;
        const FROM_SHIFT: u16 = 6;
        const PROMO_MASK: u16 = 0xF;
        const PROMO_SHIFT: u16 = 12;

        let to_bits = mv.data & TO_MASK;
        let from_bits = (mv.data >> FROM_SHIFT) & FROM_MASK;
        let promo_bits = (mv.data >> PROMO_SHIFT) & PROMO_MASK;
        let promo =
            match promo_bits {
                1 => Some(Piece::Knight),
                2 => Some(Piece::Bishop),
                3 => Some(Piece::Rook),
                4 => Some(Piece::Queen),
                _ => None
            };
        unsafe { Move::new(Square::new(to_bits as u8), Square::new(from_bits as u8), promo) }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Entry {
    pub key: u64,               // 8 bytes
    pub score: Eval,            // 4 bytes
    pub depth: u8,              // 1 byte
    pub bound: Bound,           // 1 byte
    pub best_move: CompactMove, // 2 bytes
}

impl Entry {
    pub fn new(key: u64, score: Eval, depth: u8, bound: Bound, best_move: Move) -> Self {
        Self {
            key,
            score,
            depth,
            bound,
            best_move: best_move.into(),
        }
    }
}

#[derive(Default, Clone)]
pub struct TCell {
    inner: RefCell<Entry>
}

unsafe impl Sync for TCell { }

impl TCell {
    pub fn borrow(&self) -> std::cell::Ref<Entry> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<Entry> {
        self.inner.borrow_mut()
    }
}

pub struct TranspositionTable {
    entries: Vec<TCell>
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SIZE)
    }
}

impl TranspositionTable {
    pub const DEFAULT_SIZE: usize = 16;
    pub const ENTRY_SIZE: usize = size_of::<TCell>();

    fn index(&self, hash: u64) -> usize {
        (hash as usize) % self.entries.len()
    }

    pub fn new(size_mib: usize) -> Self {
        let mut tt = Self {
            entries: vec![]
        };
        tt.resize(size_mib);

        tt
    }

    pub fn resize(&mut self, size_mib: usize) {
        let entry_count = (size_mib << 20) / Self::ENTRY_SIZE;
        self.entries.resize(entry_count, TCell::default());
    }

    pub fn clear(&mut self) {
        self.entries.iter_mut().for_each(|e| *e = TCell::default());
    }

    pub fn get(&self, key: u64) -> &TCell {
        &self.entries[self.index(key)]
    }

    pub fn insert(&self, key: u64, score: Eval, depth: u8, bound: Bound, best_move: Move) {
        let index = self.index(key);
        *self.entries[index].borrow_mut() = Entry::new(key, score, depth, bound, best_move);
    }

    pub fn hashfull(&self) -> usize {
        self.entries.iter().take(1000).filter(|e| e.borrow().key != 0).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tt_new() {
        let tt = TranspositionTable::new(1);
        assert_eq!(tt_size(&tt), 1.0);
    }

    #[test]
    fn test_tt_resize() {
        let mut tt = TranspositionTable::new(1);
        tt.resize(2);
        assert_eq!(tt_size(&tt), 2.0);
        assert_eq!(tt.entries.capacity(), (2 << 20) / TranspositionTable::ENTRY_SIZE);
    }

    #[test]
    fn test_tt_clear() {
        let mut tt = TranspositionTable::new(1);
        tt.insert(1, 1, 1, Bound::Exact, Move::default());
        tt.insert(1, 1, 1, Bound::Exact, Move::default());
        tt.clear();
        assert_eq!(tt.hashfull(), 0);
    }

    #[test]
    fn test_tt_insert() {
        let tt = TranspositionTable::new(1);
        tt.insert(1, 1, 1, Bound::Exact, Move::default());
        assert_eq!(tt.get(1).borrow().key, 1);
        assert_eq!(tt.hashfull(), 1);
    }

    #[test]
    fn test_compact_move() {
        let mv = Move::new(Square::E2, Square::E4, None);
        let c_mv = CompactMove::from(mv);
        let mv_res = Move::from(c_mv);
        assert_eq!(mv, mv_res);
        let mv = Move::new(Square::H7, Square::H8, Some(Piece::Queen));
        let c_mv = CompactMove::from(mv);
        let mv_res = Move::from(c_mv);
        assert_eq!(mv, mv_res);
    }

    fn tt_size(tt: &TranspositionTable) -> f32 {
        ((tt.entries.len() * TranspositionTable::ENTRY_SIZE) as f32 / (1024 * 1024) as f32).round()
    }
}
