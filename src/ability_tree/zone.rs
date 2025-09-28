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

impl crate::ability_tree::terminals::KeywordTerminal for Zone {
    fn repr(&self) -> &'static str {
        match self {
            Zone::Graveyard => "graveyard",
            Zone::Library => "library",
            Zone::Hand => "hand",
            Zone::Exile => "exile",
        }
    }
    fn iter() -> impl Iterator<Item = Self> {
        [Zone::Graveyard, Zone::Library, Zone::Hand, Zone::Exile].into_iter()
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
