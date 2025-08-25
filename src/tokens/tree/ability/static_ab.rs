use std::fmt;

use mtg_data::KeywordAbility;

use crate::ability_display::AbilityDisplay;

/// Represent a Static Ability.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StaticAbility {
    /// A common static ability, represented by a single keyword. (e.g.: "flying") 
    CommonStaticAbility(KeywordAbility),
}

impl AbilityDisplay for StaticAbility {
    fn display(&self, f: &mut fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> fmt::Result {
        match &self {
            StaticAbility::CommonStaticAbility(ab) => write!(f, "Keyword (Static Ability) : {:?}", ab)?,
        }
        Ok(())

    }
}