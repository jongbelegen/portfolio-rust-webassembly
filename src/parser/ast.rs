#![feature(slice_patterns)]

use crate::parser::token::Token;

#[derive(Debug, PartialEq)]
pub enum LogicalExpressionOp {
    Or,
    And,
}

impl From<&Token> for LogicalExpressionOp {
    fn from(token: &Token) -> LogicalExpressionOp {
        match token {
            Token::Or => LogicalExpressionOp::Or,
            Token::And => LogicalExpressionOp::And,
            token => panic!("{:?} is not a logical expression operator", token),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AstItem {
    Command {
        raw: String,
    },
    Script(Vec<AstItem>),
    LogicalExpression {
        op: LogicalExpressionOp,
        left: Box<AstItem>,
        right: Box<AstItem>,
    },
    Pipeline(Vec<AstItem>),
    Debug,
}
use AstItem::{LogicalExpression, Pipeline, Script};

fn split_last_by_logical_expr(
    tokens: &[Token],
) -> Option<(LogicalExpressionOp, &[Token], &[Token])> {
    let maybe_index = tokens
        .iter()
        .rposition(|token| token == &Token::Or || token == &Token::And);

    match maybe_index {
        Some(i) => {
            let (left, right_with_token) = tokens.split_at(i);
            let logical_op = LogicalExpressionOp::from(right_with_token.get(0).unwrap());

            Some((logical_op, left, &right_with_token[1..]))
        }
        None => None,
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

pub fn parse_to_ast(tokens: &[Token]) -> AstItem {
    if let Some(groups) = group_by_token(tokens, &Token::Semicolon) {
        let tokens: Vec<_> = groups
            .into_iter()
            .map(|token_slice| parse_to_ast(token_slice))
            .collect();

        return Script(tokens);
    }

    if let Some((logical_token, left, right)) = split_last_by_logical_expr(tokens) {
        return LogicalExpression {
            op: logical_token,
            left: Box::new(parse_to_ast(left)),
            right: Box::new(parse_to_ast(right)),
        };
    }

    if let Some(groups) = group_by_token(tokens, &Token::Pipeline) {
        let tokens: Vec<_> = groups
            .into_iter()
            .map(|token_slice| parse_to_ast(token_slice))
            .collect();

        return Pipeline(tokens);
    }

    match &tokens {
        &[Token::Raw(str) ] => AstItem::Command { raw: str.clone() },
        tokens => panic!("All tokens should have been processed, and we should have been left with only an command : {:?}", tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::AstItem::*;
    use super::*;
    use crate::parser::token::Token::Semicolon;

    #[test]
    fn test_logical_expression() {
        let expect = LogicalExpression {
            op: LogicalExpressionOp::Or,
            left: Box::new(Command {
                raw: String::from("a"),
            }),
            right: Box::new(Command {
                raw: String::from("b"),
            }),
        };

        // a || b
        let from = &[
            Token::Raw(String::from("a")),
            Token::Or,
            Token::Raw(String::from("b")),
        ];

        assert_eq!(parse_to_ast(from), expect);
    }

    // The last logical expression token should be the top of the tree
    // since this represents execution order
    #[test]
    fn test_multiple_logical_expression() {
        let expect = LogicalExpression {
            op: LogicalExpressionOp::And,
            left: Box::new(LogicalExpression {
                op: LogicalExpressionOp::Or,
                left: Box::new(Command {
                    raw: String::from("a"),
                }),
                right: Box::new(Command {
                    raw: String::from("b"),
                }),
            }),
            right: Box::new(Command {
                raw: String::from("c"),
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

        assert_eq!(parse_to_ast(from), expect);
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
                raw: String::from("a"),
            },
            Command {
                raw: String::from("b"),
            },
        ]);

        assert_eq!(parse_to_ast(from), expect)
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
                raw: String::from("a"),
            },
            Command {
                raw: String::from("b"),
            },
            Command {
                raw: String::from("c"),
            },
        ]);

        assert_eq!(parse_to_ast(from), expect)
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
                raw: String::from("a"),
            },
            Command {
                raw: String::from("b"),
            },
        ]);

        assert_eq!(parse_to_ast(from), expect)
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
                raw: String::from("a"),
            },
            Command {
                raw: String::from("b"),
            },
            Command {
                raw: String::from("c"),
            },
        ]);

        assert_eq!(parse_to_ast(from), expect)
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
            Some((LogicalExpressionOp::And, &tokens[..3], &tokens[4..]))
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
}
