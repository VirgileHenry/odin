use std::collections::VecDeque;

use crate::tokens::{
    tree::tree_token::TreeNode
};

/// Errors that can be thrown by the parser.
#[derive(Debug, Clone)]
pub enum ParserError {
    /// The parser is stuck in the current branch, and is going to backtrack.
    /// If this is thrown by the parsing function, the parser was unable to parse the given token vec.
    Stuck{
        /// State of the stack when it was stuck.
        stack: Vec<TreeNode>,
        /// State of the input when it was stuck.
        input: VecDeque<TreeNode>,
    },
    /// The parser managed to parse the tokens into a single token,
    /// but the final token was not the root token.
    InvalidRoot,
}