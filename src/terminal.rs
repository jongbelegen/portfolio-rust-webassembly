
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

    let trimmed = String::from(command.trim());
    match command.eq("") {
        false => ReadResult::Ok(trimmed),
        true => ReadResult::Empty,
    }
}

fn print_prompt() {
    let prompt_char = "%";

    print!("{} ", prompt_char);
    io::stdout().flush().unwrap();
}

// pub fn print_result(shell_state: &ShellState) {
//     match &shell_state.output {
//         ShellOutput {
//             code: Some(0),
//             stdout: Some(text),
//             stderr: _,
//             stdin: _,
//         } => println!("{}", text),
//         ShellOutput {
//             code: Some(_),
//             stderr: Some(text),
//             stdout: _,
//             stdin: _,
//         } => eprintln!("{}", text),
//         _ => ()
//     }
// }
