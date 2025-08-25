use std::env;

pub mod ability_tree;
pub mod ability_display;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod tokens;


use errors::OdinErrors;


fn main() -> Result<(), OdinErrors> {

    let mut args = env::args();
    // discard first arg (prog launch)
    let _ = args.next();

    Ok(())
}

