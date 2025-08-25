#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerCondition {
    ObjectDoesAction {
        object: crate::ability_tree::object::ObjectReference,
        action: crate::ability_tree::terminals::Actions,
    },
}
