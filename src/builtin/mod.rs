use std::str::FromStr;

use crate::command::Command;
use crate::executor::history;
use crate::shell_state::ShellState;

pub mod clear;
mod echo;

enum BuiltinCommands {
    Echo,
    History,
    Cd,
    Pwd,
    Clear,
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
            _ => Err(()),
        }
    }
}

pub fn evaluate(cmd: &Command, shell_state: &mut ShellState) -> Result<(), ()> {
    match BuiltinCommands::from_str(&cmd.keyword) {
        Ok(BuiltinCommands::Echo) => Ok(echo::run(cmd, shell_state)),
        Ok(BuiltinCommands::History) => Ok(history::run(shell_state)),
        Ok(BuiltinCommands::Clear) => Ok(clear::run(shell_state)),
        _ => Err(()),
    }
}
