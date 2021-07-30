use std::io::{self, Write};
use crate::ShellOutput;


pub enum ReadResult {
    Ok(String),
    Empty
}

pub fn read_line() -> ReadResult  {
    print_prompt();

    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read command");

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

pub fn print_result(output: ShellOutput) {
    match output {
        ShellOutput {
            stderr: Some(text),
            stdout: _,
        } => println!("{}", text),
        ShellOutput {
            stdout: Some(text),
            stderr: _,
        } => eprintln!("{}", text),
        output => eprintln!("No values provided, {:?}", output),
    }
}
