#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Zone {
    Graveyard,
    Library,
    Hand,
    Exile,
}

impl std::fmt::Display for Zone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Zone::Graveyard => write!(f, "Graveyard"),
            Zone::Library => write!(f, "Library"),
            Zone::Hand => write!(f, "Hand"),
            Zone::Exile => write!(f, "Exile"),
        }
    }
}

impl crate::ability_tree::terminals::Terminal for Zone {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "graveyard" => Some(Zone::Graveyard),
            "library" => Some(Zone::Library),
            "hand" => Some(Zone::Hand),
            "exile" => Some(Zone::Exile),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZoneReference {
    TheBattlefield,
    OwnedZone(Zone, crate::ability_tree::terminals::Appartenance),
}

impl crate::ability_tree::AbilityTreeImpl for ZoneReference {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ZoneReference::TheBattlefield => write!(out, "The Battlefield"),
            ZoneReference::OwnedZone(zone, appartenance) => {
                write!(out, "{appartenance} {zone}")
            }
        }
    }
}
