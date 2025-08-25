use std::fmt;

use mtg_data::KeywordAbility;
use crate::{ability_display::AbilityDisplay, tokens::tree::{trigger_condition::TriggerCondition, statement::Statement}, ability_display_elems};

/// Represent a Triggered Ability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggeredAbility {
    /// A common triggerd ability, represented by a single keyword. (e.g.: "mentor") 
    CommonTriggeredAbilities(KeywordAbility),
    TriggerAbility(TriggerCondition, Statement),
}

impl AbilityDisplay for TriggeredAbility {
    fn display(&self, f: &mut fmt::Formatter<'_>, padding: &mut Vec<bool>) -> fmt::Result {
        match &self {
            TriggeredAbility::CommonTriggeredAbilities(ab) => {
                write!(f, "Keyword (Triggered Ability) : {:?}", ab)?;
            },
            TriggeredAbility::TriggerAbility(cond, statement) => {
                write!(f, "Triggered Ability:\n")?;
                ability_display_elems!(f; padding; cond, statement);
            },
        }
        Ok(())

    }
}

// Triggered Ab
//      