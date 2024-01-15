use chess::Piece;

use crate::board::Board;

pub type Eval = i32;

/// !TODO this is a temporary eval implementation for testing
pub fn evaluate(board: &Board) -> Eval {
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
