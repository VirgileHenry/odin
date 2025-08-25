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

    // here let's assert everything we do does not break previous tests ?
    use std::str::FromStr;
    println!("{}", odin::tokens::tree::AbilityTree::from_str("Flying\nMentor (Whenever this creature attacks, put a +1/+1 counter on target attacking creature with lesser power.)\nWhen ~ dies, put a +1/+1 counter on each white creature you control.$").expect("Light of legion now fails"));
    println!("{}", odin::tokens::tree::AbilityTree::from_str("~ deals 1 damage to each creature you don't control.$").expect("deal damages now fails"));
    println!("{}", odin::tokens::tree::AbilityTree::from_str("{4}, {T}, Sacrifice ~: ~ deals 3 damage to target creature.$").expect("deal damages now fails"));

    // quick test, lesgooo
    let test_input = "$";

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

