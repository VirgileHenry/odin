
/// Errors that can be thrown by the lexer.
#[derive(Debug, Clone)]
pub enum OdinLexerError {
    /// A comment block was oppened with an oppening parnthesis '(' but never closed.
    CommentBlockNeverClose,
    /// No more characters to read, but could not made the remaining characters into tokens.
    NoTokenMatch(String),
    /// The tokens inside the braces ({}) was not recognized.
    InvalidBraceCost(String),
}

