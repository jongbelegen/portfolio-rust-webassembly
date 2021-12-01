use terminal::ReadResult;
use crate::exception::Exception;
use crate::shell_state::ShellState;

mod executor;
mod shell_state;
mod terminal;
mod command;
mod builtin;
mod parser;
mod utils;
mod exception;

fn main() -> Result<(), Exception> {
    let mut shell_state = ShellState::default();

    loop {
        match terminal::read_line() {
            ReadResult::Empty => continue,
            ReadResult::Ok(line) => {
                executor::run(&line, &mut shell_state)?
            }
        }
    }
}
