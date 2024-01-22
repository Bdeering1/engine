use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread;
use std::{io::stdin, time::Instant};
use std::str::FromStr;

use chess::Color;

use crate::{board::{Board, Move}, search::SearchContext};

pub fn run_uci() {
    let mut sc = SearchContext::new();
    let mut debug = false;
    let mut current_pos = "startpos".to_string();
    let mut move_count = 0;
    let mut searching = false;

    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let tokens = buf.trim().split(" ").collect::<Vec<&str>>();

        match tokens[0] {
            "uci" => {
                println!("id name engine v0.1.1");
                println!("id author Bryn Deering");
                println!("option name Hash type spin default 16 min 1 max 1048576"); 
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
            "setoption" => {
                match tokens[1] {
                    "Hash" => {
                        let hash_size = tokens[2].parse::<usize>().unwrap();
                        Arc::get_mut(&mut sc.tt).unwrap().resize(hash_size);
                    },
                    _ => (),
                }
            
            },
            "ucinewgame" => {
                Arc::get_mut(&mut sc.tt).unwrap().clear();
            },
            "position" => {
                searching = false;

                let mut moves_start = 3;
                if tokens[1] != current_pos {
                    sc.board = match tokens[1] {
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
                    sc.board.make_move(m);
                    move_count += 1;
                }
                if debug {
                    println!("info string {}", sc.board);
                    println!("info string move_count = {}", move_count);
                }
            },
            "go" => {
                if searching { continue }

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
                            if sc.board.side_to_move() == Color::White {
                                idx += 1;
                                ms_remaining = tokens[idx].parse().unwrap();
                            }
                        },
                        "btime" => {
                            if sc.board.side_to_move() == Color::Black {
                                idx += 1;
                                ms_remaining = tokens[idx].parse().unwrap();
                            }
                        },
                        "winc" => {
                            if sc.board.side_to_move() == Color::White {
                                idx += 1;
                                ms_inc = tokens[idx].parse().unwrap();
                            }
                        },
                        "binc" => {
                            if sc.board.side_to_move() == Color::Black {
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

                let mut sc = sc.clone();
                thread::spawn(move || {
                    println!("bestmove {}", sc.search(move_time, strict_timing, debug));
                });
                searching = true;
            },
            "benchmark" => {
                match tokens[1] {
                    "nps" => { // nodes per second
                        if tokens.len() <= 2 {
                            println!("Expected: benchmark nps <num_trials> <ms/move>");
                            break;
                        }

                        let startpos_file: File = File::open(current_dir().unwrap().join("tools").join("res").join("lichess_elite_smaller.epd")).unwrap();
                        let mut startpositions = BufReader::new(startpos_file).lines().enumerate();
                        let num_trials: usize = tokens[2].parse::<usize>().expect("Expected: benchmark nps <num_trials> <ms/move>");
                        let move_time: u32 = tokens[3].parse::<u32>().expect("Expected: benchmark nps <num_trials> <ms/move>");
                        let mut nps_avg: f64 = 0.;
                        let mut nps_max: f64 = f64::MIN;
                        let mut nps_min: f64 = f64::MAX;
                        let mut trials: f64 = 0.0;

                        println!("started benchmark");
                        while let Some((n, Ok(line))) = startpositions.next() {
                            sc.board = Board::from_fen(&line);
                            let time: Instant = Instant::now();

                            sc.search(move_time, true, false);

                            let end = time.elapsed().as_millis();
                            let nps = sc.debug.nodes as f64 / (end as f64 / 1000.0);
                            
                            nps_max = nps_max.max(nps);
                            nps_min = nps_min.min(nps);
                            nps_avg = ((nps_avg * trials) + nps) / (trials + 1.0);
                            trials += 1.0;
                            println!("processed {} nodes in {}ms, ({:.0} nps) [{}/{}]",
                                sc.debug.nodes,
                                end,
                                nps,
                                n,
                                num_trials
                            );
                            
                            if n % 20 == 0 {
                                println!("-----------");
                                println!("Current avg nps: {:.0} nps max: {:.0} nps min: {:.0} nps [{}/{}]",
                                    nps_avg,
                                    nps_max,
                                    nps_min,
                                    trials,
                                    num_trials
                                );
                                println!("-----------");
                            }
                            if n >= num_trials - 1 { break; }
                        }
                        println!("-----------");
                        println!("Results [{:.0} trials]\nAvg nps: {:.0} nps\nAvg ms/node {:.3}ms\nMax: {:.0} nps\nMin: {:.0} nps",
                            trials,
                            nps_avg,
                            (1.0 / nps_avg) * 1000.0,
                            nps_max,
                            nps_min
                        );
                        println!("-----------");
                    },
                    _ => (),
                }
            },
            "ponderhit" => (),
            "stop" => {
                sc.stop_search.store(true, Ordering::Relaxed);
                searching = false;
            },
            "quit" => break,
            _ => ()
        }
    }
}
