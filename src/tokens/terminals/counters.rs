use strum_macros::{EnumString, EnumIter};

use crate::ability_display::AbilityDisplay;

/// Terminal keywords that represents counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Counter {
    #[strum(serialize = "+1/+1 counter")]
    PlusOne,
}

impl AbilityDisplay for Counter {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Counter::PlusOne => write!(f, "+1/+1 Counter"),
        }
    }
}