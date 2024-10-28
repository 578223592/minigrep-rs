use std::{env, fmt, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_content = fs::read_to_string(&args[1]).expect("fs read error");
    println!("{file_content}")
}

// run : cargo run src/file.txt