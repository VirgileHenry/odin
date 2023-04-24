use crate::{tokens::terminals::{trigger_condition::TriggerConditionKW, actions::Actions}, ability_display::AbilityDisplay, ability_display_elems};

use super::object_reference::ObjectReference;

/// A trigger ability condition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerCondition {
    /// When an object does something.
    ObjectDoesAction(TriggerConditionKW, ObjectReference, Actions),
}

impl AbilityDisplay for TriggerCondition {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            TriggerCondition::ObjectDoesAction(kw, obj, act) => {
                write!(f, "Condition: Object does Action\n")?;
                ability_display_elems!(f; padding; kw, obj, act);
            }
        }

        Ok(())
    }
}