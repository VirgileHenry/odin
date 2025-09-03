/*
ODIN

The Odin library can parse Magic: The gathering abilities text into ability trees.
To do so, the elements AbilityTree implements TryFromStr, which calls the lexer and parser.
ALternatively, there is the lex and parse functions that allows lexing and parsing.
*/

pub mod ability_display;
pub mod ability_tree;
pub mod error;
pub mod lexer;
pub mod utils;
