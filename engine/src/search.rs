use crate::{board::{Board, Move}, eval::evaluate, tt::TranspositionTable};
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

    best_move: Move,
    search_depth: u32,
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

            best_move: Move::default(),
            search_depth: 0,
            strict_timing: false,
            move_time: 0,
        }
    }

    pub fn search(&mut self, move_time: u32, strict_timing: bool, verbose: bool) -> Move {
        self.strict_timing = strict_timing;
        self.move_time = move_time;
        self.stop_search.store(false, Ordering::Relaxed);
        self.debug.nodes = 0;

        let timer = Instant::now();

        self.search_depth = 1;
        loop {
            let score = self.nega_max(&timer, self.search_depth, i32::MIN + 1, i32::MAX);
            let stop = self.stop_search.load(Ordering::Relaxed);
            if verbose && !stop {
                println!("info depth {} score cp {} pv {}", self.search_depth, score, self.best_move);
            }
            if stop || timer.elapsed().as_millis() as u32 > move_time {
                return self.best_move
            }

            self.search_depth += 1;
        }
    }

    fn nega_max(&mut self, timer: &Instant, depth: u32, mut alpha: i32, beta: i32) -> i32 {
        if self.strict_timing && timer.elapsed().as_millis() as u32 > self.move_time {
            self.stop_search.store(true, Ordering::Relaxed);
            return OUT_OF_TIME_VALUE;
        }

        let is_root = depth == self.search_depth;
        if !is_root
            && (self.board.is_repeated()
            || self.board.is_insufficient_material()
            || self.board.is_fifty_move_draw()) {
            return 0;
        }

        if depth == 0 { return self.q_search(timer, alpha, beta); }

        let moves = self.board.moves();
        // let mut pv = Move::default();
        
        if moves.len() == 0 {
            return if self.board.checkers().popcnt() > 0 { CHECKMATE_VALUE } else { 0 }
        }

        // main search
        for cur_move in moves {
            //should make this only happen if in debug mode?
            self.debug.nodes += 1;

            self.board.make_move(cur_move);
            let score = -self.nega_max(timer, depth - 1, -beta, -alpha);
            self.board.undo_move();

            if self.stop_search.load(Ordering::Relaxed) { return OUT_OF_TIME_VALUE; }

            if score > alpha {
                alpha = score;
                // pv = cur_move;
                if is_root { self.best_move = cur_move; }
            }
            if score >= beta {
                break;
            }
        }

        i32::min(alpha, beta)
    }

    fn q_search(&mut self, timer: &Instant, mut alpha: i32, beta: i32) -> i32 {
        if self.strict_timing && timer.elapsed().as_millis() as u32 > self.move_time {
            self.stop_search.store(true, Ordering::Relaxed);
            return OUT_OF_TIME_VALUE;
        }

        if self.board.is_repeated()
            || self.board.is_insufficient_material()
            || self.board.is_fifty_move_draw() {
            return 0;
        }

        let score = evaluate(&self.board);

        if score >= beta { return beta; }
        if score > alpha { alpha = score; }

        let mut moves = self.board.moves();
        self.board.filter_captures(&mut moves);

        if moves.len() == 0 {
            return if self.board.checkers().popcnt() > 0 { CHECKMATE_VALUE } else { 0 }
        }

        for cur_move in moves {
            self.board.make_move(cur_move);
            let score = -self.q_search(timer, -beta, -alpha);
            self.board.undo_move();

            if self.stop_search.load(Ordering::Relaxed) { return OUT_OF_TIME_VALUE; }

            if score >= beta { return beta; }
            if score > alpha { alpha = score; }
        }

        alpha
    }
}

