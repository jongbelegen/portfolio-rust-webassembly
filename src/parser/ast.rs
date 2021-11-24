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
    LogicalExpression {
        op: LogicalExpressionOp,
        left: Box<AstItem>,
        right: Box<AstItem>,
    },
    Pipeline(Vec<AstItem>),
    Debug,
}
use AstItem::{LogicalExpression, Pipeline};

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

fn group_by_pipeline(tokens: &[Token]) -> Option<Vec<&[Token]>> {
    let tokens: Vec<&[Token]> = tokens.split(|token| token == &Token::Pipeline).collect();

    
    match tokens.len() {
        1 => None,
        _ => Some(tokens),
    }
}

// ast prioritizes tokens to be evaluated earlier to represent the tree as how it should be executed
// tree will be executed depth-first

fn parse_to_ast(tokens: &[Token]) -> AstItem {
    match split_last_by_logical_expr(tokens) {
        Some((logical_token, left, right)) => LogicalExpression {
            op: logical_token,
            left: Box::new(parse_to_ast(left)),
            right: Box::new(parse_to_ast(right)),
        },
        None => match group_by_pipeline(tokens) {
            Some(tokens) => {
                let tokens: Vec<_> = tokens
                    .into_iter()
                    .map(|token_slice| parse_to_ast(token_slice))
                    .collect();
                Pipeline(tokens)
            }
            None => match tokens.first().unwrap() {
                Token::Raw(str) => AstItem::Command { raw: str.clone() },
                _ => AstItem::Debug,
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::AstItem::*;

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

        assert_eq!(group_by_pipeline(tokens), Some(expected));
    }

    #[test]
    fn test_group_by_pipeline_when_pipelines_are_not_defined() {
        let tokens = &[Token::And, Token::Async];
        assert_eq!(group_by_pipeline(tokens), None);
    }
}
