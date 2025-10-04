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
