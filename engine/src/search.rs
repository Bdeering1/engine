use chess::Piece;

use crate::{board::{Board, Move}, eval::{evaluate, Eval}, tt::{TranspositionTable, Bound}};
use std::{time::Instant, sync::{atomic::{AtomicBool, Ordering}, Arc}};

const CHECKMATE_VALUE: Eval = 50000;
const OUT_OF_TIME_VALUE: Eval = 77777;
const FUTILITY_MARGIN: Eval = 200;

#[derive(Clone, Debug, Default)]
pub struct DebugInfo {
    pub nodes: u32,
    pub delta_pruned: u32,
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
            debug: DebugInfo::default(),

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
        let mut best_move = Move::default();

        self.search_depth = 1;
        loop {
            let score = self.nega_max(&timer, self.search_depth, i32::MIN + 1, i32::MAX);
            let stop = self.stop_search.load(Ordering::Relaxed);

            println!("info string nodes {} delta_pruned {}", self.debug.nodes, self.debug.delta_pruned);
            if stop || timer.elapsed().as_millis() as u32 > move_time {
                return best_move;
            }
            println!("info depth {} score cp {} hashfull {} time {} pv {}",
                self.search_depth,
                score,
                self.tt.hashfull(),
                timer.elapsed().as_millis(),
                self.root_best_move
            );

            best_move = self.root_best_move;
            self.search_depth += 1;
        }
    }

    fn nega_max(&mut self, timer: &Instant, depth: u8, mut alpha: i32, beta: i32) -> i32 {
        self.debug.nodes += 1;

        if self.strict_timing && timer.elapsed().as_millis() as u32 > self.move_time {
            self.stop_search.store(true, Ordering::Relaxed);
            return OUT_OF_TIME_VALUE;
        }

        let mut best_move = None;
        let is_root = depth == self.search_depth;

        /* Non-Stalemate Draw Conditions
         *
         * repeated positions and fifty move rule must be checked before checking
         * transpositions, otherwise they may be ignored
         * positions with insufficient material are not stored in the table
        */
        if !is_root {
            if self.board.is_repeated()
            || self.board.is_insufficient_material()
            || self.board.is_fifty_move_draw() {
                return 0;
            }

            /* Probe Transposition Table */
            let tt_entry = self.tt.get(self.board.hash()).borrow();
            if tt_entry.key == self.board.hash() && tt_entry.depth >= depth {
                match tt_entry.bound {
                    Bound::Exact => return tt_entry.score,
                    Bound::Upper if tt_entry.score <= alpha => return alpha,
                    Bound::Lower if tt_entry.score >= beta => return beta,
                    _ => ()
                }
                best_move = Some(tt_entry.best_move.into());
                if best_move.unwrap() == Move::default() {
                    best_move = None;
                }
            }
        }

        /* Quiescence Search */
        if depth == 0 { return self.q_search(timer, alpha, beta); }

        /* Checkmate or Stalemate */
        let moves = self.board.sorted_moves(best_move, false);
        if moves.len() == 0 {
            return if self.board.checkers().popcnt() > 0 { -CHECKMATE_VALUE + (self.search_depth - depth) as i32 } else { 0 }
        }

        /* Core Negamax Search */
        let alpha_orig = alpha;
        for mv in moves {
            self.board.make_move(mv);
            let score = -self.nega_max(timer, depth - 1, -beta, -alpha);
            self.board.undo_move();

            if self.stop_search.load(Ordering::Relaxed) { return OUT_OF_TIME_VALUE; }

            if score > alpha {
                alpha = score;
                best_move = Some(mv);

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
            best_move.unwrap_or_default(),
        );

        alpha
    }

    fn q_search(&mut self, timer: &Instant, mut alpha: i32, beta: i32) -> i32 {
        if self.strict_timing && timer.elapsed().as_millis() as u32 > self.move_time {
            self.stop_search.store(true, Ordering::Relaxed);
            return OUT_OF_TIME_VALUE;
        }

        /* Non-Stalemate Draw Conditions */
        if self.board.is_repeated()
            || self.board.is_insufficient_material()
            || self.board.is_fifty_move_draw() {
            return 0;
        }

        let mut best_move = None;

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
                best_move = Some(tt_entry.best_move.into());
                if best_move.unwrap() == Move::default() {
                    best_move = None;
                }
            }
        }

        /* Standing Pat */
        let static_eval = evaluate(&self.board);
        if self.board.checkers().popcnt() == 0 && static_eval >= beta { return beta; }
        let alpha_orig = alpha;
        if static_eval > alpha { alpha = static_eval; }

        /* Core Negamax Search */
        let moves = self.board.sorted_moves(best_move, true);
        for mv in moves {
            /* Futility (Delta) Pruning */
            let capture_val = match self.board.piece_on(mv.get_dest()) {
                Some(Piece::Pawn) => 100,
                Some(Piece::Knight) => 300,
                Some(Piece::Bishop) => 300,
                Some(Piece::Rook) => 500,
                Some(Piece::Queen) => 900,
                _ => 0, // case where non-capture is read from tt move
            };
            if static_eval + capture_val + FUTILITY_MARGIN < alpha {
                self.debug.delta_pruned += 1;
                continue;
            }

            self.board.make_move(mv);
            let score = -self.q_search(timer, -beta, -alpha);
            self.board.undo_move();

            if self.stop_search.load(Ordering::Relaxed) { return OUT_OF_TIME_VALUE; }

            if score > alpha {
                alpha = score;
                best_move = Some(mv);

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
            best_move.unwrap_or_default(),
        );
        
        alpha
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::Square;
    use crate::board::Board;

    #[test]
    fn test_checkmate_position() {
        let mut sc = SearchContext::new();
        sc.board = Board::from_fen("6k1/8/R5K1/8/8/8/8/8 w - - 0 1");
        let mv = sc.search(20, true, false);
        assert_eq!(mv, Move::new(Square::A6, Square::A8, None));
    }

    #[test]
    fn test_repeated_draw() {
        let mut sc = SearchContext::new();
        sc.board = Board::from_fen("r5k1/5p2/3n1QpK/8/8/8/8/8 w - - 0 1");
        sc.board.make_move(Move::new(Square::F6, Square::E7, None));
        sc.board.make_move(Move::new(Square::G8, Square::H8, None));
        sc.board.make_move(Move::new(Square::E7, Square::F6, None));
        sc.board.make_move(Move::new(Square::H8, Square::G8, None));
        sc.board.make_move(Move::new(Square::F6, Square::E7, None));
        sc.board.make_move(Move::new(Square::G8, Square::H8, None));
        let mv = sc.search(20, true, false);
        // white cannot play Qe7 (attempting M2) to avoid a draw
        assert_ne!(mv, Move::new(Square::F6, Square::E7, None));
    }

    #[test]
    fn test_fifty_move_draw() {
        let mut sc = SearchContext::new();
        sc.board = Board::from_fen("8/1R5p/6k1/8/8/8/1R4K1/8 w - - 99 60");
        let mv = sc.search(20, true, false);
        // white must sacrifice material to avoid a draw
        assert_eq!(mv, Move::new(Square::B7, Square::H7, None));
    }

    #[test]
    fn test_insufficient_material() {
        let mut sc = SearchContext::new();
        sc.board = Board::from_fen("5Nbk/4KP2/8/8/8/8/8/8 w - - 0 1");
        let mv = sc.search(20, true, false);
        // white must not trade to avoid a draw
        assert_eq!(mv, Move::new(Square::F8, Square::G6, None));
    }
}
