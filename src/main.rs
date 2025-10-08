use std::env;

fn main() {
    if let Some(second) = env::args().nth(1) {
        println!("arg: {}", second);
    } else {
        println!("Please include filepath, as command line argument!");
        std::process::exit(1);
    }
}
