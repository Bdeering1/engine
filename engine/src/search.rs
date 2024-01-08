use chess::Piece;

use crate::board::{Board, Move};
use std::time::Instant;

const CHECKMATE_VALUE: i32 = 50000;
const OUT_OF_TIME_VALUE: i32 = 77777;

pub struct DebugInfo {
    pub nodes: u32,
}

pub struct SearchContext {
    pub board: Board,
    best_move: Move,
    search_depth: u32,

    pub strict_timing: bool,
    move_time: u32,
    stop_search: bool,

    pub debug: DebugInfo,
}

impl SearchContext {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            best_move: Move::default(),
            search_depth: 0,

            strict_timing: false,
            move_time: 0,
            stop_search: false,

            debug: DebugInfo {nodes: 0},
        }
    }

    pub fn search(&mut self, move_time: u32, strict_timing: bool, verbose: bool) -> Move {
        self.strict_timing = strict_timing;
        self.move_time = move_time;
        self.stop_search = false;
        self.debug.nodes = 0;

        let timer = Instant::now();

        self.search_depth = 1;
        loop {
            let score = self.nega_max(&timer, self.search_depth, i32::MIN + 1, i32::MAX);
            if verbose && !self.stop_search {
                println!("info depth {} score cp {} pv {}", self.search_depth, score, self.best_move);
            }
            if timer.elapsed().as_millis() as u32 > move_time {
                return self.best_move
            }

            self.search_depth += 1;
        }
    }

    fn nega_max(&mut self, timer: &Instant, depth: u32, mut alpha: i32, beta: i32) -> i32 {
        if self.strict_timing && timer.elapsed().as_millis() as u32 > self.move_time {
            self.stop_search = true;
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

            if self.stop_search { return OUT_OF_TIME_VALUE; }

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
            self.stop_search = true;
            return OUT_OF_TIME_VALUE;
        }

        if self.board.is_repeated()
            || self.board.is_insufficient_material()
            || self.board.is_fifty_move_draw() {
            return 0;
        }

        let score = eval(&self.board);

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

            if self.stop_search { return OUT_OF_TIME_VALUE; }

            if score >= beta { return beta; }
            if score > alpha { alpha = score; }
        }

        alpha
    }
}

/// !TODO this is a temporary eval implementation for testing
fn eval(board: &Board) -> i32 {
    let mut score = 0;

    let pos = board.position;
    let our_pieces = pos.color_combined(pos.side_to_move());
    let their_pieces = pos.color_combined(!pos.side_to_move());

    score += (pos.pieces(Piece::Pawn) & our_pieces).popcnt() as i32 * 100;
    score += (pos.pieces(Piece::Knight) & our_pieces).popcnt() as i32 * 300;
    score += (pos.pieces(Piece::Bishop) & our_pieces).popcnt() as i32 * 300;
    score += (pos.pieces(Piece::Rook) & our_pieces).popcnt() as i32 * 500;
    score += (pos.pieces(Piece::Queen) & our_pieces).popcnt() as i32 * 900;

    score -= (pos.pieces(Piece::Pawn) & their_pieces).popcnt() as i32 * 100;
    score -= (pos.pieces(Piece::Knight) & their_pieces).popcnt() as i32 * 300;
    score -= (pos.pieces(Piece::Bishop) & their_pieces).popcnt() as i32 * 300;
    score -= (pos.pieces(Piece::Rook) & their_pieces).popcnt() as i32 * 500;
    score -= (pos.pieces(Piece::Queen) & their_pieces).popcnt() as i32 * 900;

    score
}
