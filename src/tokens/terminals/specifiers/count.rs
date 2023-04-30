use strum_macros::{EnumString, EnumIter};

use crate::ability_display::AbilityDisplay;

/// Terminal keywords that gives a count specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum CountSpecifier {
    Each,
    Target,
    #[strum(serialize = "up to")]
    UpTo,
}

impl AbilityDisplay for CountSpecifier {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            CountSpecifier::Each => write!(f, "Each"),
            CountSpecifier::Target => write!(f, "Target"),
            CountSpecifier::UpTo => write!(f, "Up To"),
        }
    }
}