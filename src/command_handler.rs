use std::env;
use std::process::exit;

const BUILT_IN_COMMANDS: [&str; 3] = ["echo", "exit", "type"];

pub(crate) trait CommandHandler {
    fn handle(&self, arguments: &[&str]);
}

struct EchoCommandHandler;

impl CommandHandler for EchoCommandHandler {
    fn handle(&self, arguments: &[&str]) {
        println!("{}", arguments.join(" "));
    }
}

struct TypeCommandHandler;

impl CommandHandler for TypeCommandHandler {
    fn handle(&self, arguments: &[&str]) {
        if BUILT_IN_COMMANDS.contains(&arguments[0]) {
            println!("{} is a shell builtin", arguments[0]);
        } else {
            match env::var("PATH")
                .unwrap()
                .split(":")
                .map(|path| format!("{}/{}", path, arguments[0]))
                .find(|path| std::fs::metadata(path).is_ok()) {
                Some(path) => println!("{} is {}", arguments[0], path),
                None => println!("{}: not found", arguments[0])
            }
        }
    }
}

struct ExitCommandHandler;

impl CommandHandler for ExitCommandHandler {
    fn handle(&self, arguments: &[&str]) {
        // parse the first argument as an integer
        let code = arguments[0].parse::<i32>().unwrap();
        exit(code);
    }
}

struct ExectuableCommandHandler{
    command: String
}

impl CommandHandler for ExectuableCommandHandler {
    
    fn handle(&self, arguments: &[&str]) {
        let cmd = &self.command;
        let status = std::process::Command::new(cmd)
            .args(arguments)
            .status()
            .expect("failed to execute process");
        if !status.success() {
            println!("{}: command not found", cmd);
        }
    }
}

pub(crate) fn create_command_handler(cmd: &str) -> Result<Box<dyn CommandHandler>, ()> {
    match cmd {
        "echo" => Ok(Box::new(EchoCommandHandler)),
        "exit" => Ok(Box::new(ExitCommandHandler)),
        "type" => Ok(Box::new(TypeCommandHandler)),
        _ => Ok(Box::new(ExectuableCommandHandler{command: cmd.to_string()}))
    }
}