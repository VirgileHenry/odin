pub mod error;
pub mod span;
pub mod tokens;

const DELIMITERS: [char; 2] = ['(', ')'];
const DISCARDABLE_PREFIX: [char; 1] = [' '];

/// Create a vec of Terminals from a string. Can fail, and will return an error.
pub fn lex(input: &str) -> Result<Vec<Terminal>, LexerError> {
    let mut lexer = OdinLexer::new(input);
    lexer.lex()
}

/// Internal Odin lexer.
struct OdinLexer {
    /// Reversed input regarding what we got, to push / pop it's end.
    input: String,
}

impl OdinLexer {
    /// Creates a new Odin lexer.
    pub fn new(input: &str) -> OdinLexer {
        OdinLexer {
            input: input.chars().rev().collect(),
        }
    }

    /// Create an array of tokens from the input, leaving the input empty.
    pub fn lex(&mut self) -> Result<Vec<Terminal>, LexerError> {
        let mut result = Vec::new();

        // while there is remaining input, parse it up
        while !self.input.is_empty() {
            match self.parse_token()? {
                Some(token) => result.push(token),
                None => { /* keep parsing */ }
            }
        }

        Ok(result)
    }

    /// Create a token from the input, consuming only the characters used.
    fn parse_token(&mut self) -> Result<Option<Terminal>, LexerError> {
        // discard garbage at the front
        while self.input.ends_with(&DISCARDABLE_PREFIX) {
            self.input.pop();
        }

        // if we encounter a comment, remove it
        if self
            .input
            .ends_with(&CommentBlockDelimiter::OPENING_DELIMITERS)
        {
            let first_char = self
                .input
                .pop()
                .expect("No first char, but input comment delimiter detected");
            let opening_delimiter = CommentBlockDelimiter::opening_from_char(first_char)
                .expect("Comment block detected, but failed to parse first char");
            self.remove_comment_block(opening_delimiter)?;
            // discard garbage at the front, after comment
            while self.input.ends_with(&DISCARDABLE_PREFIX) {
                self.input.pop();
            }
        }

        match self.input.pop() {
            Some(digit_char @ '0'..='9') => self
                .parse_number(digit_char)
                .map(|n| Some(Terminal::Number(Number::Number(n)))),
            Some('{') => {
                let cost = self.parse_braces_content()?.to_ascii_uppercase();
                if cost == "T" {
                    Ok(Some(Terminal::TapCost))
                } else {
                    match Mana::from_str(&cost) {
                        Ok(cost) => Ok(Some(Terminal::Mana(cost))),
                        Err(_) => Err(LexerError::InvalidBraceCost(cost)),
                    }
                }
            }
            Some(other) => {
                let mut buffer = String::from(other);
                loop {
                    match Terminal::from_str(&buffer) {
                        Ok(token) => break Ok(Some(token)),
                        Err(_) => match self.input.pop() {
                            None => break Err(LexerError::NoTokenMatch(buffer)),
                            Some(del) if DELIMITERS.contains(&del) => {
                                break Err(LexerError::NoTokenMatch(buffer))
                            }
                            Some(other) => buffer.push(other),
                        },
                    }
                }
            }
            None => Ok(None), // empty input
        }
    }

    fn parse_number(&mut self, first_char: char) -> Result<i32, LexerError> {
        let mut buffer = String::from(first_char);

        loop {
            match self.input.pop() {
                Some(digit @ '0'..='9') => buffer.push(digit),
                Some(other) => {
                    self.input.push(other);
                    break;
                }
                None => break,
            }
        }

        buffer
            .parse()
            .map_err(|_| LexerError::InvalidNumeric(buffer))
    }

    fn parse_braces_content(&mut self) -> Result<String, LexerError> {
        let mut result = "".to_string();
        // Pop chars until we pop the end of braces.
        loop {
            match self.input.pop() {
                Some('}') => break Ok(result),
                Some('{') => {
                    // bit of a weird case, as we found a brace inside one another. Parse it and add it to the result
                    result.push('{');
                    result.push_str(&self.parse_braces_content()?);
                    result.push('}');
                }
                Some(other) => result.push(other),
                None => break Err(LexerError::UnclosedCost),
            }
        }
    }

    /// Remove a comment block from the input.
    ///
    /// This assumes we entered a comment block, and requires the entry delimiter to find the next matching closing delimiter.
    fn remove_comment_block(
        &mut self,
        opening_delimiter: CommentBlockDelimiter,
    ) -> Result<(), LexerError> {
        // pop chars until we pop the end of our comment entry.
        // if we find another comment entry, call recursively.
        loop {
            let c = self
                .input
                .pop()
                .ok_or(LexerError::UnclosedCommentBlock { opening_delimiter })?;
            if opening_delimiter.closing_from_char(c) {
                break Ok(()); // all good, we finisehd
            }
            if let Some(other_opening_delimiter) = CommentBlockDelimiter::opening_from_char(c) {
                self.remove_comment_block(other_opening_delimiter)?;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentBlockDelimiter {
    Parenthesis,
}

impl CommentBlockDelimiter {
    const OPENING_DELIMITERS: [char; 1] = ['('];

    fn opening_from_char(c: char) -> Option<CommentBlockDelimiter> {
        match c {
            '(' => Some(CommentBlockDelimiter::Parenthesis),
            _ => None,
        }
    }

    fn closing_from_char(&self, c: char) -> bool {
        match self {
            CommentBlockDelimiter::Parenthesis => c == ')',
        }
    }
}

impl std::fmt::Display for CommentBlockDelimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommentBlockDelimiter::Parenthesis => write!(f, "(")?,
        }
        Ok(())
    }
}
