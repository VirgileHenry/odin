mod error;
pub mod span;
pub mod tokens;

pub use error::LexerError;

/// Preprocess a card oracle text to properly lex it.
pub fn preprocess(card_name: &str, oracle_text: &str) -> String {
    let card_name_lowercase = card_name.to_ascii_lowercase();
    let oracle_text_lowercase = oracle_text.to_ascii_lowercase();

    let result = oracle_text_lowercase.replace(&card_name_lowercase, "~");

    result
}

/// Create a vec of Terminals from a string. Can fail, and will return an error if it does.
pub fn lex<'src>(input: &'src str) -> Result<Vec<tokens::Token<'src>>, error::LexerError<'src>> {
    /* List of non words token we also want to match */
    const MATCHABLE_NON_WORDS: &[&'static str] = &["\\.", ",", "(", ")", "'", "{", "}", "~"];

    let matchable_non_words: String = MATCHABLE_NON_WORDS.iter().cloned().collect();
    let raw_token_pattern = format!("((:?\\b\\w+\\b)+)|([{}])", matchable_non_words);
    let raw_token_regex = regex::Regex::new(&raw_token_pattern).expect("Failed to compile regex!");

    let mut raw_tokens = raw_token_regex.find_iter(input);
    let mut previous_raw_token: Option<span::Span> = None;

    let mut result = Vec::new();

    while let Some(next) = raw_tokens.next() {
        let span = match previous_raw_token {
            None => span::Span {
                start: next.start(),
                length: next.len(),
                text: next.as_str(),
            },
            Some(prev) => span::Span {
                start: prev.start,
                length: next.end() - prev.start,
                text: &input[prev.start..next.end()],
            },
        };
        match tokens::Token::try_from_str(span) {
            Some(token) => result.push(token),
            None => previous_raw_token = Some(span),
        }
    }

    match previous_raw_token {
        None => Ok(result),
        Some(prev) => Err(error::LexerError::NoTokenMatch { span: prev }),
    }
}
