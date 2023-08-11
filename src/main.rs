use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        print!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

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

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: cargo run -- <query> <file_path>");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{ query, file_path })
    }
}
