use shell_state::ShellState;
use terminal::ReadResult;

mod executor;
mod shell_state;
mod terminal;
mod command;
mod builtin;
mod parser;

fn main() {
    let mut shell_state = ShellState::default();

    loop {
        match terminal::read_line() {
            ReadResult::Empty => continue,
            ReadResult::Ok(line) => {
                executor::run(&line, &mut shell_state);
            }
        }
    }
}
