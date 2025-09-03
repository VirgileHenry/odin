#[derive(Debug, Clone)]
pub enum OdinError<'source> {
    LexerError(crate::lexer::LexerError<'source>),
}

impl<'source> std::fmt::Display for OdinError<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OdinError::LexerError(err) => write!(f, "Lexer error: {err}")?,
        }
        Ok(())
    }
}

impl<'source> From<crate::lexer::LexerError<'source>> for OdinError<'source> {
    fn from(e: crate::lexer::LexerError<'source>) -> OdinError<'source> {
        OdinError::LexerError(e)
    }
}
