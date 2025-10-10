/// Errors that can be thrown by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    NoTokenMatch(String),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::NoTokenMatch(tokens) => write!(f, "No tokens matched for: \"{tokens}\""),
        }
    }
}

impl std::error::Error for LexerError {}
