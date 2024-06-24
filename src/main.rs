#[allow(unused_imports)]
use std::io::{self, Write};

const BUILT_IN_COMMANDS: [&str; 3] = ["echo", "exit", "type"];

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
       x if x.starts_with("echo ") => {
            println!("{}", x.trim_start_matches("echo "));
        }
        x if x.starts_with("type ") => {
            let command = x.trim_start_matches("type ");
            if BUILT_IN_COMMANDS.contains(&command) {
                println!("{} is a shell builtin", command);
            } else {
                println!("{}: not found", command);
            }
        }
        "exit 0" => {
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
