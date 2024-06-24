#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    wait_for_input();

    loop {
        let input = get_input();
        let command = input.trim();
        handle_command(command);
        wait_for_input();
    }
}

fn handle_command(command: &str) {
    match command {
        "exit" => {
            std::process::exit(0);
        }
        _ => {
            println!("{}: command not found", command);
        }
    }
}

fn wait_for_input(){
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn get_input() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input
}
