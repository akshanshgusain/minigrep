use std::error::Error;
use std::fs::File;
use std::io::Read;

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
        return Ok(Config { query, file_name });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.file_name)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let res = search(&config.query, &contents);
    for line in res{
        println!("res: {:?}", line);
    }

    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }
}