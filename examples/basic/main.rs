use std::env;
use std::process;
use aleph_ollama::translate_code;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: basic <file_path> <target_language>");
        process::exit(1);
    }

    let file_path = &args[1];
    let target_language = &args[2];

    println!("Translating file '{}' to '{}'", file_path, target_language);

    // Call the translation function
    match translate_code(file_path, target_language) {
        Ok(translated_code) => {
            println!("{}", translated_code);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}

