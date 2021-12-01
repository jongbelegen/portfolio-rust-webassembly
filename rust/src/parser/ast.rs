mod conversions;

use crate::parser::token::Token;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum LogicalExpressionOp {
    Or,
    And,
}

impl TryFrom<&Token> for LogicalExpressionOp {
    type Error = Exception;
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Or => Ok(LogicalExpressionOp::Or),
            Token::And => Ok(LogicalExpressionOp::And),
            token => Err(Exception::TokenIsNotALogicalExpr(format!("{:?}", token))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AstItem {
    Command {
        keyword: String,
        args: Vec<String>,
    },
    Script(Vec<AstItem>),
    LogicalExpression {
        op: LogicalExpressionOp,
        left: Box<AstItem>,
        right: Box<AstItem>,
    },
    Pipeline(Vec<AstItem>),
}

impl TryFrom<&Token> for AstItem {
    type Error = Exception;
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Raw(token) => conversions::convert_token_to_command(token),
            token => Err(Exception::ConversionNotImplemented(format!("{:?}", token))),
        }
    }
}

use crate::exception::Exception;
use AstItem::{Command, LogicalExpression, Pipeline, Script};

fn split_last_by_logical_expr(
    tokens: &[Token],
) -> Result<Option<(LogicalExpressionOp, &[Token], &[Token])>, Exception> {
    let maybe_index = tokens
        .iter()
        .rposition(|token| token == &Token::Or || token == &Token::And);

    match maybe_index {
        Some(i) => {
            let (left, right_with_token) = tokens.split_at(i);
            let logical_op = LogicalExpressionOp::try_from(right_with_token.get(0).unwrap())?;

            Ok(Some((logical_op, left, &right_with_token[1..])))
        }
        None => Ok(None),
    }
}

fn group_by_pipeline(tokens: &[Token]) -> Result<Option<Vec<&[Token]>>, Exception> {
    if !tokens.contains(&Token::Pipeline) {
        return Ok(None);
    }

    let tokens: Vec<&[Token]> = tokens.split(|t| t == &Token::Pipeline).collect();

    match tokens.as_slice() {
        [&[], ..] | [.., &[]] => Err(Exception::TokenCannotBeParsed(Token::Pipeline)),
        _ => Ok(Some(tokens)),
    }
}

// ast prioritizes tokens to be evaluated earlier to represent the tree as how it should be executed
// tree will be executed depth-first

pub fn parse_to_ast(tokens: &[Token]) -> Result<AstItem, Exception> {
    if tokens.contains(&Token::Semicolon) {
        let tokens: Result<Vec<_>, _> = tokens
            .split(|t| t == &Token::Semicolon)
            .filter(|slice| !slice.is_empty())
            .map(|token_slice| parse_to_ast(token_slice))
            .collect();

        return Ok(Script(tokens?));
    }

    if let Some((logical_token, left, right)) = split_last_by_logical_expr(tokens)? {
        if left.is_empty() || right.is_empty() {
            return Err(Exception::TokenCannotBeParsed(Token::from(&logical_token)));
        }

        return Ok(LogicalExpression {
            op: logical_token,
            left: Box::new(parse_to_ast(left)?),
            right: Box::new(parse_to_ast(right)?),
        });
    }

    if let Some(groups) = group_by_pipeline(tokens)? {
        let tokens: Result<Vec<_>, _> = groups
            .into_iter()
            .map(|token_slice| parse_to_ast(token_slice))
            .collect();

        return Ok(Pipeline(tokens?));
    }

    if tokens.contains(&Token::Async) {
        return Err(Exception::AsyncIsUnSupported);
    }

    match &tokens {
        &[token] => AstItem::try_from(token),
        tokens => Err(Exception::Unexpected(format!("Ast should have processed all non raw tokens: {:?}", tokens))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_expression() {
        let expect = LogicalExpression {
            op: LogicalExpressionOp::Or,
            left: Box::new(Command {
                keyword: String::from("a"),
                args: Vec::new(),
            }),
            right: Box::new(Command {
                keyword: String::from("b"),
                args: Vec::new(),
            }),
        };

        // a || b
        let from = &[create_raw("a"), Token::Or, create_raw("b")];

        assert_eq!(parse_to_ast(from), Ok(expect));
    }

    // The last logical expression token should be the top of the tree
    // since this represents execution order
    #[test]
    fn test_multiple_logical_expression() {
        let expect = LogicalExpression {
            op: LogicalExpressionOp::And,
            left: Box::new(LogicalExpression {
                op: LogicalExpressionOp::Or,
                left: Box::new(AstItem::Command {
                    keyword: String::from("a"),
                    args: Vec::new(),
                }),
                right: Box::new(AstItem::Command {
                    keyword: String::from("b"),
                    args: Vec::new(),
                }),
            }),
            right: Box::new(AstItem::Command {
                keyword: String::from("c"),
                args: Vec::new(),
            }),
        };

        // a || b && c
        let from = &[
            create_raw("a"),
            Token::Or,
            create_raw("b"),
            Token::And,
            create_raw("c"),
        ];

        assert_eq!(parse_to_ast(from), Ok(expect));
    }

    #[test]
    fn logical_expression_should_have_values() {
        assert_eq!(
            parse_to_ast(&[Token::And]),
            Err(Exception::TokenCannotBeParsed(Token::And))
        )
    }

    #[test]
    fn logical_expression_should_have_a_left_value() {
        assert_eq!(
            parse_to_ast(&[Token::And, create_raw("a")]),
            Err(Exception::TokenCannotBeParsed(Token::And))
        )
    }

    #[test]
    fn logical_expression_should_have_a_right_value() {
        assert_eq!(
            parse_to_ast(&[create_raw("a"), Token::And]),
            Err(Exception::TokenCannotBeParsed(Token::And))
        )
    }

    #[test]
    fn test_pipeline() {
        let from = &[create_raw("a"), Token::Pipeline, create_raw("b")];

        let expect = Pipeline(vec![
            Command {
                keyword: String::from("a"),
                args: Vec::new(),
            },
            Command {
                keyword: String::from("b"),
                args: Vec::new(),
            },
        ]);

        assert_eq!(parse_to_ast(from), Ok(expect))
    }

    #[test]
    fn test_multiple_pipelines() {
        let from = &[
            create_raw("a"),
            Token::Pipeline,
            create_raw("b"),
            Token::Pipeline,
            create_raw("c"),
        ];

        let expect = Pipeline(vec![
            Command {
                keyword: String::from("a"),
                args: Vec::new(),
            },
            Command {
                keyword: String::from("b"),
                args: Vec::new(),
            },
            Command {
                keyword: String::from("c"),
                args: Vec::new(),
            },
        ]);

        assert_eq!(parse_to_ast(from), Ok(expect))
    }

    #[test]
    fn pipeline_should_have_values() {
        let tokens = &[Token::Pipeline];
        assert_eq!(
            parse_to_ast(tokens),
            Err(Exception::TokenCannotBeParsed(Token::Pipeline))
        );
    }

    #[test]
    fn pipeline_should_have_a_left_value() {
        let tokens = &[create_raw("a"), Token::Pipeline];
        assert_eq!(
            parse_to_ast(tokens),
            Err(Exception::TokenCannotBeParsed(Token::Pipeline))
        );
    }

    #[test]
    fn pipeline_should_have_a_right_value() {
        let tokens = &[Token::Pipeline, create_raw("a")];
        assert_eq!(
            parse_to_ast(tokens),
            Err(Exception::TokenCannotBeParsed(Token::Pipeline))
        );
    }

    #[test]
    fn test_semicolon() {
        let from = &[create_raw("a"), Token::Semicolon, create_raw("b")];

        let expect = Script(vec![
            Command {
                keyword: String::from("a"),
                args: Vec::new(),
            },
            Command {
                keyword: String::from("b"),
                args: Vec::new(),
            },
        ]);

        assert_eq!(parse_to_ast(from), Ok(expect))
    }

    #[test]
    fn test_semicolon_with_only_empty_values() {
        let tokens = &[Token::Semicolon];
        assert_eq!(parse_to_ast(tokens), Ok(Script(Vec::new())));
    }

    #[test]
    fn test_semicolon_with_only_left_value() {
        let tokens = &[create_raw("a"), Token::Semicolon];
        assert_eq!(parse_to_ast(tokens), Ok(Script(vec![create_cmd("a")])));
    }

    #[test]
    fn test_semicolon_with_only_right_value() {
        let tokens = &[Token::Semicolon, create_raw("a")];
        assert_eq!(parse_to_ast(tokens), Ok(Script(vec![create_cmd("a")])));
    }

    #[test]
    fn test_multiple_semicolons() {
        let from = &[
            create_raw("a"),
            Token::Semicolon,
            create_raw("b"),
            Token::Semicolon,
            create_raw("c"),
        ];

        let expect = Script(vec![
            Command {
                keyword: String::from("a"),
                args: Vec::new(),
            },
            Command {
                keyword: String::from("b"),
                args: Vec::new(),
            },
            Command {
                keyword: String::from("c"),
                args: Vec::new(),
            },
        ]);

        assert_eq!(parse_to_ast(from), Ok(expect))
    }

    #[test]
    fn test_split_last_by_logical_expr() {
        let tokens = &[
            Token::Semicolon,
            Token::Or,
            Token::Semicolon,
            Token::And,
            Token::Async,
        ];

        assert_eq!(
            split_last_by_logical_expr(tokens),
            Ok(Some((LogicalExpressionOp::And, &tokens[..3], &tokens[4..])))
        )
    }

    #[test]
    fn test_group_by_pipeline_when_pipelines_are_defined() {
        let tokens = &[Token::Or, Token::Pipeline, Token::And, Token::Async];
        let expected: Vec<&[Token]> = vec![&[Token::Or], &[Token::And, Token::Async]];

        assert_eq!(group_by_pipeline(tokens), Ok(Some(expected)));
    }

    #[test]
    fn test_group_by_pipeline_when_pipelines_are_not_defined() {
        let tokens = &[Token::And, Token::Async];
        assert_eq!(group_by_pipeline(tokens), Ok(None));
    }

    fn create_raw(token: &str) -> Token {
        Token::Raw(String::from(token))
    }

    fn create_cmd(token: &str) -> AstItem {
        AstItem::try_from(&Token::Raw(String::from(token))).unwrap()
    }
}
