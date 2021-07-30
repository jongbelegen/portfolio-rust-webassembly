
use std::io;



use terminal::ReadResult;

mod builtin;
mod terminal;
mod error;
mod executor;

#[derive(Debug, PartialEq)]
pub struct Command {
    keyword: String,
    args: Vec<String>,
}

#[derive(Debug)]
pub struct ShellOutput {
    stdout: Option<String>,
    stderr: Option<String>,
}

fn main() {
    loop {
        match terminal::read_line() {
            ReadResult::Empty => continue,
            ReadResult::Ok(line) => {
                executor::run(&line);

                // terminal::print_result(result);
            }
        }
    }
}

fn eval_command(command: Command) -> ShellOutput {
    builtin::evaluate(command).unwrap_or(ShellOutput {
        stdout: None,
        stderr: Some(String::from("Command not found")),
    })
}

fn read_command() -> String {
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read command");

    String::from(command.trim())
}

fn tokenize_command(command: &String) -> Result<Command, ()> {
    let mut command_split: Vec<String> =
        command.split_whitespace().map(|s| s.to_string()).collect();

    if command_split.len() == 0 {
        return Err(());
    }

    let command = Command {
        keyword: command_split.remove(0),
        args: command_split,
    };

    Ok(command)
}
