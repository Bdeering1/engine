use std::mem::size_of;

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

pub struct TranspositionTable {
    entries: Vec<Transposition>
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SIZE)
    }
}

impl TranspositionTable {
    pub const DEFAULT_SIZE: usize = 16;
    pub const ENTRY_SIZE: usize = size_of::<Transposition>();

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
        self.entries.resize(entry_count, Transposition::default());
    }

    pub fn clear(&mut self) {
        self.entries.iter_mut().for_each(|e| *e = Transposition::default());
    }

    pub fn get(&self, key: u64) -> Transposition {
        self.entries[self.index(key)]
    }

    pub fn insert(&mut self, entry: Transposition) {
        let index = self.index(entry.key);
        self.entries[index] = entry;
    }

    pub fn size(&self) -> f32 {
        ((self.entries.len() * Self::ENTRY_SIZE) as f32 / (1024 * 1024) as f32).round()
    }
}
