use std::str::FromStr;

use crate::command::Command;
use crate::executor::history;
use crate::shell_state::ShellState;

mod bools;
mod cat;
pub mod clear;
mod echo;

#[derive(Debug, PartialEq)]
enum BuiltinCommands {
    Echo,
    History,
    Cd,
    Pwd,
    Clear,
    Cat,
    False,
    True,
}

impl FromStr for BuiltinCommands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(BuiltinCommands::Echo),
            "history" => Ok(BuiltinCommands::History),
            "cd" => Ok(BuiltinCommands::Cd),
            "pwd" => Ok(BuiltinCommands::Pwd),
            "clear" => Ok(BuiltinCommands::Clear),
            "cat" => Ok(BuiltinCommands::Cat),
            "false" => Ok(BuiltinCommands::False),
            "true" => Ok(BuiltinCommands::True),
            _ => Err(()),
        }
    }
}

pub fn evaluate(cmd: &Command, shell_state: &mut ShellState) -> Result<(), ()> {
    match BuiltinCommands::from_str(&cmd.keyword) {
        Ok(BuiltinCommands::Echo) => Ok(echo::run(cmd, shell_state)),
        Ok(BuiltinCommands::History) => Ok(history::run(shell_state)),
        Ok(BuiltinCommands::Clear) => Ok(clear::run(shell_state)),
        Ok(BuiltinCommands::Cat) => Ok(cat::run(cmd, shell_state)),
        Ok(BuiltinCommands::False) => Ok(bools::false_builtin(shell_state)),
        Ok(BuiltinCommands::True) => Ok(bools::true_builtin(shell_state)),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_commands_from_str_when_defined() {
        assert!(BuiltinCommands::from_str("echo").is_ok());
        assert_eq!(BuiltinCommands::from_str("echo"), Ok(BuiltinCommands::Echo));
    }

    #[test]
    fn test_builtin_commands_from_str_when_undefined() {
        assert!(BuiltinCommands::from_str("notabuiltin").is_err());
    }
}
