use std::fmt::Display;
use crate::parser::token::Token;

#[derive(Debug, PartialEq)]
pub enum Exception {
    AsyncIsUnSupported,
    CommandHasNoCharacters,
    TokenCannotBeParsed(Token),
    TokensCannotBeParsed(String), // near token todo: change to token
    TokenIsNotALogicalExpr(String), // which token todo: change to token
    ConversionNotImplemented(String),
    Unexpected(String)
}
