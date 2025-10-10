mod layout;

/// A card in the game sense: any hing that can be in a deck and played.
pub struct Card {
    pub name: crate::card::CardName,
    pub scryfall_id: uuid::Uuid,
    pub legalities: crate::card::legalities::Legalities,
    pub color_identity: crate::card::ColorIdentity,
    pub layout: layout::Layout,
}

pub enum CardType {
    Artifact {
        subtypes: arrayvec::ArrayVec<mtg_data::ArtifactType, 8>,
    },
    Battle {
        subtypes: arrayvec::ArrayVec<mtg_data::BattleType, 8>,
    },
    Creature {
        subtypes: arrayvec::ArrayVec<mtg_data::CreatureType, 8>,
        power: u32,
        toughness: u32,
    },
    Dungeon,
    Emblem,
    Enchantment {
        subtypes: arrayvec::ArrayVec<mtg_data::EnchantmentType, 8>,
    },
    Instant {
        subtypes: arrayvec::ArrayVec<mtg_data::SpellType, 8>,
    },
    Land {
        subtypes: arrayvec::ArrayVec<mtg_data::LandType, 8>,
    },
    Planeswalker {
        subtypes: arrayvec::ArrayVec<mtg_data::PlaneswalkerType, 8>,
        start_loyaulty: u32,
    },
    Sorcery {
        subtypes: arrayvec::ArrayVec<mtg_data::SpellType, 8>,
    },
}
