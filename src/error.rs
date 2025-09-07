#[derive(Debug, Clone)]
pub enum OdinError<'src> {
    LexerError(crate::lexer::LexerError<'src>),
}

impl<'src> std::fmt::Display for OdinError<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OdinError::LexerError(err) => write!(f, "Lexer error: {err}")?,
        }
        Ok(())
    }
}

impl<'src> From<crate::lexer::LexerError<'src>> for OdinError<'src> {
    fn from(e: crate::lexer::LexerError<'src>) -> OdinError<'src> {
        OdinError::LexerError(e)
    }
}
