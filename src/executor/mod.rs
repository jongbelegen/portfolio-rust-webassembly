use crate::builtin;
use crate::command::Command;
use crate::parser;
use crate::shell_state::{ShellOutput, ShellState};
use crate::terminal::print_result;
use std::str::FromStr;

pub mod history;

pub fn run(raw_line: &String, shell_state: &mut ShellState) {
    history::append(raw_line).expect("History should be appendable");
    evaluate_tokens(parser::tokenize_raw_line(raw_line), shell_state);
    shell_state.output.clear()
}

// pub fn run_as_pipeline(tokens: Vec<String>, shell_state: &mut ShellState) {
//     for token in tokens {
//         token.split()
//     }
// }

fn evaluate_tokens(tokens: Vec<String>, shell_state: &mut ShellState) {
    let mut skip_next = false;
    for token in tokens {
        if skip_next {
            skip_next = false;
            continue;
        }

        if is_and_or_list_token(&token) {
            if token.eq(OR_TOKEN) && shell_state.output.is_ok() {
                skip_next = true;
            }

            if token.eq(AND_TOKEN) && !shell_state.output.is_ok() {
                skip_next = true;
            }

            continue;
        }

        // TODO: Handle this somewhere else
        if execute_command(&token, shell_state).is_err() {
            shell_state
                .output
                .set_stderr(127, format!("shell: command not found: {}", &token));
        }

        print_result(shell_state);
    }
}

const OR_TOKEN: &str = "||";
const AND_TOKEN: &str = "&&";
const SEMICOLON_TERMINATOR: &str = ";";
const ASYNC_TOKEN: &str = "&";

// https://pubs.opengroup.org/onlinepubs/009695399/utilities/xcu_chap02.html#tag_02_09_03
const AND_OR_LIST_TOKENS: [&str; 4] = [AND_TOKEN, OR_TOKEN, SEMICOLON_TERMINATOR, ASYNC_TOKEN];

pub fn is_and_or_list_token(token: &String) -> bool {
    let token_as_str: &str = &token;
    AND_OR_LIST_TOKENS.contains(&token_as_str)
}

fn execute_command(raw_cmd: &String, shell_state: &mut ShellState) -> Result<(), ()> {
    let cmd = Command::from_str(raw_cmd)?;
    builtin::evaluate(&cmd, shell_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn or_token_should_only_eval_first_cmd_when_first_succeeds() {
        let mut state = ShellState::default();

        evaluate_tokens(
            vec![
                String::from("true"),
                String::from(OR_TOKEN),
                String::from("UNKNOWN_COMMAND"),
            ],
            &mut state,
        );

        assert_eq!(state.output.code, Some(0));
    }

    #[test]
    fn or_token_should_eval_second_cmd_when_first_fails() {
        let mut state = ShellState::default();

        evaluate_tokens(
            vec![
                String::from("UNKNOWN_COMMAND"),
                String::from(OR_TOKEN),
                String::from("true"),
            ],
            &mut state,
        );

        assert_eq!(state.output.code, Some(0));
    }

    #[test]
    fn and_token_should_eval_second_cmd_when_first_cmd_succeeds() {
        let mut state = ShellState::default();

        evaluate_tokens(
            vec![
                String::from("true"),
                String::from(AND_TOKEN),
                String::from("false"),
            ],
            &mut state,
        );

        assert_eq!(state.output.code, Some(1));
    }

    #[test]
    fn and_token_should_not_eval_second_cmd_when_first_cmd_fails() {
        let mut state = ShellState::default();

        evaluate_tokens(
            vec![
                String::from("false"),
                String::from(AND_TOKEN),
                String::from("true"),
            ],
            &mut state,
        );

        assert_eq!(state.output.code, Some(1));
    }

    #[test]
    fn semicolon_terminator_should_eval_when_first_cmd_fails() {
        let mut state = ShellState::default();

        evaluate_tokens(
            vec![
                String::from("false"),
                String::from(SEMICOLON_TERMINATOR),
                String::from("true"),
            ],
            &mut state,
        );

        assert_eq!(state.output.code, Some(0));
    }

    #[test]
    fn semicolon_terminator_should_eval_when_first_cmd_succeeds() {
        let mut state = ShellState::default();

        evaluate_tokens(
            vec![
                String::from("true"),
                String::from(SEMICOLON_TERMINATOR),
                String::from("echo foo"),
            ],
            &mut state,
        );

        assert_eq!(state.output.code, Some(0));
        assert_eq!(state.output.stdout, Some(String::from("foo")));
    }

    #[test]
    fn evaluate_should_always_evaluate_all_tokens() {
        let mut state = ShellState::default();

        evaluate_tokens(
            vec![
                String::from("false"),
                String::from(AND_TOKEN),
                String::from("true"),
                String::from(OR_TOKEN),
                String::from("false"),
                String::from(";"),
                String::from("UNKNOWN COMMAND"),
            ],
            &mut state,
        );

        assert_eq!(state.output.code, Some(127));
    }
}
