mod ability;
mod imperative;
mod object;
mod statement;
mod terminals;
mod zone;

/// One or more abilities.
/// This is the root of the Magic: the Gathering texts,
///and can represent on its own the full text box of a card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityTree {
    pub abilities: Vec<ability::Ability>,
}
