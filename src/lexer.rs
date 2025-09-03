mod error;
pub mod span;
pub mod tokens;

const DELIMITERS: [char; 2] = ['(', ')'];
const DISCARDABLE_PREFIX: [char; 1] = [' '];

pub use error::LexerError;

/// Create a vec of Terminals from a string. Can fail, and will return an error.
pub fn lex<'source>(
    input: &'source str,
) -> Result<Vec<tokens::Token<'source>>, error::LexerError<'source>> {
    let mut input = input;
    let mut result = Vec::new();

    loop {
        for discardable_prefix in DISCARDABLE_PREFIX {
            input = match input.strip_prefix(discardable_prefix) {
                Some(new) => new,
                None => input,
            };
        }

        if input.is_empty() {
            break;
        }
    }

    Ok(result)
}
