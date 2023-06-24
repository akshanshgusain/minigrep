use std::{env, process};
use minigrep::Config;
use minigrep::run;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Query String: {}", config.query);
    println!("File Name String: {}", config.file_name);

    /*It allows you to handle both the success and failure cases in a single expression.*/
    // run(config).unwrap_or_else(|err|{
    //    println!("application error: {}", err);
    //     process::exit(1);
    // });

    /*It is useful when you only need to handle a specific case and want to ignore other cases.*/
    if let Err(e) = run(config) {
        println!("application error: {}", e);
        process::exit(1);
    }
}

