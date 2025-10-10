use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::Command;

fn main() {
    if let Some(filepath) = env::args().nth(1) {
        let file = File::open(filepath.clone()).unwrap();
        let reader = io::BufReader::new(file);

        let mut line_buffer: Vec<String> = vec![];

        for line in reader.lines() {
            line_buffer.push(line.unwrap());
        }

        // TODO
        println!("{:?}", line_buffer);



        let filename:String = filepath.clone().strip_suffix('.').unwrap_or(&filepath.clone()).to_string();

        Command::new("clang")
            .args(["-arch", "arm64", "-o", &filename, &format!("{}.s", filename)])
            .status()
            .expect("failed to execute clang");

    } else {
        println!("Please include filepath, as command line argument!");
        std::process::exit(1);
    }
}
