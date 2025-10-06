pub mod ability_tree;
pub mod error;
pub mod lexer;
pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tree = ability_tree::example();

    let mut stdout = std::io::stdout().lock();
    tree.display(&mut stdout).unwrap();

    let krenko_name = "Krenko, Mob Boss";
    let krenko_text = "+2/+1";

    let preprocessed = lexer::preprocess(krenko_name, krenko_text);
    let tokens = lexer::lex(&preprocessed);

    println!("{tokens:?}");

    Ok(())
}
