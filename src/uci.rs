use std::io::stdin;

pub fn run_uci() {
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
                    "on" => (),
                    "off" => (),
                    _ => (),
                }
            },
            "isready" => {
                println!("readyok");
            },
            "setoption" => (),
            "ucinewmgame" => (),
            "position" => (),
            "go" => (),
            "ponderhit" => (),
            "stop" => (),
            "quit" => break,
            _ => println!("unrecognized"),
        }
    }
}
