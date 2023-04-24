use std::fmt;

use crate::ability_display::AbilityDisplay;

/// Represent a Spell Ability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellAbility {
    
}

impl AbilityDisplay for SpellAbility {
    fn display(&self, f: &mut fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> fmt::Result {
        write!(f, "Spell Ability: Not Implemented.")
    }
}