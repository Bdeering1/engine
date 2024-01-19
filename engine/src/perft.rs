use crate::board::Board;

pub fn perft(board: &mut Board, depth: u8) -> usize {
    let mut nodes = 0;
    let moves = board.moves();

    if depth == 1 {
        return moves.len();
    }

    for mv in moves {
        board.make_move(mv);
        nodes += perft(board, depth - 1);
        board.undo_move()
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perft_startpos() {
        let mut board = Board::new();
        assert_eq!(perft(&mut board, 1), 20);
        assert_eq!(perft(&mut board, 2), 400);
        assert_eq!(perft(&mut board, 3), 8902);
        assert_eq!(perft(&mut board, 4), 197281);
    }

    #[test]
    fn test_perft_position_2() {
        let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        assert_eq!(perft(&mut board, 1), 48);
        assert_eq!(perft(&mut board, 2), 2039);
        assert_eq!(perft(&mut board, 3), 97862);
        assert_eq!(perft(&mut board, 4), 4085603);
    }

    #[test]
    fn test_perft_position_3() {
        let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
        assert_eq!(perft(&mut board, 1), 14);
        assert_eq!(perft(&mut board, 2), 191);
        assert_eq!(perft(&mut board, 3), 2812);
        assert_eq!(perft(&mut board, 4), 43238);
    }

    #[test]
    fn test_perft_position_4() {
        let mut board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
        assert_eq!(perft(&mut board, 1), 6);
        assert_eq!(perft(&mut board, 2), 264);
        assert_eq!(perft(&mut board, 3), 9467);
        assert_eq!(perft(&mut board, 4), 422333);
    }
}
