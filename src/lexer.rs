mod error;
pub mod span;
pub mod tokens;

pub use error::LexerError;

/// Preprocess a card oracle text to properly lex it.
pub fn preprocess(card_name: &str, oracle_text: &str) -> String {
    let card_name_lowercase = card_name.to_ascii_lowercase();
    let result = oracle_text;

    let result = result.to_ascii_lowercase();
    let result = remove_comments(&result);
    let result = result.replace(&card_name_lowercase, "~");
    let result = result.replace("\\n", "\n");

    result
}

/// Remove all text within parenthesis in the given source, and returns the newly built string.
fn remove_comments(input: &str) -> String {
    let mut chars = input.chars();
    let mut result = String::with_capacity(input.len());

    while let Some(char) = chars.next() {
        match char {
            '(' => remove_parens(&mut chars),
            c => result.push(c),
        }
    }

    result
}

/// Pop chars from the given iterator until a closing parens is popped.
/// If an oppening parens is popped, will call itelsef recursively.
fn remove_parens<I: Iterator<Item = char>>(chars: &mut I) {
    loop {
        match chars.next() {
            Some(')') => break,
            Some('(') => remove_parens(chars),
            _ => { /* keep popping */ }
        }
    }
}

/// Create a vec of Terminals from a string. Can fail, and will return an error if it does.
pub fn lex<'src>(input: &'src str) -> Result<Vec<tokens::Token<'src>>, error::LexerError> {
    /* List of non words token we also want to match */
    const MATCHABLE_NON_WORDS: &[&'static str] =
        &["\\.", ",", "'", "{", "}", "~", "\\/", ":", "+", "\\-"];

    let matchable_non_words: String = MATCHABLE_NON_WORDS.iter().cloned().collect();
    let raw_token_pattern = format!("(\\b\\w+\\b)|([{}])", matchable_non_words);
    let raw_token_regex = regex::Regex::new(&raw_token_pattern).expect("Failed to compile regex!");

    let mut raw_tokens: std::collections::VecDeque<_> = raw_token_regex.find_iter(input).collect();

    let mut result = Vec::new();

    'outer: while !raw_tokens.is_empty() {
        /* Attempt to parse as much tokens as possible, reducing by one each time */
        for token_count in (0..raw_tokens.len()).rev() {
            let start = raw_tokens[0].start();
            let end = raw_tokens[token_count].end();
            let span = span::Span {
                start,
                length: end - start,
                text: &input[start..end],
            };
            if let Some(token) = tokens::Token::try_from_str(span) {
                raw_tokens.drain(0..token_count + 1);
                result.push(token);
                continue 'outer;
            }
        }
        /* Failed to parse at all, stop the loop */
        break;
    }

    if raw_tokens.is_empty() {
        Ok(result)
    } else {
        let start = raw_tokens[0].start();
        let end = raw_tokens[raw_tokens.len() - 1].end();
        Err(error::LexerError::NoTokenMatch(
            input[start..end].to_string(),
        ))
    }
}
