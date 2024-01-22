mod uci;

mod board;
mod search;
mod eval;
mod tt;
mod perft;

fn main() {
    uci::run_uci();
}
