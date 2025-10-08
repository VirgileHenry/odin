mod layout;
mod legalities;

type CardName = arrayvec::ArrayString<64>;

pub struct ScryfallCard {
    /* === General features === */
    pub scryfall_id: uuid::Uuid,
    pub name: CardName,
    pub legalities: legalities::Legalities,
    /* === Gameplay related features === */
    pub color_identity: arrayvec::ArrayVec<mtg_data::Color, 5>,
    pub layout: layout::Layout,
}
