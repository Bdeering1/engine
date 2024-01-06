use std::io::stdin;
use std::str::FromStr;

use crate::board::{Board, Move};

pub fn run_uci() {
    let mut board = Board::new();
    let mut debug = false;
    let mut current_pos = "startpos".to_string();
    let mut move_count = 0;

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
                let mut moves_start = 3;
                if tokens[1] != current_pos {
                    board = match tokens[1] {
                        "startpos" => {
                            Board::new()
                        },
                        "fen" => {
                            moves_start += 6;
                            current_pos = format!("{} {} {} {} {} {}",
                                tokens[2], tokens[3], tokens[4], tokens[5], tokens[6], tokens[7]);
                            Board::from_fen(&current_pos)
                        },
                        _ => continue
                    };
                    move_count = 0;
                }

                moves_start += move_count;
                for idx in moves_start..tokens.len() {
                    let m = Move::from_str(tokens[idx]).unwrap();
                    board.make_move(m);
                    move_count += 1;
                }
                if debug {
                    println!("info string {}", board);
                    println!("info string move_count = {}", move_count);
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
