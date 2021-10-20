use crate::builtin;
use crate::command::Command;
use crate::parser;
use crate::shell_state::ShellState;
use crate::terminal::print_result;
use std::str::FromStr;

pub mod history;
mod pipelines;

pub fn run(raw_line: &String, shell_state: &mut ShellState) {
    history::append(raw_line).expect("History should be appendable");

    for token in parser::tokenize_command(raw_line) {
        pipelines::is_pipeline_token(&token);

        execute_command(&token, shell_state);
        print_result(shell_state);
        shell_state.output.clear();
    }
}

fn execute_command(cmd: &String, shell_state: &mut ShellState) -> Result<(), ()> {
    let cmd = Command::from_str(cmd)?;
    builtin::evaluate(&cmd, shell_state);

    Ok(())
}
