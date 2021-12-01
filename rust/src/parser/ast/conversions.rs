use crate::exception::Exception;
use crate::parser::ast::AstItem;

pub fn convert_token_to_command(token: &String) -> Result<AstItem, Exception> {
    let mut command_split = token.split_whitespace().map(|s| s.to_string());

    Ok(AstItem::Command {
        keyword: command_split
            .next()
            .ok_or(Exception::CommandHasNoCharacters)?,
        args: command_split.collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_from_str_when_only_keyword_is_provided() {
        let cmd = convert_token_to_command(&String::from("echo"));

        assert_eq!(
            cmd,
            Ok(AstItem::Command {
                keyword: String::from("echo"),
                args: Vec::new()
            })
        )
    }

    #[test]
    fn test_command_str_when_keyword_and_args_are_provided() {
        let cmd = convert_token_to_command(&String::from("echo --foo bar"));

        assert_eq!(
            cmd,
            Ok(AstItem::Command {
                keyword: String::from("echo"),
                args: vec![String::from("--foo"), String::from("bar")]
            })
        )
    }

    #[test]
    fn test_command_fails() {
        let cmd = convert_token_to_command(&String::from(""));
        assert_eq!(cmd, Err(Exception::CommandHasNoCharacters));
    }
}
