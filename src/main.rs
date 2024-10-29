use std::{env, fs};

// run : cargo run query src/file.txt

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let config_result = Config::build(args.into_iter());
    let config = config_result.unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(err) = run(&config){  //if let 去匹配是否存在错误即可
        println!("Problem run fn: {err}");
        std::process::exit(1);
    }

   
}

fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(config.file_path.clone())?;
    println!("Searching for {}", config.query);
    println!("{file_content}");

    return Ok(());
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        return Ok(Config { query, file_path });
    }
}

/*
2024-10-29 00:04:53进度：
看到了：https://course.rs/basic-practice/refactoring.html中的
分离逻辑代码到库包中
*/
