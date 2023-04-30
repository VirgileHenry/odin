use strum_macros::{EnumString, EnumIter};

use crate::ability_display::AbilityDisplay;


/// Terminal keywords that gives a control specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum AppartenanceSpecifier {
    Your,
}

impl AbilityDisplay for AppartenanceSpecifier {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            AppartenanceSpecifier::Your => write!(f, "Your"),
        }
    }
}