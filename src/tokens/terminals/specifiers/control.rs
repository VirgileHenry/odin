use strum_macros::{EnumString, EnumIter};

use crate::ability_display::AbilityDisplay;

/// Terminal keywords that gives a control specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum ControlSpecifier {
    #[strum(serialize = "you control")]
    YouControl,
    #[strum(serialize = "you don't control")]
    YouDontControl,
}

impl AbilityDisplay for ControlSpecifier {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            ControlSpecifier::YouControl => write!(f, "You Control"),
            ControlSpecifier::YouDontControl => write!(f, "You don't Control"),
        }
    }
}