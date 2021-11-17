use crate::parser::ast::ControlToken::Semicolon;
use crate::parser::token::Token;

#[derive(Debug, PartialEq)]
pub struct Command {
    raw: String,
}

#[derive(Debug, PartialEq)]
enum LogicalExpressionOp {
    Or,
    And,
}

#[derive(Debug, PartialEq)]
pub struct LogicalExpression {
    op: LogicalExpressionOp,
    left: Box<AstItem>,
    right: Box<AstItem>,
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
    Command(Command),
    LogicalExpression(LogicalExpression),
    Debug,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ControlToken {
    Or,
    And,
    Semicolon,
}

// TODO: accept slice
pub fn parse_to_ast_entry(mut tokens: Vec<Token>) -> AstItem {
    parse_to_ast(tokens.as_slice())
}

fn split_last_by_logical_expr(
    tokens: &[Token],
) -> Result<(LogicalExpressionOp, &[Token], &[Token]), &[Token]> {
    let maybe_index = tokens
        .iter()
        .rposition(|token| token == &Token::Or || token == &Token::And);

    match maybe_index {
        Some(i) => {
            let (left, right_with_token) = tokens.split_at(i);
            let logical_op = LogicalExpressionOp::from(right_with_token.get(0).unwrap());

            Ok((logical_op, left, &right_with_token[1..]))
        }
        None => Err(tokens),
    }
}

fn parse_to_ast(tokens: &[Token]) -> AstItem {
    match split_last_by_logical_expr(tokens) {
        Ok((logical_token, left, right)) => AstItem::LogicalExpression(LogicalExpression {
            op: logical_token,
            left: Box::new(parse_to_ast(left)),
            right: Box::new(parse_to_ast(right)),
        }),
        Err(tokens) => match tokens.first().unwrap() {
            Token::Raw(str) => AstItem::Command(Command { raw: str.clone() }),
            _ => AstItem::Debug,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_expression() {
        let expect = AstItem::LogicalExpression(LogicalExpression {
            op: LogicalExpressionOp::Or,
            left: Box::new(AstItem::Command(Command {
                raw: String::from("a"),
            })),
            right: Box::new(AstItem::Command(Command {
                raw: String::from("b"),
            })),
        });

        // a || b
        let from = vec![
            Token::Raw(String::from("a")),
            Token::Or,
            Token::Raw(String::from("b")),
        ];

        assert_eq!(parse_to_ast_entry(from), expect);
    }

    // The last logical expression token should be the top of the tree
    // since this represents execution order
    #[test]
    fn test_multiple_logical_expression() {
        let expect = AstItem::LogicalExpression(LogicalExpression {
            op: LogicalExpressionOp::And,
            left: Box::new(AstItem::LogicalExpression(LogicalExpression {
                op: LogicalExpressionOp::Or,
                left: Box::new(AstItem::Command(Command {
                    raw: String::from("a"),
                })),
                right: Box::new(AstItem::Command(Command {
                    raw: String::from("b"),
                })),
            })),
            right: Box::new(AstItem::Command(Command {
                raw: String::from("c"),
            })),
        });

        // a || b && c
        let from = vec![
            Token::Raw(String::from("a")),
            Token::Or,
            Token::Raw(String::from("b")),
            Token::And,
            Token::Raw(String::from("c")),
        ];

        assert_eq!(parse_to_ast_entry(from), expect);
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
            Ok((LogicalExpressionOp::And, &tokens[..3], &tokens[4..]))
        )
    }
}
