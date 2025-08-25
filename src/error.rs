use crate::{lexer::error::LexerError, parser::error::ParserError};

#[derive(Debug, Clone)]
pub enum OdinError<'source> {
    LexerError(LexerError<'source>),
    ParserError(ParserError),
}

impl<'source> std::fmt::Display for OdinError<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OdinError::LexerError(err) => write!(f, "Lexer error: {err}")?,
            OdinError::ParserError(err) => write!(f, "Parser error: {err}")?,
        }
        Ok(())
    }
}

impl<'source> From<LexerError<'source>> for OdinError<'source> {
    fn from(e: LexerError<'source>) -> OdinError<'source> {
        OdinError::LexerError(e)
    }
}

impl<'source> From<ParserError> for OdinError<'source> {
    fn from(e: ParserError) -> OdinError<'source> {
        OdinError::ParserError(e)
    }
}
