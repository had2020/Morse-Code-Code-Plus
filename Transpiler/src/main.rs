use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::Command;

enum Operations {
    IncrementPointer,
    DecrementPointer,
    IncreaseBlock,
    DecreaseBlock,
    WhileFront,
    WhileBack,
    Input,
    Output,
    Constant { n: u8 },
}

fn main() {
    if let Some(filepath) = env::args().nth(1) {
        let file = File::open(filepath.clone()).unwrap();
        let reader = io::BufReader::new(file);

        let mut token_buffer: Vec<char> = vec![];

        for line in reader.lines() {
            for i in line.unwrap().chars() {
                token_buffer.push(i)
            }
        }
    } else {
        println!("Please include filepath, as command line argument!");
        std::process::exit(1);
    }
}
