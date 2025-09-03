mod cost;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedAbility {
    costs: Vec<cost::Cost>,
    effect: crate::ability_tree::statement::Statement,
}

impl crate::ability_tree::AbilityTreeImpl for ActivatedAbility {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Activated:")?;
        out.push_inter_branch()?;
        write!(out, "Costs:")?;
        out.push_final_branch()?;
        for cost in self.costs.iter() {
            // fixme: also in a tree for a cost
            cost.display(out)?;
            write!(out, ", ")?;
        }
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
