use std::{collections::VecDeque, str::FromStr};

use mtg_data::ManaCost;

use crate::tokens::terminals::{Terminal, numbers::Number};

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
            match self.parse_token()? {
                Some(token) => result.push(token),
                None => {/* keep parsing */},
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

        if c.is_digit(10) {
            let num = self.try_number(c);
            return Ok(Some(Terminal::Number(Number::Number(num))));
        }

        if c == '{' {
            // let's read a cost thingy
            let cost = self.get_braces_symbol()?.to_ascii_uppercase();
            // check for tap costs
            if cost == "T" {
                return Ok(Some(Terminal::TapCost));
            }
            return match ManaCost::from_str(&cost) {
                Ok(cost) => Ok(Some(Terminal::ManaCost(cost))),
                Err(_) => Err(OdinLexerError::InvalidBraceCost(cost)),
            }
        }

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
                        None => return Err(OdinLexerError::NoTokenMatch(self.current_word.iter().collect())),
                    };
                },
            }
        }
    }

    fn try_number(&mut self, c: char) -> u32 {
        // while there are numbers, keep parsing. When not anymore, return value.
        let mut result = c.to_digit(10).unwrap(); // we can unwrap safely as we called this function knowing c is a digit.

        loop {
            let c = match self.input.pop_front() {
                Some(c) => c,
                None => return result, // no more chars, return the result.
            };
            if c.is_digit(10) {
                result = result * 10 + c.to_digit(10).unwrap();
            }
            else {
                // put the char back and return result
                self.input.push_front(c);
                return result;
            }
        }
    }

    fn get_braces_symbol(&mut self) -> Result<String, OdinLexerError> {
        let mut result = "".to_string();
        // pop chars until we pop the end of braces.
        // assume we do not find another brace, or this could break ?
        loop {
            let c = match self.input.pop_front() {
                Some(c) => c,
                None => return Err(OdinLexerError::CommentBlockNeverClose),
            };
            if c == '}' {
                return Ok(result);
            }
            else {
                result.push(c);
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