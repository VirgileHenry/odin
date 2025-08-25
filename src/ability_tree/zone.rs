#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Zone {
    Graveyard,
    Battlefield,
    Library,
    Hand,
    Exile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZoneReference {
    TheBattlefield,
    OwnedZone(Zone, crate::ability_tree::terminals::AppartenanceSpecifier),
}
