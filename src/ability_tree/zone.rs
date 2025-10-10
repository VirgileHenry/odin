#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Zone {
    Battlefield,
    Exile,
    Graveyard,
    Hand,
    Library,
}

impl std::fmt::Display for Zone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Zone::Battlefield => write!(f, "battlefield"),
            Zone::Exile => write!(f, "exile"),
            Zone::Graveyard => write!(f, "graveyard"),
            Zone::Hand => write!(f, "hand"),
            Zone::Library => write!(f, "library"),
        }
    }
}

impl crate::ability_tree::terminals::Terminal for Zone {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "battlefield" => Some(Zone::Battlefield),
            "exile" => Some(Zone::Exile),
            "graveyard" => Some(Zone::Graveyard),
            "hand" => Some(Zone::Hand),
            "library" => Some(Zone::Library),
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
