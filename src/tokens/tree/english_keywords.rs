use crate::tokens::terminals::{trigger_condition::TriggerConditionKW, imperatives::ImperativeKW};

/// All the english keywords found on mtg cards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnglishKeywords {
    When,
    On,
    Put,
}

impl From<TriggerConditionKW> for EnglishKeywords {
    fn from(value: TriggerConditionKW) -> Self {
        match value {
            TriggerConditionKW::When => EnglishKeywords::When,
        }
    }
}

impl From<ImperativeKW> for EnglishKeywords {
    fn from(value: ImperativeKW) -> Self {
        match value {
            ImperativeKW::On => EnglishKeywords::On,
            ImperativeKW::Put => EnglishKeywords::Put,
        }
    }
}