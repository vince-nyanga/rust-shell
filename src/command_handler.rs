use std::env;
use std::process::exit;

const BUILT_IN_COMMANDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

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
            match get_command_from_path(arguments[0]) {
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

struct ExecutableCommandHandler {
    command: String,
}

impl CommandHandler for ExecutableCommandHandler {
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

struct PwdCommandHandler;

impl CommandHandler for PwdCommandHandler {
    fn handle(&self, _arguments: &[&str]) {
        println!("{}", env::current_dir().unwrap().display());
    }
}

struct NonExistentCommandHandler {
    command: String,
}

impl CommandHandler for NonExistentCommandHandler {
    fn handle(&self, _arguments: &[&str]) {
        println!("{}: command not found", self.command);
    }
}

struct CurrentDirectoryCommandHandler;

impl CommandHandler for CurrentDirectoryCommandHandler {
    fn handle(&self, arguments: &[&str]) {
       let path = arguments[0];
        if !env::set_current_dir(path).is_ok() {
            println!("cd: {}: No such file or directory", path);
        }
    }
}

fn get_command_from_path(command: &str) -> Option<String> {
    return env::var("PATH")
        .unwrap()
        .split(":")
        .map(|path| format!("{}/{}", path, command))
        .find(|path| std::fs::metadata(path).is_ok());
}

pub(crate) fn create_command_handler(command: &str) -> Box<dyn CommandHandler> {
    match command {
        "echo" => Box::new(EchoCommandHandler),
        "exit" => Box::new(ExitCommandHandler),
        "type" => Box::new(TypeCommandHandler),
        "pwd" => Box::new(PwdCommandHandler),
        "cd" => Box::new(CurrentDirectoryCommandHandler),
        _ => {
            match get_command_from_path(command) {
                Some(_) => Box::new(ExecutableCommandHandler { command: command.to_string() }),
                None => Box::new(NonExistentCommandHandler { command: command.to_string() }),
            }
        }
    }
}