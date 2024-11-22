use std::env;
use std::process;
use aleph_ollama::translate_code_from_string;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: basic_string <code> <target_language>");
        process::exit(1);
    }

    let code = &args[1];
    let target_language = &args[2];

    println!("Translating code '{}' to '{}'", code, target_language);

    // Call the translation function
    match translate_code_from_string(code, target_language) {
        Ok(translated_code) => {
            println!("{}", translated_code);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}

