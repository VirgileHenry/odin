pub mod ability_tree;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let krenko_name = "Krenko, Mob Boss";
    let krenko_text = "{T}: Create X 1/1 red Goblin creature tokens, where X is the number of Goblins you control.";

    let preprocessed = lexer::preprocess(krenko_name, krenko_text);
    println!("Preprocessed oracle text: {preprocessed}");

    let tokens = lexer::lex(&preprocessed)?;
    println!("tokens: [");
    tokens.iter().for_each(|token| println!("  {token:?}"));
    println!("]");

    let parsed = parser::parse(&tokens);

    Ok(())
}
