use ini_file_parser::parse_ini;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Command required. Use 'help' to see available commands.");
        return;
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                eprintln!("Error: Filename required for 'parse' command.");
                return;
            }
            let filename = &args[2];
            let content = fs::read_to_string(filename).expect("Could not read file");
            match parse_ini(&content) {
                Ok(x) => println!("Parsed successfully. {:?}", x),
                Err(e) => eprintln!("Error parsing file: {}", e),
            }
        }
        "help" => print_help(),
        "credits" => print_credits(),
        _ => {
            eprintln!("Error: Unknown command. Use 'help' to see available commands.");
        }
    }
}

fn print_help() {
    println!("Usage: ini_parser <command> [file]");
    println!();
    println!("Commands:");
    println!("  parse <file_path>   Parse the specified INI file");
    println!("  help                Show this help message");
    println!("  credits             Show the author");
}

fn print_credits() {
    println!("ini_parser v1.0");
    println!("Developed by Semytskyi Oleksandr");
}
