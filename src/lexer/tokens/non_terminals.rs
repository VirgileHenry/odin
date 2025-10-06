#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlowToken {
    NewLine,
    Comma,
    Dot,
    Colons,
}

impl ControlFlowToken {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "\n" => Some(ControlFlowToken::NewLine),
            "," => Some(ControlFlowToken::Comma),
            "." => Some(ControlFlowToken::Dot),
            ":" => Some(ControlFlowToken::Colons),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerAbilityMarker {
    Whenever,
    When,
    At,
}

impl TriggerAbilityMarker {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "whenever" => Some(TriggerAbilityMarker::Whenever),
            "when" => Some(TriggerAbilityMarker::When),
            "at" => Some(TriggerAbilityMarker::At),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapUntapCost {
    Tap,
    Untap,
}

impl TapUntapCost {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "{t}" => Some(TapUntapCost::Tap),
            "{q}" => Some(TapUntapCost::Untap),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnglishKeywords {
    Be,
    Cant,
    From,
    Have,
    No,
    The,
    Where,
}

impl EnglishKeywords {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "be" => Some(EnglishKeywords::Be),
            "can't" => Some(EnglishKeywords::Cant),
            "from" => Some(EnglishKeywords::From),
            "have" => Some(EnglishKeywords::Have),
            "no" => Some(EnglishKeywords::No),
            "the" => Some(EnglishKeywords::The),
            "where" => Some(EnglishKeywords::Where),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfReferencing;

impl SelfReferencing {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "this" => Some(SelfReferencing),
            "~" => Some(SelfReferencing),
            _ => None,
        }
    }
}
