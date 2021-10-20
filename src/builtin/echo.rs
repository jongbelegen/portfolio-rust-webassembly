use crate::command::Command;
use crate::shell_state::ShellState;

pub fn run(cmd: &Command, shell_state: &mut ShellState) {
    shell_state.output.set_stdout(cmd.args.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_no_args() {
        let mut state = ShellState::default();
        let cmd = Command {
            keyword: String::from(""),
            args: vec!(),
        };

        run(&cmd, &mut state);

        assert_eq!(state.output.stdout, Some(String::from("")));
    }

    #[test]
    fn test_run_some_args() {
        let mut state = ShellState::default();
        let cmd = Command {
            keyword: String::from(""),
            args: vec!(String::from("--test"), String::from("ee!!"), String::from("\nabc")),
        };

        run(&cmd, &mut state);

        assert_eq!(state.output.stdout, Some(String::from("--test ee!! \nabc")));
    }
}
