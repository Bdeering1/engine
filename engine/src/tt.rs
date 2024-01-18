use std::{mem::size_of, cell::RefCell};

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
pub struct Transposition {
    pub key: u64,
    pub score: Eval,
    pub depth: u8,
    pub bound: Bound,
    pub best_move: Move,
}

impl Transposition {
    pub fn update(&mut self, key: u64, score: Eval, depth: u8, bound: Bound, best_move: Move) {
        self.key = key;
        self.score = score;
        self.depth = depth;
        self.bound = bound;
        self.best_move = best_move;

    }
}

#[derive(Default, Clone)]
pub struct TCell {
    inner: RefCell<Transposition>
}

unsafe impl Sync for TCell { }

impl TCell {
    pub fn borrow(&self) -> std::cell::Ref<Transposition> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<Transposition> {
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
        self.entries[index].borrow_mut().update(key, score, depth, bound, best_move);
    }

    pub fn size(&self) -> f32 {
        ((self.entries.len() * Self::ENTRY_SIZE) as f32 / (1024 * 1024) as f32).round()
    }

    pub fn hashfull(&self) -> usize {
        self.entries.iter().take(1000).filter(|e| e.borrow().key != 0).count()
    }
}
