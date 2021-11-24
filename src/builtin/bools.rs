
use crate::shell_state::ShellState;

pub fn false_builtin(shell_state: &mut ShellState) {
    shell_state.output.code = Some(1);
}

pub fn true_builtin(shell_state: &mut ShellState) {
    shell_state.output.code = Some(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_false() {
        let mut state = ShellState::default();
        false_builtin(&mut state);
        assert_eq!(state.output.code, Some(1));
    }

    #[test]
    fn test_true() {
        let mut state = ShellState::default();
        true_builtin(&mut state);
        assert_eq!(state.output.code, Some(0));
    }
}
