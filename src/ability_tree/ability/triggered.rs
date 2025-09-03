pub mod trigger_cond;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggeredAbility {
    pub condition: trigger_cond::TriggerCondition,
    pub effect: crate::ability_tree::statement::Statement,
}

impl crate::ability_tree::AbilityTreeImpl for TriggeredAbility {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Triggered Ability:")?;
        out.push_inter_branch()?;
        write!(out, "Condition:")?;
        out.push_final_branch()?;
        self.condition.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "Effect:")?;
        out.push_final_branch()?;
        self.effect.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
