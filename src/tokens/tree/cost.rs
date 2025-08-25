use mtg_data::Mana;

use crate::{ability_display::AbilityDisplay, ability_display_elems};

use super::imperative::Imperative;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cost {
    ManaCost(Mana),
    Imperative(Imperative),
    TapCost,
}

impl AbilityDisplay for Cost {
    fn display(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        padding: &mut Vec<bool>,
    ) -> std::fmt::Result {
        match self {
            Cost::TapCost => write!(f, "Tap"),
            Cost::ManaCost(mc) => write!(f, "Mana cost: {mc}"),
            Cost::Imperative(imp) => {
                write!(f, "imperative cost:\n")?;
                ability_display_elems!(f; padding; imp);
                Ok(())
            }
        }
    }
}
