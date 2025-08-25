mod trigger_cond;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggeredAbility {
    condition: trigger_cond::TriggerCondition,
    effect: crate::ability_tree::statement::Statement,
}
