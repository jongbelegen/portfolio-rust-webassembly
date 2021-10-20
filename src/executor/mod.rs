use crate::builtin;
use crate::command::Command;
use crate::parser;
use crate::shell_state::ShellState;
use std::str::FromStr;
use crate::terminal::print_result;

pub mod history;
mod pipelines;

pub fn run(raw_line: &String, shell_state: &mut ShellState) {
    history::append(raw_line).expect("History should be appendable");

    for token in parser::tokenize_command(raw_line) {
        pipelines::is_pipeline_token(&token);

        let result = execute_command(&token, shell_state);
        print_result(shell_state);
        dbg!(result);
    }
}

fn execute_command(cmd: &String, shell_state: &mut ShellState) -> Result<(), ()> {
    let cmd = Command::from_str(cmd)?;
    builtin::evaluate(&cmd, shell_state)?;

    dbg!(cmd);
    Ok(())
}
