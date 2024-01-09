use std::process::Command;

use dotenvy_macro::dotenv;

pub fn run(args: &Vec<String>) {
   let cutechess = dotenv!("CUTE_CHESS");

   println!("running cutechess from {}", cutechess);

   //args should be engine1, engine 2, # of rounds

   let arg0 = format!("conf={}",args[0]);
   let arg1 = format!("conf={}",args[1]);
   let arg2 = format!("{}",args[2]);

   let run_args = vec!(
      "-engine",
      arg0.as_str(),
      "-engine",
      arg1.as_str(),
      "-each","tc=0/60+0",
      "-maxmoves","1000",
      "-games","2",
      "-repeat", "-recover",
      "-resultformat", "wide2",
      "-ratinginterval", "10",
      "-rounds", arg2.as_str(),
      "-concurrency", "2",
      "-tournament", "gauntlet",
      "-openings", "file=C:\\Users\\Josh\\Desktop\\Code Stuff\\rust\\engine\\tools\\res\\openings1.epd", "format=epd", "order=random"
   );

   let mut cutechess_program = Command::new(cutechess).args(run_args).spawn().expect("Failed to run cutechess");
   
   //this makes sure that the child process actually exits when the rest of the program exits
   cutechess_program.wait().expect("Cutechess failed to execute");
   
}
