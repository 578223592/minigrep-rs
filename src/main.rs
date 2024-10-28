use std::{env, fmt, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let (query,file_path) = parse_config(&args);
    let file_content =fs::read_to_string(file_path).expect("fs read error");
    println!("{file_content}")
}

fn parse_config(configs: &Vec<String> ) ->(&str,&str){
    let query = &configs[1];
    let filename = &configs[2];
    println!("Searching for {} in {}", query, filename);
    return (query,filename)
}

// run : cargo run query src/file.txt

/*
2024-10-29 00:04:53进度：
看到了：https://course.rs/basic-practice/refactoring.html中的
修改后，类似 String::new 的调用，我们可以通过 Config::new 来创建一个实例，看起来代码是不是更有那味儿了 ：）
*/