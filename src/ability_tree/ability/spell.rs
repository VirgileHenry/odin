#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellAbility {
    effect: crate::ability_tree::statement::Statement,
}
