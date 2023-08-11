use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run -- <query> <file_path>");
        return
    }

    let query = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("Searching for {}", query);
    println!("In file {}", file_path);
    println!("With text:\n{contents}");
}
