mod error;
pub mod span;
pub mod tokens;

const DELIMITERS: [char; 2] = ['(', ')'];
const DISCARDABLE_PREFIX: [char; 1] = [' '];

pub use error::LexerError;

/// Create a vec of Terminals from a string. Can fail, and will return an error if it does.
pub fn lex<'src>(input: &'src str) -> Result<Vec<tokens::Token<'src>>, error::LexerError<'src>> {
    let mut source = input;
    let mut offset = 0;
    let mut result = Vec::new();

    loop {
        let (stripped, consumed) = strip_prefixes(source)?;
        source = stripped;
        offset += consumed;

        match tokens::Token::try_from_str(source, offset) {
            Some((token, consumed)) => {
                result.push(token);
                offset += consumed;
                source = &source[consumed..];
                if token.is_eof() {
                    break Ok(result);
                }
            }
            None => {
                break Err(error::LexerError::NoTokenMatch {
                    span: span::Span {
                        start: offset,
                        length: input.len() - offset,
                        text: source,
                    },
                })
            }
        }
    }
}

fn strip_prefixes<'src>(source: &'src str) -> Result<(&'src str, usize), error::LexerError<'src>> {
    unimplemented!()
}
