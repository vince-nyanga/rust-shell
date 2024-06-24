mod command_handler;

#[allow(unused_imports)]
use std::io::{self, Write};
use crate::command_handler::create_command_handler;

fn main() {
    // Uncomment this block to pass the first stage
    wait_for_input();

    loop {
        let input = get_input();
        let prompt_parts: Vec<&str> = input.trim().split_whitespace().collect();

        if let Some((cmd, args)) = prompt_parts.split_first() {
            let handler = create_command_handler(cmd);
            handler.handle(args);
        }

        wait_for_input();
    }
}

fn wait_for_input() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn get_input() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input
}
