pub mod ability_display;
pub mod ability_tree;
pub mod error;
pub mod lexer;
pub mod utils;

fn main() {
    let tree = ability_tree::example();

    let mut stdout = std::io::stdout().lock();
    tree.display(&mut stdout).unwrap();
}
