pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = match args.next() {
            Some(arg) => {
                if arg.to_lowercase() == "true" {
                    true
                } else {
                    false
                }
            }
            None => false,
        };
        return Ok(Config {
            query,
            file_path,
            ignore_case,
        });
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(config.file_path.clone())?;
    println!(
        "Searching for {} in file {}",
        config.query, config.file_path
    );
    // println!("{file_content}");
    println!("search result: ");
    let mut search_fun: for<'a> fn(query: &'a str, contents: &'a str) -> Vec<&'a str> =
        case_sensitive_search;
    if config.ignore_case {
        search_fun = case_insensitive_search;
    }
    let search_result = search_fun(&config.query, &file_content);
    println!("{:?}", search_result);
    return Ok(());
}

pub fn case_sensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut res = Vec::new();
    let res: Vec<&str> = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();

    return res;
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let res = contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect();
    return res;
}

// add codes for test
// cargo test --package minigrep_rs --lib -- tests --show-output
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_sensitive() {
        let query = "ductive";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            case_sensitive_search(query, contents)
        );
    }

    #[test]
    fn search_insensitive() {
        let query = "DUCTIVE";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            case_insensitive_search(query, contents)
        );
    }
}
