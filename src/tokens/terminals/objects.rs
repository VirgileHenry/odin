use strum_macros::{EnumString, EnumIter};

use crate::ability_display::AbilityDisplay;

/// Terminal keywords that represent object types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum Object {
    Creature,
    Card,
    Permanent,
}

impl AbilityDisplay for Object {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Object::Creature => write!(f, "Creature"),
            Object::Card => write!(f, "Card"),
            Object::Permanent => write!(f, "Permanent"),
        }
    }
}