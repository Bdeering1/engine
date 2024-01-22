use std::env::args;

mod cutechess;

fn main() {
   let args: Vec<String> = args().collect();

   if args.len() < 2 {
      println!("Usage: {} <command>\n", args[0]);
      return;
   }

   let cmd = args[1].as_str();
   match cmd {
      "cutechess" => cutechess::run(&args[2..].to_vec()),
      _ => println!("Unknown command: {}", cmd),
   }
}
