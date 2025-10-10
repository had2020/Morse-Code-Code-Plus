use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::Command;

enum Op {
    IncrementPointer,
    DecrementPointer,
    IncrementBlock,
    DecrementBlock,
    WhileFront,
    WhileBack,
    Input,
    OutputC,
    OutputB,
    Constant { n: usize },
}

fn main() {
    if let Some(filepath) = env::args().nth(1) {
        let file = File::open(filepath.clone()).unwrap();
        let reader = io::BufReader::new(file);

        let mut tokens: Vec<Op> = vec![];

        for line in reader.lines() {
            let mut token_buffer: Vec<char> = vec![];
            let linew = format!("{} ", line.unwrap());
            for i in linew.chars() {
                match i {
                    ' ' => match token_buffer.iter().collect::<String>().as_str() {
                        ".-" => {
                            tokens.push(Op::IncrementPointer);
                            token_buffer = vec![];
                        }
                        "-." => {
                            tokens.push(Op::DecrementPointer);
                            token_buffer = vec![];
                        }
                        "--" => {
                            tokens.push(Op::IncrementBlock);
                            token_buffer = vec![];
                        }
                        ".." => {
                            tokens.push(Op::DecrementBlock);
                            token_buffer = vec![];
                        }
                        "..." => {
                            tokens.push(Op::WhileFront);
                            token_buffer = vec![];
                        }
                        "---" => {
                            tokens.push(Op::WhileBack);
                            token_buffer = vec![];
                        }
                        "." => {
                            tokens.push(Op::Input);
                            token_buffer = vec![];
                        }
                        "-" => {
                            tokens.push(Op::OutputC);
                            token_buffer = vec![];
                        }
                        "-.-" => {
                            tokens.push(Op::OutputB);
                            token_buffer = vec![];
                        }
                        _ => {
                            if token_buffer.len() == 8 {
                                let mut r: usize = 0;
                                for i in 0..token_buffer.len() {
                                    let n: usize = if token_buffer[i] == '-' {
                                        0usize
                                    } else {
                                        1usize
                                    };
                                    r += n * 2usize.pow((token_buffer.len() - i - 1) as u32)
                                }
                                tokens.push(Op::Constant { n: r });
                            }
                            token_buffer = vec![];
                        }
                    },

                    _ => token_buffer.push(i),
                }
            }
        }

        let mut tempfile = File::create("temp.rs").unwrap();
        let mut contents: String = format!(
            "use std::io; fn main() {} let mut v: Vec<usize> = vec![]; let mut pc: usize = 0;",
            '{'
        );

        for i in tokens {
            let a = format!("{}", r#"while v.len() < pc + 1 {v.push(0);}"#);
            let newc;
            match i {
                Op::IncrementPointer => newc = format!("pc = pc.saturating_add(1); "),
                Op::DecrementPointer => newc = format!("pc = pc.saturating_sub(1); "),
                Op::IncrementBlock => newc = format!("{} v[pc] = v[pc].saturating_add(1); ", a),
                Op::DecrementBlock => newc = format!("{} v[pc] = v[pc].saturating_sub(1); ", a),
                Op::WhileFront => newc = format!("{} {}", a, r#" while (v[pc.clone()] != 0) {"#),
                Op::WhileBack => newc = format!("{}", '}'),
                Op::Input => {
                    newc = format!(
                        "{} {}",
                        a,
                        r#"v[pc] = { let mut s = String::new(); io::stdin().read_line(&mut s).unwrap(); s.trim().parse().unwrap() };"#
                    )
                }
                Op::OutputC => {
                    newc = format!(
                        "{} {}",
                        a,
                        r#"print!("{}", char::from_u32(v[pc] as u32).map_or("Not a valid char".to_string(), |c| c.to_string()));"#
                    )
                }
                Op::OutputB => newc = format!("{} {}", a, r#"print!("{}", v[pc]);"#),
                Op::Constant { n } => newc = format!("{} v[pc] = {};", a, n),
            }
            contents = format!("{}{}", contents, newc);
        }
        contents = format!("{}{}", contents, '}');

        tempfile.write_all(contents.as_bytes()).unwrap();

        let mut exe_name = String::new();
        for i in filepath.chars() {
            if i != '.' {
                exe_name = format!("{}{}", exe_name, i)
            } else {
                break;
            }
        }

        Command::new("rustc")
            .arg("temp.rs")
            .arg("-o")
            .arg(&exe_name)
            .status()
            .expect("Failed to use rustc to transpile!");

        std::fs::remove_file("temp.rs").unwrap();
    } else {
        println!("Please include filepath, as command line argument!");
        std::process::exit(1);
    }
}
