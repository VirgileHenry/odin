#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cost {
    ManaCost(mtg_data::Mana),
    Imperative(crate::ability_tree::imperative::Imperative),
}
