/// Errors that can be thrown by the lexer.
#[derive(Debug, Clone)]
pub enum LexerError<'source> {
    UnclosedCommentBlock {
        opening_delimiter: crate::lexer::CommentBlockDelimiter,
        span: crate::lexer::span::Span<'source>,
    },
    UnclosedCost {
        span: crate::lexer::span::Span<'source>,
    },
    NoTokenMatch {
        span: crate::lexer::span::Span<'source>,
    },
    InvalidBraceCost {
        span: crate::lexer::span::Span<'source>,
    },
    InvalidNumeric {
        span: crate::lexer::span::Span<'source>,
    },
}

impl<'source> std::fmt::Display for LexerError<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::InvalidBraceCost { span } => {
                write!(f, "Invalid cost within braces: \"{}\"", span.text)?;
            }
            LexerError::InvalidNumeric { span } => write!(f, "Invalid number: \"{}\"", span.text)?,
            LexerError::NoTokenMatch { span } => {
                write!(f, "No token could match \"{}\"", span.text)?
            }
            LexerError::UnclosedCommentBlock {
                opening_delimiter,
                span,
            } => write!(
                f,
                "Comment block opened with \"{}\" was never closed: \"{}\"",
                opening_delimiter, span.text
            )?,
            LexerError::UnclosedCost { span } => write!(
                f,
                "Cost block opened with \"{{\" was never closed: \"{}\"",
                span.text
            )?,
        }
        Ok(())
    }
}
