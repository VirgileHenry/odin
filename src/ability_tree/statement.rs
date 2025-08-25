#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Imperative(crate::ability_tree::imperative::Imperative),
    May(crate::ability_tree::imperative::Imperative),
}
