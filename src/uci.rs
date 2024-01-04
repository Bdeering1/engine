use std::io::stdin;
use std::str::FromStr;

use chess::BitBoard;

use crate::board::{Board, Move};

pub fn run_uci() {
    let mut board: Board;
    let mut debug = false;

    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let tokens = buf.trim().split(" ").collect::<Vec<&str>>();

        match tokens[0] {
            "uci" => {
                println!("name engine v0.0 author Bryn Deering");
                println!("uciok");
            }
            "debug" => {
                match tokens[1] {
                    "on" => debug = true,
                    "off" => debug = false,
                    _ => (),
                }
            },
            "isready" => {
                println!("readyok");
            },
            "setoption" => (),
            "ucinewgame" => (),
            "position" => {
                board = match tokens[1] {
                    "startpos" => {
                        Board::new()
                    },
                    fen => {
                        Board::from_fen(fen)
                    }
                };
                for idx in 2..tokens.len() {
                    let m = Move::from_str(tokens[idx]).unwrap();
                    board.make_move(m);
                }
                if debug {
                    println!("info string {}", board);
                }
            },
            "go" => (),
            "ponderhit" => (),
            "stop" => (),
            "quit" => break,
            _ => ()
        }
    }
}
