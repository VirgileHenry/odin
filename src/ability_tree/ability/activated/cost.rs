#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cost {
    ManaCost(Vec<mtg_data::Mana>),
    Imperative(crate::ability_tree::imperative::Imperative),
}

impl crate::ability_tree::AbilityTreeImpl for Cost {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Cost::ManaCost(mana_cost) => {
                write!(out, "ManaCost:")?;
                for cost in mana_cost.iter().take(mana_cost.len().saturating_sub(1)) {
                    write!(out, "{cost}, ")?;
                }
                if let Some(cost) = mana_cost.last() {
                    write!(out, "{cost}, ")?;
                }
            }
            Cost::Imperative(cost) => {
                write!(out, "Imperative:")?;
                out.push_final_branch()?;
                cost.display(out)?;
                out.pop_branch();
            }
        }
        Ok(())
    }
}
