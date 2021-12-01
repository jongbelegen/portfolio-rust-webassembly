use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Exception {
    AsyncIsUnSupported,
    CommandHasNoCharacters,
    TokensCannotBeParsed(String),
    TokenIsNotALogicalExpr(String),
    ConversionNotImplemented(String),
}
