#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerCondition {
    ObjectDoesAction {
        object: crate::ability_tree::object::ObjectReference,
        action: crate::ability_tree::terminals::Actions,
    },
}

impl crate::ability_tree::AbilityTreeImpl for TriggerCondition {
    fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        match self {
            TriggerCondition::ObjectDoesAction { object, action } => {
                writeln!(out, "When:")?;
                writeln!(out, "Object:")?;
                object.display(out)?;
                writeln!(out, "")?;
                writeln!(out, "Does: {action}")?;
                Ok(())
            }
        }
    }
}
