#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Imperative(crate::ability_tree::imperative::Imperative),
    May(crate::ability_tree::imperative::Imperative),
}

impl crate::ability_tree::AbilityTreeImpl for Statement {
    fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        match self {
            Statement::Imperative(imperative) => {
                writeln!(out, "Imperative:")?;
                imperative.display(out)
            }
            Statement::May(may) => {
                writeln!(out, "May Ability")?;
                may.display(out)
            }
        }
    }
}
