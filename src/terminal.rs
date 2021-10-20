use crate::shell_state::{ShellOutput, ShellState};
use std::io::{self, Write};

pub enum ReadResult {
    Ok(String),
    Empty,
}

pub fn read_line() -> ReadResult {
    print_prompt();

    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read command");

    match command.eq("") {
        false => ReadResult::Ok(command),
        true => ReadResult::Empty,
    }
}

fn print_prompt() {
    let prompt_char = "%";

    print!("{} ", prompt_char);
    io::stdout().flush().unwrap();
}

pub fn print_result(shell_state: &ShellState) {
    match &shell_state.output {
        ShellOutput {
            stdout: Some(text),
            stderr: _,
        } => println!("{}", text),
        ShellOutput {
            stderr: Some(text),
            stdout: _,
        } => eprintln!("{}", text),
        output => eprintln!("No values provided, {:?}", output),
    }
}
