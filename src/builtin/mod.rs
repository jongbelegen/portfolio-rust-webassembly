use std::str::FromStr;

use crate::{Command, ShellOutput};
use crate::executor::history;

mod echo;
pub mod clear;

enum BuiltinCommands {
    Echo,
    History,
    Cd,
    Pwd,
    Clear
}

impl FromStr for BuiltinCommands {
    type Err = ();

    fn from_str(s : &str) -> Result<Self, Self::Err> {
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

pub fn evaluate(command: Command) -> Result<ShellOutput, ()> {
    match BuiltinCommands::from_str(&command.keyword) {
        Ok(BuiltinCommands::Echo) => Ok(echo::run(command.args)),
        Ok(BuiltinCommands::History) => Ok(history::run()),
        Ok(BuiltinCommands::Clear) => Ok(clear::run()),
        _ => Err(()) // temp, all builtins will be implemented
    }
}
