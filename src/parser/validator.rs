use crate::exception::Exception;
use crate::exception::Exception::TokensCannotBeParsed;
use crate::parser::token::Token;
use std::any::Any;

pub fn is_valid_token_order(tokens: &[Token]) -> Result<(), Exception> {
    dbg!(&tokens.first().unwrap().type_id());

    match &tokens {
        &[token, ..] if token.type_id() != Token::Raw.type_id() => {
            Err(Exception::TokensCannotBeParsed(format!("{}", token)))
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_should_should_start_with_raw() {
        assert_eq!(
            is_valid_token_order(&[Token::Pipeline]),
            Err(Exception::TokensCannotBeParsed(String::from("|")))
        );
    }
}
