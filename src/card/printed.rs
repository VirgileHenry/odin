mod layout;

/// A card in the printed sense, as "any piece of cardboard WotC have printed".
/// This includes common playable cards, but also art only cards, tokens, etc.
pub struct PrintedCard {
    pub scryfall_id: uuid::Uuid,
    pub name: crate::card::CardName,
    pub legalities: crate::card::legalities::Legalities,
    pub color_identity: crate::card::ColorIdentity,
    pub layout: layout::Layout,
}
