use strum_macros::{EnumString, EnumIter};

use crate::ability_display::AbilityDisplay;


/// Terminal keywords that gives a control specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum Zone {
    Graveyard,
    #[strum(serialize = "the battlefield")] 
    Battlefield, // will always be mentioned as "the battlefield".
}

impl AbilityDisplay for Zone {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Zone::Battlefield => write!(f, "The Battlefield"),
            Zone::Graveyard => write!(f, "Graveyard"),
        }
    }
}