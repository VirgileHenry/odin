/// Errors that can be thrown by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError<'src> {
    UnclosedCommentBlock {
        span: crate::lexer::span::Span<'src>,
    },
    UnclosedCost {
        span: crate::lexer::span::Span<'src>,
    },
    NoTokenMatch {
        span: crate::lexer::span::Span<'src>,
    },
    InvalidBraceCost {
        span: crate::lexer::span::Span<'src>,
    },
    InvalidNumeric {
        span: crate::lexer::span::Span<'src>,
    },
}

impl<'src> std::fmt::Display for LexerError<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::InvalidBraceCost { span } => {
                write!(f, "Invalid cost within braces: \"{}\"", span.text)?;
            }
            LexerError::InvalidNumeric { span } => write!(f, "Invalid number: \"{}\"", span.text)?,
            LexerError::NoTokenMatch { span } => {
                write!(f, "No token could match \"{}\"", span.text)?
            }
            LexerError::UnclosedCommentBlock { span } => {
                write!(f, "Comment block was never closed: \"{}\"", span.text)?
            }
            LexerError::UnclosedCost { span } => write!(
                f,
                "Cost block opened with \"{{\" was never closed: \"{}\"",
                span.text
            )?,
        }
        Ok(())
    }
}

impl<'src> std::error::Error for LexerError<'src> {}
