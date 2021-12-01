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

fn group_by_token<'a>(tokens: &'a [Token], token: &Token) -> Option<Vec<&'a [Token]>> {
    let tokens: Vec<&[Token]> = tokens.split(|t| t == token).collect();

    if tokens.len() == 1 {
        None
    } else {
        Some(tokens)
    }
}

// ast prioritizes tokens to be evaluated earlier to represent the tree as how it should be executed
// tree will be executed depth-first

pub fn parse_to_ast(tokens: &[Token]) -> Result<AstItem, Exception> {
    if tokens.contains(&Token::Semicolon) {
        let tokens: Result<Vec<_>, _> = tokens
            .split(|t| t == &Token::Semicolon)
            .map(|token_slice| parse_to_ast(token_slice))
            .collect();

        return Ok(Script(tokens?));
    }

    if let Some((logical_token, left, right)) = split_last_by_logical_expr(tokens)? {
        return Ok(LogicalExpression {
            op: logical_token,
            left: Box::new(parse_to_ast(left)?),
            right: Box::new(parse_to_ast(right)?),
        });
    }

    if let Some(groups) = group_by_token(tokens, &Token::Pipeline) {
        let tokens: Result<Vec<_>, _> = groups
            .into_iter()
            .map(|token_slice| parse_to_ast(token_slice))
            .collect();

        return Ok(Pipeline(tokens?));
    }

    match &tokens {
        &[token] => AstItem::try_from(token),
        tokens => Err(Exception::TokensCannotBeParsed(format!("{:?}", tokens))),
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
        let from = &[
            Token::Raw(String::from("a")),
            Token::Or,
            Token::Raw(String::from("b")),
        ];

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
            Token::Raw(String::from("a")),
            Token::Or,
            Token::Raw(String::from("b")),
            Token::And,
            Token::Raw(String::from("c")),
        ];

        assert_eq!(parse_to_ast(from), Ok(expect));
    }

    #[test]
    fn test_pipeline() {
        let from = &[
            Token::Raw(String::from("a")),
            Token::Pipeline,
            Token::Raw(String::from("b")),
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
        ]);

        assert_eq!(parse_to_ast(from), Ok(expect))
    }

    #[test]
    fn test_multiple_pipelines() {
        let from = &[
            Token::Raw(String::from("a")),
            Token::Pipeline,
            Token::Raw(String::from("b")),
            Token::Pipeline,
            Token::Raw(String::from("c")),
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
    fn test_semicolon() {
        let from = &[
            Token::Raw(String::from("a")),
            Token::Semicolon,
            Token::Raw(String::from("b")),
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
        ]);

        assert_eq!(parse_to_ast(from), Ok(expect))
    }

    #[test]
    fn test_multiple_semicolons() {
        let from = &[
            Token::Raw(String::from("a")),
            Token::Semicolon,
            Token::Raw(String::from("b")),
            Token::Semicolon,
            Token::Raw(String::from("c")),
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

        assert_eq!(group_by_token(tokens, &Token::Pipeline), Some(expected));
    }

    #[test]
    fn test_group_by_pipeline_when_pipelines_are_not_defined() {
        let tokens = &[Token::And, Token::Async];
        assert_eq!(group_by_token(tokens, &Token::Pipeline), None);
    }

    // In real life this should be caught by the token validator that is being called before the ast is being ran
    #[test]
    fn test_incompatible_order_of_tokens() {
        assert_eq!(
            parse_to_ast(&[Token::Or, Token::And]),
            Err(Exception::TokensCannotBeParsed(String::from("[]")))
        );

        assert_eq!(
            parse_to_ast(&[Token::Or]),
            Err(Exception::TokensCannotBeParsed(String::from("[]")))
        )
    }
}
