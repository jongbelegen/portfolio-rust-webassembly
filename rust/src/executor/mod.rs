use crate::{builtin, log};
use crate::command::Command;
use crate::exception::Exception;
use crate::parser::ast;
use crate::parser::token;
use crate::parser::token::Token;
use crate::parser::token::Token::{And, Or, Raw};
use crate::shell_state::ShellState;
use std::convert::TryFrom;

pub mod history;

pub fn run(raw_line: &String, shell_state: &mut ShellState) -> Result<(), Exception> {
    // history::append(raw_line).expect("History should be appendable");
    let tokens = token::tokenize_raw_line(raw_line);

    let a = ast::parse_to_ast(tokens.as_slice());

    log(format!("{:?}", &a));
    shell_state.output.clear();
    Ok(())
}

// fn run_with_pipelines(tokens: Vec<Token>, shell_state: &mut ShellState) {
//     let mut iter = group_by_pipeline(tokens).iter().peekable();
//
//     while Some(group) = iter.next() {
//         evaluate_tokens(group, shell_state);
//     }
//
//     dbg!(tokens.clone());
//     dbg!(group_by_pipeline(tokens.clone()).windows(2));
//     for group in group_by_pipeline(tokens).windows(2) {
//         dbg!(&group[0]);
//         dbg!(&group[1]);
//
//         // evaluate_tokens(group.clone(), shell_state);
//     }
// }

fn evaluate_tokens(tokens: Vec<Token>, shell_state: &mut ShellState) {
    let mut skip_next = false;
    for token in tokens {
        if skip_next {
            skip_next = false;
            continue;
        }

        match token {
            Raw(cmd) => {
                if execute_command(&cmd, shell_state).is_err() {
                    shell_state
                        .output
                        .set_stderr(127, format!("shell: command not found: {}", &cmd));
                }
            }
            And => {
                if !shell_state.output.is_ok() {
                    skip_next = true;
                }
            }
            Or => {
                if shell_state.output.is_ok() {
                    skip_next = true;
                }
            }
            _ => (),
        }
    }
}

fn execute_command(raw_cmd: &String, shell_state: &mut ShellState) -> Result<(), ()> {
    let cmd = Command::try_from(raw_cmd)?;
    builtin::evaluate(&cmd, shell_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::token::Token::{And, Or, Raw, Semicolon};

    #[test]
    fn or_token_should_only_eval_first_cmd_when_first_succeeds() {
        let mut state = ShellState::default();

        evaluate_tokens(
            vec![
                Raw(String::from("true")),
                Or,
                Raw(String::from("UNKNOWN_COMMAND")),
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
                Raw(String::from("UNKNOWN_COMMAND")),
                Or,
                Raw(String::from("true")),
            ],
            &mut state,
        );

        assert_eq!(state.output.code, Some(0));
    }

    #[test]
    fn and_token_should_eval_second_cmd_when_first_cmd_succeeds() {
        let mut state = ShellState::default();

        evaluate_tokens(
            vec![Raw(String::from("true")), And, Raw(String::from("false"))],
            &mut state,
        );

        assert_eq!(state.output.code, Some(1));
    }

    #[test]
    fn and_token_should_not_eval_second_cmd_when_first_cmd_fails() {
        let mut state = ShellState::default();

        evaluate_tokens(
            vec![Raw(String::from("false")), And, Raw(String::from("true"))],
            &mut state,
        );

        assert_eq!(state.output.code, Some(1));
    }

    #[test]
    fn semicolon_terminator_should_eval_when_first_cmd_fails() {
        let mut state = ShellState::default();

        evaluate_tokens(
            vec![
                Raw(String::from("false")),
                Semicolon,
                Raw(String::from("true")),
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
                Raw(String::from("true")),
                Semicolon,
                Raw(String::from("echo foo")),
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
                Raw(String::from("false")),
                And,
                Raw(String::from("true")),
                Or,
                Raw(String::from("false")),
                Semicolon,
                Raw(String::from("UNKNOWN COMMAND")),
            ],
            &mut state,
        );

        assert_eq!(state.output.code, Some(127));
    }
}
