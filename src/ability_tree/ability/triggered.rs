pub mod trigger_cond;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggeredAbility {
    pub condition: trigger_cond::TriggerCondition,
    pub effect: crate::ability_tree::statement::Statement,
}

impl crate::ability_tree::AbilityTreeImpl for TriggeredAbility {
    fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        writeln!(out, "Condition:")?;
        self.condition.display(out)?;
        writeln!(out, "Effect:")?;
        self.effect.display(out)?;
        Ok(())
    }
}
