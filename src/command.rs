use std::convert::TryFrom;

#[derive(Debug, PartialEq, Default)]
pub struct Command {
    pub keyword: String,
    pub args: Vec<String>,
}

impl TryFrom<&String> for Command {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut command_split = s.split_whitespace().map(|s| s.to_string());

        Ok(Command {
            keyword: command_split.next().ok_or(())?,
            args: command_split.collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_from_str_when_only_keyword_is_provided() {
        let cmd = Command::try_from(&String::from("echo"));

        assert_eq!(
            cmd,
            Ok(Command {
                keyword: String::from("echo"),
                args: Vec::new()
            })
        )
    }

    #[test]
    fn test_command_str_when_keyword_and_args_are_provided() {
        let cmd = Command::try_from(&String::from("echo --foo bar"));

        assert_eq!(
            cmd,
            Ok(Command {
                keyword: String::from("echo"),
                args: vec![String::from("--foo"), String::from("bar")]
            })
        )
    }

    #[test]
    fn test_command_fails() {
        let cmd = Command::try_from(&String::from(""));
        assert_eq!(cmd, Err(()));
    }
}
