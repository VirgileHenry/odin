use strum_macros::{EnumString, EnumIter};

use crate::ability_display::AbilityDisplay;

/// Terminal keyword that represent an action that can be done.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum Actions {
    Dies,
    Attacks,
}

impl AbilityDisplay for Actions {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Actions::Dies => write!(f, "Dies"),
            Actions::Attacks => write!(f, "Attacks"),
        }
    }
}