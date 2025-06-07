#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    

    // Wait for user input
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "exit 0" => {
                exit(0)
            },
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
        input.clear(); // Clear the input buffer for the next command
    }
    
}
