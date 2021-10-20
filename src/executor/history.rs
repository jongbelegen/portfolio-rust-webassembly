use crate::shell_state::ShellState;
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Error;
use std::path::PathBuf;

const HISTORY_PATH: &str = "history.txt";

#[derive(Debug, PartialEq)]
pub struct History {
    input: String,
    date: DateTime<Utc>,
}

fn get_path() -> PathBuf {
    PathBuf::from(HISTORY_PATH)
}

fn read_file() -> Result<String, Error> {
    let mut file = File::open(get_path())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn run(shell_state: &mut ShellState) {
    let read_file = read_file().expect("Reading of history file failed");
    let history_list: Vec<String> = read_file
        .lines()
        .map(|line| parse_line(&String::from(line)))
        .map(format_history)
        .collect();

    shell_state.output.set_stdout(history_list.join("\n"));
}

pub fn format_history(history: History) -> String {
    let History { input, date } = history;

    format!("{} {}", date, input)
}

// Every line is expecting following format: " :{timestamp};{command}"
pub fn parse_line(history: &String) -> History {
    let result = history.split_once(";");

    let (date, command) = result.unwrap();

    let date: String = date.chars().skip(2).collect();
    let date = date.parse().unwrap();

    History {
        input: String::from(command),
        date: Utc.timestamp(date, 0),
    }
}

pub fn append(cmd: &String) -> Result<(), Error> {
    if !get_path().exists() {
        File::create(HISTORY_PATH)?;
    }

    let timestamp = Utc::now().timestamp();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(HISTORY_PATH)
        .unwrap();

    writeln!(file, ": {};{}", timestamp, cmd)?;

    Ok(())
}

#[cfg(test)]
mod history_tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line(&String::from(" : 1626441404;echo eee")),
            History {
                input: String::from("echo eee"),
                date: Utc.timestamp(1626441404, 0),
            }
        );
    }
}
