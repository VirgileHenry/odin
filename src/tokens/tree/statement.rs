use crate::ability_display::AbilityDisplay;

use super::imperative::Imperative;

/// A Mtg statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// An Imperative statement.
    Imperative(Imperative)
}

impl AbilityDisplay for Statement {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Statement::Imperative(imp) => imp.display(f, padding),
        }
    }
}