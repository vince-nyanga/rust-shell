use std::env;
use std::process::exit;

const BUILT_IN_COMMANDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

/// `CommandHandler` is a trait that defines a common interface for handling shell commands.
///
/// This trait has a single method `handle` which takes a slice of string references as arguments.
/// The specific implementation of `handle` in the selected code is designed to process shell commands.
///
/// # Method
///
/// * `handle` - Takes a slice of string references as arguments and performs the action associated with the command.
///
/// # Examples
///
/// Implementations of this trait will provide the specific behavior for the `handle` method.
///
/// ```
/// struct EchoCommandHandler;
/// impl CommandHandler for EchoCommandHandler {
///     fn handle(&self, arguments: &[&str]) {
///         println!("{}", arguments.join(" "));
///     }
/// }
/// ```
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
       let path = match arguments[0] {
            "~" => env::var("HOME").unwrap(),
            _ => arguments[0].to_string()
       };
        
        if !env::set_current_dir(path.clone()).is_ok() {
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

/// Creates a command handler based on the given command.
///
/// This function takes a command as a string and returns a boxed trait object
/// that implements the `CommandHandler` trait. The specific type of the trait
/// object depends on the command.
///
/// # Arguments
///
/// * `command` - A string slice that holds the name of the command.
///
/// # Returns
///
/// * A boxed trait object that implements `CommandHandler`.
///
/// # Examples
///
/// ```
/// let handler = create_command_handler("echo");
/// handler.handle(&["Hello, world!"]);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return a `Result`, so it does not report errors via the type system.
/// However, if the command is not recognized, it returns a `NonExistentCommandHandler` that
/// prints an error message when its `handle` method is called.
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