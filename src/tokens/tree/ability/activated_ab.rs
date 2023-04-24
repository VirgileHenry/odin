use std::fmt;

use crate::ability_display::AbilityDisplay;

/// Represent an activated ability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivatedAbility {

}

impl AbilityDisplay for ActivatedAbility {
    fn display(&self, f: &mut fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> fmt::Result {
        write!(f, "Activated Ability: Not Implemented.")

    }
}