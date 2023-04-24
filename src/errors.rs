use crate::{
    lexer::error::OdinLexerError,
    parser::error::OdinParserError
};

/// All the error this lib can produce.
/// Basically wraps up lexer and parser errors.
#[derive(Debug, Clone)]
pub enum OdinErrors {
    LexerError(OdinLexerError),
    ParserError(OdinParserError),
}

impl From<OdinLexerError> for OdinErrors {
    fn from(e: OdinLexerError) -> Self {
        OdinErrors::LexerError(e)
    }
}

impl From<OdinParserError> for OdinErrors {
    fn from(e: OdinParserError) -> Self {
        OdinErrors::ParserError(e)
    }
}