mod uci;

mod board;
mod search;
mod eval;
mod tt;

fn main() {
    uci::run_uci();
}
