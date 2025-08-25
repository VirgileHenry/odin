use std::fmt;

use mtg_data::KeywordAbility;

use crate::{ability_display::AbilityDisplay, tokens::tree::statement::Statement, ability_display_elems};

/// Represent a Spell Ability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellAbility {
    CommonSpellAbility(KeywordAbility),
    Statement(Statement),
}

impl AbilityDisplay for SpellAbility {
    fn display(&self, f: &mut fmt::Formatter<'_>, padding: &mut Vec<bool>) -> fmt::Result {
        match self {
            SpellAbility::CommonSpellAbility(ab) => write!(f, "Keyword (Spell Ability) : {:?}", ab)?,
            SpellAbility::Statement(stmt) => {
                write!(f, "Spell ability:\n")?;
                ability_display_elems!(f; padding; stmt);
            }
        }
        Ok(())
    }
}