use std::fmt;
use mtg_data::AbilityKeyword;
use crate::ability_display::AbilityDisplay;

use self::{
    activated_ab::ActivatedAbility,
    spell_ab::SpellAbility,
    static_ab::StaticAbility,
    triggered_ab::TriggeredAbility
};

/// Any kind of Mtg ability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ability {
    Activated(ActivatedAbility),
    Spell(SpellAbility),
    Static(StaticAbility),
    Triggered(TriggeredAbility)
}

impl AbilityDisplay for Ability {
    fn display(&self, f: &mut fmt::Formatter<'_>, padding: &mut Vec<bool>) -> fmt::Result {
        match &self {
            Ability::Activated(ab) => ab.display(f, padding)?,
            Ability::Spell(ab) => ab.display(f, padding)?,
            Ability::Static(ab) => ab.display(f, padding)?,
            Ability::Triggered(ab) => ab.display(f, padding)?,
        }
        Ok(())
    }
}

impl From<AbilityKeyword> for Ability {
    fn from(value: AbilityKeyword) -> Self {
        // got to match the type of ability it is 
        // most of them or static, some triggered
        match value {
            AbilityKeyword::Flying => Ability::Static(StaticAbility::CommonStaticAbility(value)),
            AbilityKeyword::Mentor => Ability::Triggered(TriggeredAbility::CommonTriggeredAbilities(value)),
            AbilityKeyword::Rebound => Ability::Spell(SpellAbility::CommonSpellAbility(AbilityKeyword::Rebound)),
        }
    }
}

pub mod activated_ab;
pub mod spell_ab;
pub mod static_ab;
pub mod triggered_ab;