use dotenvy_macro::dotenv;

pub fn run(args: &Vec<String>) {
   let cutechess = dotenv!("CUTE_CHESS");

   println!("running cutechess from {}", cutechess);
}
