mod cost;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedAbility {
    costs: Vec<cost::Cost>,
    effect: crate::ability_tree::statement::Statement,
}

impl crate::ability_tree::AbilityTreeImpl for ActivatedAbility {
    fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        writeln!(out, "Costs:")?;
        for cost in self.costs.iter() {
            cost.display(out)?;
            writeln!(out, "")?;
        }
        writeln!(out, "Effect:")?;
        self.effect.display(out)?;
        Ok(())
    }
}
