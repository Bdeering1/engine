use std::{io::stdin, time::Instant};
use std::str::FromStr;

use chess::Color;

use crate::{board::{Board, Move}, search::SearchContext};

pub fn run_uci() {
    let mut s = SearchContext::new();
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
                    s.board = match tokens[1] {
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
                    s.board.make_move(m);
                    move_count += 1;
                }
                if debug {
                    println!("info string {}", s.board);
                    println!("info string move_count = {}", move_count);
                }
            },
            "go" => {
                let mut ms_remaining: u32 = 0;
                let mut ms_inc: u32 = 0;
                let mut move_time: u32 = 0;
                let mut strict_timing = false;

                let mut idx = 1;
                while idx < tokens.len() {
                    match tokens[idx] {
                        "ponder" => ms_remaining = u32::MAX,
                        "infinite" => ms_remaining = u32::MAX,
                        "movetime" => {
                            idx += 1;
                            move_time = tokens[idx].parse().unwrap();
                            strict_timing = true;
                        },
                        "wtime" => {
                            if s.board.position.side_to_move() == Color::White {
                                idx += 1;
                                ms_remaining = tokens[idx].parse().unwrap();
                            }
                        },
                        "btime" => {
                            if s.board.position.side_to_move() == Color::Black {
                                idx += 1;
                                ms_remaining = tokens[idx].parse().unwrap();
                            }
                        },
                        "winc" => {
                            if s.board.position.side_to_move() == Color::White {
                                idx += 1;
                                ms_inc = tokens[idx].parse().unwrap();
                            }
                        },
                        "binc" => {
                            if s.board.position.side_to_move() == Color::Black {
                                idx += 1;
                                ms_inc = tokens[idx].parse().unwrap();
                            }
                        },
                        _ => ()
                    }
                    idx += 1;
                }

                if move_time == 0 {
                    move_time = ms_remaining / 60 + ms_inc;
                }
                println!("bestmove {}", s.search(move_time, strict_timing, debug));
            },
            "benchmark" => {
                match tokens[1] {
                    "nps" => { //nodes per second
                        let trials: u64 = 20;
                        let mut nps_sum: f64 = 0.;
                        for i in 0..trials {
                            let time: Instant = Instant::now();
                            let m = s.search(500, true, false);
                            let end = time.elapsed().as_millis();
                            s.board.make_move(m);
                            let nps = s.debug.nodes as f64/(end as f64/1000.);
                            nps_sum += nps;
                            println!("processed {} nodes in {}ms, ({:.2} nps) [{}/{}]", s.debug.nodes, end, nps, i+1, trials);
                        }
                        println!("average nps: {:.2} over {} trials", nps_sum/trials as f64, trials);
                    },
                    _ => (),
                }
            }
            "ponderhit" => (),
            "stop" => (),
            "quit" => break,
            _ => ()
        }
    }
}
