// Quick example to demonstrate the Odin lexer and parser.
// this examples show the parsing of the card "Light of the Legion",
// which was the first card this parser was built to read.

use std::str::FromStr;

use odin::tokens::tree::AbilityTree;

fn main() {
    println!("{}", AbilityTree::from_str("Flying\nMentor (Whenever this creature attacks, put a +1/+1 counter on target attacking creature with lesser power.)\nWhen ~ dies, put a +1/+1 counter on each white creature you control.$").unwrap());
}