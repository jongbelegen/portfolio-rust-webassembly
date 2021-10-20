use crate::shell_state::ShellState;

pub fn run(shell_state: &mut ShellState) {
    shell_state.output.set_stdout(String::from("\u{001b}c"))
}
