pub mod ability;
pub mod imperative;
pub mod object;
pub mod statement;
pub mod terminals;
pub mod zone;

/// One or more abilities.
/// This is the root of the Magic: the Gathering texts,
/// and can represent on its own the full text box of a card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityTree {
    pub abilities: Vec<ability::Ability>,
}

impl AbilityTree {
    pub fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        writeln!(out, "Abilities:")?;

        Ok(())
    }
}

pub trait AbilityTreeImpl {
    fn display<W: std::io::Write>(&self, out: &mut W, padding: usize) -> std::io::Result<()>;
}
