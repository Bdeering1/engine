use crate::board::{Board, Move};
use std::time::Instant;

pub fn search(board: &mut Board, ms_remaining: u32, ms_inc: u32) -> Move {

    //for tracking how much time has passed in the search [timer.elapsed();]
    let timer = Instant::now();

    nega_max(board, &timer, 0, i32::MIN, i32::MAX);


    Move::default()
}

fn nega_max(board: &mut Board, timer: &Instant, depth: u32, mut alpha: i32, mut beta: i32) -> i32 {
    let moves = board.moves();
    let mut pv = Move::default();

    //main search
    for cur_move in moves {
        board.make_move(cur_move);
        let score = -nega_max(board, timer, depth + 1, -beta, -alpha);
        board.undo_move();

        if score > alpha {
            alpha = score;
            pv = cur_move;
        }
    }

    i32::min(alpha, beta)
}

fn eval(board: &Board) {
    todo!();
}