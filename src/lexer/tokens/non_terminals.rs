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
    And,
    Another,
    Be,
    Cant,
    For,
    From,
    Have,
    Into,
    It,
    May,
    No,
    Of,
    On,
    Or,
    The,
    Top,
    Until,
    Where,
    With,
}

impl EnglishKeywords {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "and" => Some(EnglishKeywords::And),
            "another" => Some(EnglishKeywords::Another),
            "be" => Some(EnglishKeywords::Be),
            "can't" => Some(EnglishKeywords::Cant),
            "from" => Some(EnglishKeywords::From),
            "for" => Some(EnglishKeywords::For),
            "have" => Some(EnglishKeywords::Have),
            "into" => Some(EnglishKeywords::Into),
            "it" => Some(EnglishKeywords::It),
            "may" => Some(EnglishKeywords::May),
            "no" => Some(EnglishKeywords::No),
            "of" => Some(EnglishKeywords::Of),
            "or" => Some(EnglishKeywords::Or),
            "on" => Some(EnglishKeywords::On),
            "the" => Some(EnglishKeywords::The),
            "top" => Some(EnglishKeywords::Top),
            "until" => Some(EnglishKeywords::Until),
            "where" => Some(EnglishKeywords::Where),
            "with" => Some(EnglishKeywords::With),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKeywords {
    Deals,
    Gain,
    Get,
    Put,
    Reveal,
}

impl ActionKeywords {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "deal" | "deals" => Some(ActionKeywords::Deals),
            "gain" | "gains" => Some(ActionKeywords::Gain),
            "get" | "gets" => Some(ActionKeywords::Gain),
            "put" | "puts" => Some(ActionKeywords::Gain),
            "reveal" | "reveals" => Some(ActionKeywords::Gain),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VhyToSortLater {
    Damage,
    Life,
    HandSize,
    MaximumHandSize,
}

impl VhyToSortLater {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "life" => Some(VhyToSortLater::Life),
            "damage" | "damages" => Some(VhyToSortLater::Damage),
            "hand size" => Some(VhyToSortLater::HandSize),
            "maximum hand size" => Some(VhyToSortLater::HandSize),
            _ => None,
        }
    }
}
