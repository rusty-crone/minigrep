use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run -- <query> <file_path>");
        return
    }

    let config = parse_configs(&args);

    let contents = fs::read_to_string(&config.file_path)
        .expect("Should have been able to read the file");

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_configs(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config{ query, file_path }
}