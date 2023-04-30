use strum_macros::{EnumString, EnumIter};

use crate::ability_display::AbilityDisplay;

/// Terminal word that represent the trigger ability markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum TriggerConditionKW {
    When,
}

impl AbilityDisplay for TriggerConditionKW {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}