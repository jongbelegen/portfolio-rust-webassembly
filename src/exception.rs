use crate::parser::token::Token;

pub enum Exception {
    AsyncIsUnSupported,
    CommandHasNoCharacters
}
