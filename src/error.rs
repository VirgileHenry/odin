#[derive(Debug, Clone)]
pub enum OdinError {
    LexerError(crate::lexer::LexerError),
}

impl std::fmt::Display for OdinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OdinError::LexerError(err) => write!(f, "Lexer error: {err}")?,
        }
        Ok(())
    }
}

impl From<crate::lexer::LexerError> for OdinError {
    fn from(e: crate::lexer::LexerError) -> OdinError {
        OdinError::LexerError(e)
    }
}
