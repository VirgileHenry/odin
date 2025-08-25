
/// Errors that can be thrown by the lexer.
#[derive(Debug, Clone)]
pub enum OdinLexerError {
    /// A comment block was opened but never closed.
    /// The error carries the delimiter that was opened for debug purpuses.
    UnclosedCommentBlock {
        opening_delimiter: super::CommentBlockDelimiter,
    },
    /// A cost with braces (e.g., '{T}') was opened but never closed.
    UnclosedCost,
    /// No more characters to read, but could not made the remaining characters into tokens.
    NoTokenMatch(String),
    /// The tokens inside the braces ({}) was not recognized.
    InvalidBraceCost(String),
    /// Attemptes to parse a number, but failed
    InvalidNumeric(String),
}

