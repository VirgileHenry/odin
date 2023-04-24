use std::{collections::VecDeque, str::FromStr};

use crate::tokens::terminals::Terminal;

use self::error::OdinLexerError;

/// Create a vec of Terminals from a string. Can fail, and will return an error.
pub fn lex(input: &str) -> Result<Vec<Terminal>, OdinLexerError> {
    let mut lexer = OdinLexer::new(input);
    lexer.lex()
}

/// Internal Odin lexer. 
struct OdinLexer {
    input: VecDeque<char>,
    current_word: Vec<char>,
}

impl OdinLexer {
    /// Creates a new Odin lexer.
    pub fn new(input: &str) -> OdinLexer {
        OdinLexer { 
            input: input.chars().collect(),
            current_word: Vec::new(),
        }
    }

    /// Create an array of tokens from the input, leaving the input empty.
    pub fn lex(&mut self) -> Result<Vec<Terminal>, OdinLexerError> {
        let mut result = Vec::new();

        // while there is remaining input, parse it up
        while !self.input.is_empty() {
            match self.parse_token() {
                Ok(option) => match option {
                    Some(token) => result.push(token),
                    None => {/* keep parsing */}
                }
                Err(_) => {
                    // we failed finding a token from our position.
                    // discard for now, but catch later on.
                    // todo : handle this error
                }
            } 
        }

        Ok(result)
    }

    /// Create a token from the input, consuming only the characters used.
    /// When called, this function assumes current_word is empty.
    fn parse_token(&mut self) -> Result<Option<Terminal>, OdinLexerError> {
        // get first char
        let mut c = self.input.pop_front().unwrap(); // unwrap bc if this is called, input is non empty
        // first, try to match single letter possibilities.
        // check flow control
        if c == ' ' { return Ok(None); } // space is a custom char, spacing tokens. If first char, remove it.

        // check comment blocks (explainations)
        match CommentBlockDefiner::try_read(c) {
            Some(comment_entry) => {
                self.remove_comment_block(comment_entry)?;
                return Ok(None);
            },
            None => {},
        }
        // no one char match : start word loop 
        
        loop {
            self.current_word.push(c);
            let word: String = self.current_word.iter().collect();
            let word = word.to_ascii_lowercase();
            match Terminal::from_str(&word) {
                Ok(token) => {
                    self.clear_word();
                    return Ok(Some(token))
                },
                Err(_) => {
                    // add character to it !
                    c = match self.input.pop_front() {
                        Some(c) => c,
                        None => return Err(OdinLexerError::NoCharsUntilEnd),
                    };
                },
            }
        }
    }

    /// Remove a comment block from the input, considering we are in one.
    fn remove_comment_block(&mut self, comment_entry: CommentBlockDefiner) -> Result<(), OdinLexerError> {
        // pop chars until we pop the end of our comment entry.
        // if we find another comment entry, call recursively.
        loop {
            let c = match self.input.pop_front() {
                Some(c) => c,
                None => return Err(OdinLexerError::CommentBlockNeverClose),
            };
            if comment_entry.is_closing(c) {
                break;
            }
            match CommentBlockDefiner::try_read(c) {
                Some(other_entry) => self.remove_comment_block(other_entry)?,
                None => {},
            }
        }

        Ok(())
    }

    /// Clear the current stored word.
    fn clear_word(&mut self) {
        self.current_word.clear();
    }
}

enum CommentBlockDefiner {
    Parenthesis,
}

impl CommentBlockDefiner {
    pub fn try_read(c: char) -> Option<CommentBlockDefiner> {
        match c {
            '(' => Some(CommentBlockDefiner::Parenthesis),
            _ => None,
        }
    }

    pub fn is_closing(&self, c: char) -> bool {
        match self {
            CommentBlockDefiner::Parenthesis => c == ')',
        }
    }
}

pub mod error;