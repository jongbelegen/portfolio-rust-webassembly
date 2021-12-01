use crate::shell_state::ShellState;

pub fn run(shell_state: &mut ShellState) {
    shell_state.output.set_stdout(String::from("\u{001b}c"))
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_run() {
        let mut state = ShellState::default();

        run(&mut state);

        assert_eq!(state.output.stdout, Some(String::from("\u{001b}c")));
    }
}
