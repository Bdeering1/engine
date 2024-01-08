use chess::Piece;

use crate::board::{Board, Move};
use std::time::Instant;

const CHECKMATE_VALUE: i32 = 50000;

pub struct DebugInfo {
    pub nodes: u32,
}

pub struct SearchContext {
    pub board: Board,
    best_move: Move,
    search_depth: u32,
    pub debug: DebugInfo,
}

impl SearchContext {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            best_move: Move::default(),
            search_depth: 0,
            debug: DebugInfo {nodes: 0},
        }
    }

    pub fn search(&mut self, ms_remaining: u32, ms_inc: u32, debug: bool) -> Move {
        // for tracking how much time has passed in the search [timer.elapsed();]
        self.debug.nodes = 0;
        let timer = Instant::now();

        self.search_depth = 1;
        loop {
            let score = self.nega_max(&timer, self.search_depth, i32::MIN + 1, i32::MAX);
            if debug {
                println!("info depth {} score cp {} pv {}", self.search_depth, score, self.best_move);
            }
            if timer.elapsed().as_millis() as u32 > ms_remaining / 60 {
                return self.best_move
            }

            self.search_depth += 1;
        }
    }

    fn nega_max(&mut self, timer: &Instant, depth: u32, mut alpha: i32, beta: i32) -> i32 {
        let is_root = depth == self.search_depth;
        if !is_root
            && (self.board.is_repeated()
            || self.board.is_insufficient_material()
            || self.board.is_fifty_move_draw()) {
            return 0;
        }

        if depth == 0 { return eval(&self.board); }

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
