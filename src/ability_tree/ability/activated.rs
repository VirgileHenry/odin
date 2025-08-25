mod cost;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedAbility {
    cost: Vec<cost::Cost>,
    effect: crate::ability_tree::statement::Statement,
}
