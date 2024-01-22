use std::{process::Command, env::current_dir};
use chrono::Local;

use dotenvy_macro::dotenv;
use num_cpus;

pub fn run(args: &Vec<String>) {
   let cutechess = dotenv!("CUTE_CHESS");

   if args.len() < 2 {
      println!("Usage: <program> cutechess <opponent> <rounds>\n");
      return;
   }
   println!("running cutechess from {}", cutechess);

   let current_dir = current_dir().expect("Failed to get current directory");

   let engine_build_path = current_dir.join("target").join("release").join("engine.exe").to_str().unwrap().to_string();
   let engine_cmd = format!("cmd={}", engine_build_path);
   let opponent_cmd = format!("cmd={}", args[0]);

   let rounds = format!("{}",args[1]);

   let openings_path = current_dir.join("tools").join("res").join("openings1.epd").to_str().unwrap().to_string();
   let openings_cmd = format!("file={}", openings_path);
   let output_path = current_dir.join("tools").join("res").join("testresults").to_str().unwrap().to_string();
   let output_cmd = format!("{}/{}.pgn", output_path, Local::now().format("%d-%m-%y %H_%M"));

   let max_threads = (num_cpus::get() - 2).to_string();
   let run_args = vec!(
      "-engine",
      &engine_cmd, "name=current-iter",
      "-engine",
      &opponent_cmd,
      "-each","tc=0/60+0.1", "proto=uci",
      "-maxmoves","1000",
      "-pgnout", &output_cmd,
      "-games","2",
      "-repeat", "-recover",
      "-resultformat", "wide2",
      "-ratinginterval", "10",
      "-rounds", &rounds,
      "-concurrency", &max_threads,
      "-tournament", "gauntlet",
      "-openings", &openings_cmd, "format=epd", "order=random",
   );

   let mut cutechess_program = Command::new(cutechess).args(run_args).spawn().expect("Failed to run cutechess");
   
   //this makes sure that the child process actually exits when the rest of the program exits
   cutechess_program.wait().expect("Cutechess failed to execute");
}
