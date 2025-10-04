#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerCondition {
    ObjectDoesAction {
        object: crate::ability_tree::object::ObjectReference,
        action: crate::ability_tree::terminals::CardActions,
    },
}

impl crate::ability_tree::AbilityTreeImpl for TriggerCondition {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            TriggerCondition::ObjectDoesAction { object, action } => {
                write!(out, "When:")?;
                out.push_inter_branch()?;
                write!(out, "Object:")?;
                out.push_final_branch()?;
                object.display(out)?;
                out.pop_branch();
                out.next_final_branch()?;
                write!(out, "Does Action:")?;
                out.push_final_branch()?;
                write!(out, "{action}")?;
                out.pop_branch();
                out.pop_branch();
                Ok(())
            }
        }
    }
}
