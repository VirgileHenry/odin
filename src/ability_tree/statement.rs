#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Imperative(crate::ability_tree::imperative::Imperative),
    May(crate::ability_tree::imperative::Imperative),
}

impl crate::ability_tree::AbilityTreeImpl for Statement {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Statement::Imperative(_) => write!(out, "Imperative:")?,
            Statement::May(_) => write!(out, "May Ability")?,
        }
        out.push_final_branch()?;
        match self {
            Statement::Imperative(imp) | Statement::May(imp) => imp.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}
