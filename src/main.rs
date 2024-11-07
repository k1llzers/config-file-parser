use config_file_parser::parse_ini;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ini_parser <file>");
        return;
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Could not read file");

    match parse_ini(&content) {
        Ok(_) => println!("Parsed successfully."),
        Err(e) => eprintln!("Error parsing file: {}", e),
    }
}