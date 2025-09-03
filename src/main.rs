pub mod ability_display;
pub mod ability_tree;
pub mod error;
pub mod lexer;

fn main() {
    let tree = ability_tree::example();
    let err = tree.display(&mut std::io::stdout().lock());

    if let Err(err) = err {
        eprintln!("Failed to display ability tree: {err}");
    }
}
