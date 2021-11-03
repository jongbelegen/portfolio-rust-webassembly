use crate::command::Command;
use crate::shell_state::ShellState;

pub fn run(_cmd: &Command, shell_state: &mut ShellState) {
    dbg!(&shell_state);

    shell_state.output.set_stderr(1, String::from("Some error"));
}
