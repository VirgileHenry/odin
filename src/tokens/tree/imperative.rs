use crate::{ability_display::AbilityDisplay, ability_display_elems};

use self::put::Put;

/// An imperative action that a player must take when called.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Imperative {
    /// The "put" imperative action.
    Put(Put),
}

pub mod put;

impl AbilityDisplay for Imperative {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Imperative::Put(put) => {
                write!(f, "Imperative:\n")?;
                ability_display_elems!(f; padding; put);
            }
        }
        Ok(())
    }
}