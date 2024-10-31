use std::env;

use minigrep_rs::{run, Config};

// run : cargo run then src/file.txt true
// 参数说明：query内容 文件路径 是否忽略大小写（可选，默认false）

fn main() {
    let args = env::args();
    dbg!(&args);
    let config_result = Config::build(args);
    let config = config_result.unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(err) = run(&config) {
        //if let 去匹配是否存在错误即可
        eprintln!("Problem run fn: {err}");
        std::process::exit(1);
    }
}

/*
2024年10月31日00:25:59进度：
看到了：https://course.rs/basic-practice/stderr.html
还没开始看
*/
