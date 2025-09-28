mod error;
pub mod span;
pub mod tokens;

const DISCARDABLE_PREFIX: [char; 1] = [' '];

pub use error::LexerError;

/// Create a vec of Terminals from a string. Can fail, and will return an error if it does.
pub fn lex<'src>(input: &'src str) -> Result<Vec<tokens::Token<'src>>, error::LexerError<'src>> {
    let mut source = input;
    let mut offset = 0;
    let mut result = Vec::new();

    loop {
        let (stripped, consumed) = strip_prefixes(source);
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

fn strip_prefixes<'src>(source: &'src str) -> (&'src str, usize) {
    let mut consumed = 0;
    let mut chars_indices = source.char_indices();

    while let Some((char_index, char_val)) = chars_indices.next() {
        if DISCARDABLE_PREFIX.contains(&char_val) {
            consumed = char_index;
        } else {
            break; /* stop on the first non discardable prefix */
        }
    }

    (&source[consumed..], consumed)
}
