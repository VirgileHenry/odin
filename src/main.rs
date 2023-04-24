use std::env;

pub mod ability_display;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod tokens;


use errors::OdinErrors;
use lexer::*;
use parser::parse;




fn main() -> Result<(), OdinErrors> {

    let mut args = env::args();
    // discard first arg (prog launch)
    let _ = args.next();

    // quick test, lesgooo
    let test_input = "Flying\nMentor (Whenever this creature attacks, put a +1/+1 counter on target attacking creature with lesser power.)\nWhen ~ dies, put a +1/+1 counter on each white creature you control.$";

    test(test_input);

    Ok(())
}


fn test(input: &str) {
    let tokens = lex(input).unwrap();
    for token in tokens.iter() {
        println!("{token}");
    }
    let tree = parse(tokens).unwrap();
    println!("{tree}");
}
