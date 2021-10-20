use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Command {
    pub keyword: String,
    pub args: Vec<String>,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command_split = s.split_whitespace().map(|s| s.to_string());

        Ok(Command {
            keyword: command_split.take(1).collect(),
            args: Vec::new(),
        })
    }
}
