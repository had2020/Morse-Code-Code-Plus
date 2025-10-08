use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Some(filepath) = env::args().nth(1) {
        let file = File::open(filepath).unwrap();
        let reader = io::BufReader::new(file);

        let mut line_buffer: Vec<String> = vec![];

        for line in reader.lines() {
            line_buffer.push(line.unwrap());
        }
    } else {
        println!("Please include filepath, as command line argument!");
        std::process::exit(1);
    }
}
