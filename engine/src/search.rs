use crate::{board::{Board, Move}, eval::evaluate, tt::{TranspositionTable, Bound}};
use std::{time::Instant, sync::{atomic::{AtomicBool, Ordering}, Arc}};

const CHECKMATE_VALUE: i32 = 50000;
const OUT_OF_TIME_VALUE: i32 = 77777;

#[derive(Clone, Debug)]
pub struct DebugInfo {
    pub nodes: u32,
}

#[derive(Clone)]
pub struct SearchContext {
    pub tt: Arc<TranspositionTable>,
    pub stop_search: Arc<AtomicBool>,

    pub board: Board,
    pub debug: DebugInfo,

    root_best_move: Move,
    search_depth: u8,
    strict_timing: bool,
    move_time: u32,
}

impl SearchContext {
    pub fn new() -> Self {
        Self {
            tt: Arc::new(TranspositionTable::default()),
            stop_search: Arc::new(AtomicBool::new(false)),

            board: Board::new(),
            debug: DebugInfo { nodes: 0 },

            root_best_move: Move::default(),
            search_depth: 0,
            strict_timing: false,
            move_time: 0,
        }
    }

    pub fn search(&mut self, move_time: u32, strict_timing: bool, _verbose: bool) -> Move {
        self.strict_timing = strict_timing;
        self.move_time = move_time;
        self.stop_search.store(false, Ordering::Relaxed);
        self.debug.nodes = 0;

        let timer = Instant::now();

        self.search_depth = 1;
        loop {
            let score = self.nega_max(&timer, self.search_depth, i32::MIN + 1, i32::MAX);
            let stop = self.stop_search.load(Ordering::Relaxed);

            if stop || timer.elapsed().as_millis() as u32 > move_time {
                return self.root_best_move;
            } else {
                println!("info depth {} score cp {} hashfull {} time {} pv {}",
                    self.search_depth,
                    score,
                    self.tt.hashfull(),
                    timer.elapsed().as_millis(),
                    self.root_best_move
                );
            }

            self.search_depth += 1;
        }
    }

    fn nega_max(&mut self, timer: &Instant, depth: u8, mut alpha: i32, beta: i32) -> i32 {
        if self.strict_timing && timer.elapsed().as_millis() as u32 > self.move_time {
            self.stop_search.store(true, Ordering::Relaxed);
            return OUT_OF_TIME_VALUE;
        }

        /* Probe Transposition Table */
        {
            let tt_entry = self.tt.get(self.board.hash()).borrow();
            if tt_entry.key == self.board.hash() && tt_entry.depth >= depth {
                match tt_entry.bound {
                    Bound::Exact => return tt_entry.score,
                    Bound::Upper if tt_entry.score <= alpha => return alpha,
                    Bound::Lower if tt_entry.score >= beta => return beta,
                    _ => ()
                }
            }
        }

        /* Non-Stalemate Draw Conditions */
        let is_root = depth == self.search_depth;
        if !is_root
            && (self.board.is_repeated()
            || self.board.is_insufficient_material()
            || self.board.is_fifty_move_draw()) {
            return 0;
        }

        /* Quiescence Search */
        if depth == 0 { return self.q_search(timer, alpha, beta); }

        /* Checkmate or Stalemate */
        let moves = self.board.moves();
        if moves.len() == 0 {
            return if self.board.checkers().popcnt() > 0 { CHECKMATE_VALUE } else { 0 }
        }

        /* Core Negamax Search */
        let alpha_orig = alpha;
        let mut best_move = Move::default();
        for mv in moves {
            self.debug.nodes += 1;

            self.board.make_move(mv);
            let score = -self.nega_max(timer, depth - 1, -beta, -alpha);
            self.board.undo_move();

            if self.stop_search.load(Ordering::Relaxed) { return OUT_OF_TIME_VALUE; }

            if score > alpha {
                alpha = score;
                best_move = mv;

                if score >= beta {
                    break;
                }

                if is_root { self.root_best_move = mv; }
            }
        }

        /* Update Transposition Table */
        let tt_bound =
            if alpha >= beta {
                Bound::Lower
            } else if alpha > alpha_orig {
                Bound::Exact
            } else {
                Bound::Upper
            };
        self.tt.insert(
            self.board.hash(),
            alpha,
            depth,
            tt_bound,
            best_move,
        );
        
        alpha
    }

    fn q_search(&mut self, timer: &Instant, mut alpha: i32, beta: i32) -> i32 {
        if self.strict_timing && timer.elapsed().as_millis() as u32 > self.move_time {
            self.stop_search.store(true, Ordering::Relaxed);
            return OUT_OF_TIME_VALUE;
        }

        /* Probe Transposition Table */
        {
            let tt_entry = self.tt.get(self.board.hash()).borrow();
            if tt_entry.key == self.board.hash() {
                match tt_entry.bound {
                    Bound::Exact => return tt_entry.score,
                    Bound::Upper if tt_entry.score <= alpha => return alpha,
                    Bound::Lower if tt_entry.score >= beta => return beta,
                    _ => ()
                }
            }
        }

        /* Non-Stalemate Draw Conditions */
        if self.board.is_repeated()
            || self.board.is_insufficient_material()
            || self.board.is_fifty_move_draw() {
            return 0;
        }

        /* Standing Pat */
        let score = evaluate(&self.board);
        if score >= beta { return beta; }
        let alpha_orig = alpha;
        if score > alpha { alpha = score; }

        /* Checkmate or Stalemate */
        let mut moves = self.board.moves();
        if moves.len() == 0 {
            return if self.board.checkers().popcnt() > 0 { CHECKMATE_VALUE } else { 0 }
        }
        self.board.filter_captures(&mut moves);

        /* Core Negamax Search */
        let mut best_move = Move::default();
        for mv in moves {
            self.board.make_move(mv);
            let score = -self.q_search(timer, -beta, -alpha);
            self.board.undo_move();

            if self.stop_search.load(Ordering::Relaxed) { return OUT_OF_TIME_VALUE; }

            if score > alpha {
                alpha = score;
                best_move = mv;

                if score >= beta {
                    return beta;
                }
            }
        }

        /* Update Transposition Table */
        let tt_bound =
            if alpha >= beta {
                Bound::Lower
            } else if alpha > alpha_orig {
                Bound::Exact
            } else {
                Bound::Upper
            };
        self.tt.insert(
            self.board.hash(),
            alpha,
            0,
            tt_bound,
            best_move,
        );
        
        alpha
    }
}

